use super::{ClassificationResult, ModelInfo};
use std::path::Path;
use tracing::{info, warn};

/// Known signal modulation types for the IQ classifier
pub const MODULATION_LABELS: &[&str] = &[
    "OOK", "ASK", "FSK", "GFSK", "MSK",
    "BPSK", "QPSK", "8PSK",
    "4QAM", "16QAM", "64QAM",
    "OFDM", "LoRa",
    "AM", "FM", "NFM", "WFM",
    "DJI_OcuSync", "DJI_Lightbridge",
    "ExpressLRS", "TBS_Crossfire",
    "FHSS_Generic", "DSSS_Generic",
    "Noise",
];

/// Known drone ESC motor signature types
pub const ESC_LABELS: &[&str] = &[
    "BLHeli_S_24kHz",
    "BLHeli_S_48kHz",
    "BLHeli_32",
    "Industrial_8kHz",
    "HVAC_Motor",
    "Vehicle_Motor",
    "No_Drone",
];

/// Known device types for WiFi/BLE fingerprinting
pub const DEVICE_LABELS: &[&str] = &[
    "iPhone", "Android_Phone", "iPad", "Android_Tablet",
    "MacBook", "Windows_Laptop", "Chromebook",
    "Smart_TV", "Smart_Speaker", "IoT_Sensor",
    "Drone_Controller", "Security_Camera",
    "Router_AP", "Printer",
    "Unknown",
];

pub struct OnnxClassifier {
    model_name: String,
    labels: Vec<String>,
    #[cfg(feature = "ml")]
    session: Option<ort::Session>,
}

impl OnnxClassifier {
    pub fn load(model_path: &Path, labels: &[&str]) -> Result<Self, String> {
        let model_name = model_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        #[cfg(feature = "ml")]
        {
            match ort::Session::builder()
                .and_then(|b| b.with_intra_threads(1))
                .and_then(|b| b.commit_from_file(model_path))
            {
                Ok(session) => {
                    info!("Loaded ONNX model: {} from {}", model_name, model_path.display());
                    Ok(Self {
                        model_name,
                        labels: labels.iter().map(|s| s.to_string()).collect(),
                        session: Some(session),
                    })
                }
                Err(e) => {
                    warn!("Failed to load ONNX model {}: {}", model_path.display(), e);
                    Err(format!("ONNX load error: {}", e))
                }
            }
        }

        #[cfg(not(feature = "ml"))]
        {
            warn!("ML feature not enabled, model {} loaded as stub", model_name);
            Ok(Self {
                model_name,
                labels: labels.iter().map(|s| s.to_string()).collect(),
            })
        }
    }

    pub fn classify(&self, features: &[f32]) -> Result<ClassificationResult, String> {
        let start = std::time::Instant::now();

        #[cfg(feature = "ml")]
        {
            if let Some(ref session) = self.session {
                let input_shape = vec![1, features.len() as i64];
                let input = ndarray::Array::from_shape_vec(
                    ndarray::IxDyn(&[1, features.len()]),
                    features.to_vec(),
                ).map_err(|e| format!("Array shape error: {}", e))?;

                let outputs = session.run(
                    ort::inputs!["input" => input.view()]
                        .map_err(|e| format!("Input error: {}", e))?
                ).map_err(|e| format!("Inference error: {}", e))?;

                // Get output tensor and apply softmax
                if let Some((_, output)) = outputs.iter().next() {
                    let output_arr = output.try_extract_tensor::<f32>()
                        .map_err(|e| format!("Output extract error: {}", e))?;
                    let scores: Vec<f32> = output_arr.iter().copied().collect();
                    let scores = softmax(&scores);

                    let (best_idx, &best_score) = scores.iter().enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap_or((0, &0.0));

                    let label = self.labels.get(best_idx)
                        .cloned()
                        .unwrap_or_else(|| format!("class_{}", best_idx));

                    let all_scores: Vec<(String, f32)> = self.labels.iter()
                        .zip(scores.iter())
                        .map(|(l, &s)| (l.clone(), s))
                        .collect();

                    return Ok(ClassificationResult {
                        label,
                        confidence: best_score,
                        all_scores,
                        model_name: self.model_name.clone(),
                        inference_ms: start.elapsed().as_secs_f64() * 1000.0,
                    });
                }
            }
        }

        // Stub mode: return unknown with zero confidence
        Ok(ClassificationResult {
            label: "Unknown (no model loaded)".into(),
            confidence: 0.0,
            all_scores: vec![],
            model_name: self.model_name.clone(),
            inference_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }

    pub fn model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.model_name.clone(),
            version: "1.0".into(),
            input_size: 0,
            num_classes: self.labels.len(),
            quantized: self.model_name.contains("int8"),
            size_bytes: 0,
        }
    }
}

fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|x| (x - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.iter().map(|e| e / sum).collect()
}
