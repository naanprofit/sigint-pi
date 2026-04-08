use super::AnomalyResult;

/// Autoencoder-based anomaly detection on spectrum data.
/// Until a trained model is available, uses statistical baseline comparison.
pub struct SpectrumAnomalyDetector {
    baseline_mean: Vec<f64>,
    baseline_std: Vec<f64>,
    num_samples: u64,
    threshold_z: f64,
}

impl SpectrumAnomalyDetector {
    pub fn new(threshold_z: f64) -> Self {
        Self {
            baseline_mean: Vec::new(),
            baseline_std: Vec::new(),
            num_samples: 0,
            threshold_z,
        }
    }

    /// Update the baseline with a new spectrum observation.
    /// Uses Welford's online algorithm for running mean/variance.
    pub fn update_baseline(&mut self, spectrum: &[f32]) {
        self.num_samples += 1;
        let n = self.num_samples as f64;

        if self.baseline_mean.len() != spectrum.len() {
            self.baseline_mean = vec![0.0; spectrum.len()];
            self.baseline_std = vec![0.0; spectrum.len()];
        }

        for (i, &val) in spectrum.iter().enumerate() {
            let v = val as f64;
            let old_mean = self.baseline_mean[i];
            self.baseline_mean[i] += (v - old_mean) / n;
            self.baseline_std[i] += (v - old_mean) * (v - self.baseline_mean[i]);
        }
    }

    /// Check if a spectrum is anomalous compared to the learned baseline.
    pub fn check_anomaly(&self, spectrum: &[f32]) -> AnomalyResult {
        if self.num_samples < 10 || self.baseline_mean.len() != spectrum.len() {
            return AnomalyResult {
                is_anomaly: false,
                reconstruction_error: 0.0,
                threshold: self.threshold_z,
                z_score: 0.0,
            };
        }

        let n = self.num_samples as f64;
        let mut total_z = 0.0;
        let mut max_z: f64 = 0.0;
        let mut anomalous_bins = 0;

        for (i, &val) in spectrum.iter().enumerate() {
            let v = val as f64;
            let std = (self.baseline_std[i] / n).sqrt();
            if std > 1e-10 {
                let z = ((v - self.baseline_mean[i]) / std).abs();
                total_z += z;
                if z > max_z { max_z = z; }
                if z > self.threshold_z { anomalous_bins += 1; }
            }
        }

        let avg_z = total_z / spectrum.len() as f64;
        let anomaly_ratio = anomalous_bins as f64 / spectrum.len() as f64;

        // Consider anomalous if >5% of bins exceed threshold OR max z-score is extreme
        let is_anomaly = anomaly_ratio > 0.05 || max_z > self.threshold_z * 2.0;

        AnomalyResult {
            is_anomaly,
            reconstruction_error: avg_z,
            threshold: self.threshold_z,
            z_score: max_z,
        }
    }

    pub fn num_baseline_samples(&self) -> u64 {
        self.num_samples
    }

    pub fn has_baseline(&self) -> bool {
        self.num_samples >= 10
    }
}
