use rustfft::{FftPlanner, num_complex::Complex};

/// Extract FFT magnitude spectrum from raw IQ samples (interleaved I,Q float32).
/// Returns magnitude vector of length window_size/2 (positive frequencies only).
pub fn iq_to_fft_magnitude(iq_samples: &[f32], window_size: usize) -> Vec<f32> {
    if iq_samples.len() < window_size * 2 {
        return vec![0.0; window_size / 2];
    }

    // Convert interleaved I,Q to Complex
    let mut buffer: Vec<Complex<f32>> = iq_samples.chunks_exact(2)
        .take(window_size)
        .map(|pair| Complex::new(pair[0], pair[1]))
        .collect();

    // Apply Hann window to reduce spectral leakage
    let n = buffer.len() as f32;
    for (i, sample) in buffer.iter_mut().enumerate() {
        let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / n).cos());
        *sample *= window;
    }

    // Forward FFT
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(buffer.len());
    fft.process(&mut buffer);

    // Magnitude spectrum (positive frequencies only)
    buffer.iter()
        .take(window_size / 2)
        .map(|c| c.norm())
        .collect()
}

/// Extract spectral features from FFT magnitude for ML classification.
/// Returns a fixed-size feature vector suitable for model input.
pub fn extract_spectral_features(magnitudes: &[f32]) -> SpectralFeatures {
    let n = magnitudes.len() as f32;
    if n == 0.0 {
        return SpectralFeatures::default();
    }

    let sum: f32 = magnitudes.iter().sum();
    let mean = sum / n;
    let max_val = magnitudes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_val = magnitudes.iter().cloned().fold(f32::INFINITY, f32::min);

    // Variance
    let variance: f32 = magnitudes.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
    let std_dev = variance.sqrt();

    // Spectral centroid (center of mass of the spectrum)
    let weighted_sum: f32 = magnitudes.iter().enumerate()
        .map(|(i, &m)| i as f32 * m)
        .sum();
    let centroid = if sum > 0.0 { weighted_sum / sum } else { 0.0 };

    // Spectral bandwidth (spread around centroid)
    let bandwidth: f32 = if sum > 0.0 {
        (magnitudes.iter().enumerate()
            .map(|(i, &m)| (i as f32 - centroid).powi(2) * m)
            .sum::<f32>() / sum).sqrt()
    } else { 0.0 };

    // Spectral flatness (geometric mean / arithmetic mean)
    // Indicates how noise-like vs tone-like the signal is
    let log_sum: f32 = magnitudes.iter()
        .map(|&m| if m > 1e-10 { m.ln() } else { -23.0 })
        .sum();
    let geometric_mean = (log_sum / n).exp();
    let flatness = if mean > 0.0 { geometric_mean / mean } else { 0.0 };

    // Peak-to-average ratio
    let peak_to_avg = if mean > 0.0 { max_val / mean } else { 0.0 };

    // Spectral rolloff (frequency below which 85% of energy is contained)
    let energy_threshold = sum * 0.85;
    let mut cumulative = 0.0;
    let mut rolloff_bin = 0;
    for (i, &m) in magnitudes.iter().enumerate() {
        cumulative += m;
        if cumulative >= energy_threshold {
            rolloff_bin = i;
            break;
        }
    }
    let rolloff = rolloff_bin as f32 / n;

    // Count peaks (local maxima above mean + 2*std_dev)
    let peak_threshold = mean + 2.0 * std_dev;
    let mut num_peaks = 0u32;
    for i in 1..magnitudes.len().saturating_sub(1) {
        if magnitudes[i] > peak_threshold
            && magnitudes[i] > magnitudes[i-1]
            && magnitudes[i] > magnitudes[i+1]
        {
            num_peaks += 1;
        }
    }

    SpectralFeatures {
        mean, std_dev, max_val, min_val,
        centroid, bandwidth, flatness,
        peak_to_avg, rolloff,
        num_peaks,
        total_energy: sum,
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SpectralFeatures {
    pub mean: f32,
    pub std_dev: f32,
    pub max_val: f32,
    pub min_val: f32,
    pub centroid: f32,
    pub bandwidth: f32,
    pub flatness: f32,
    pub peak_to_avg: f32,
    pub rolloff: f32,
    pub num_peaks: u32,
    pub total_energy: f32,
}

impl SpectralFeatures {
    /// Convert to fixed-size f32 vector for model input
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.mean, self.std_dev, self.max_val, self.min_val,
            self.centroid, self.bandwidth, self.flatness,
            self.peak_to_avg, self.rolloff,
            self.num_peaks as f32, self.total_energy,
        ]
    }
}

/// Detect harmonics in a magnitude spectrum.
/// Returns indices and magnitudes of detected harmonic series.
pub fn detect_harmonics(magnitudes: &[f32], min_harmonics: usize, tolerance_bins: usize) -> Vec<HarmonicSeries> {
    let n = magnitudes.len();
    if n < 10 { return vec![]; }

    let mean: f32 = magnitudes.iter().sum::<f32>() / n as f32;
    let std_dev: f32 = (magnitudes.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n as f32).sqrt();
    let threshold = mean + 3.0 * std_dev;

    // Find peaks
    let mut peaks: Vec<(usize, f32)> = Vec::new();
    for i in 1..n.saturating_sub(1) {
        if magnitudes[i] > threshold && magnitudes[i] > magnitudes[i-1] && magnitudes[i] > magnitudes[i+1] {
            peaks.push((i, magnitudes[i]));
        }
    }

    let mut series_list = Vec::new();

    // For each peak, try it as fundamental and look for harmonics
    for &(fund_bin, fund_mag) in &peaks {
        if fund_bin == 0 { continue; }
        let mut harmonics = vec![(fund_bin, fund_mag)];

        for h in 2..=10 {
            let expected_bin = fund_bin * h;
            if expected_bin >= n { break; }

            // Search within tolerance
            let search_start = expected_bin.saturating_sub(tolerance_bins);
            let search_end = (expected_bin + tolerance_bins).min(n - 1);
            let mut best_bin = 0;
            let mut best_mag: f32 = 0.0;
            for b in search_start..=search_end {
                if magnitudes[b] > best_mag {
                    best_mag = magnitudes[b];
                    best_bin = b;
                }
            }
            if best_mag > threshold {
                harmonics.push((best_bin, best_mag));
            }
        }

        if harmonics.len() >= min_harmonics {
            series_list.push(HarmonicSeries {
                fundamental_bin: fund_bin,
                harmonics,
            });
        }
    }

    series_list
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicSeries {
    pub fundamental_bin: usize,
    pub harmonics: Vec<(usize, f32)>,
}
