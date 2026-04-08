pub mod features;
pub mod classifier;
pub mod anomaly;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub input_size: usize,
    pub num_classes: usize,
    pub quantized: bool,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub label: String,
    pub confidence: f32,
    pub all_scores: Vec<(String, f32)>,
    pub model_name: String,
    pub inference_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub is_anomaly: bool,
    pub reconstruction_error: f64,
    pub threshold: f64,
    pub z_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlStatus {
    pub onnx_available: bool,
    pub models_loaded: Vec<ModelInfo>,
    pub models_dir: String,
    pub fft_available: bool,
}

pub fn get_ml_status() -> MlStatus {
    let models_dir = get_models_dir();
    let models_loaded = list_available_models(&models_dir);

    MlStatus {
        onnx_available: cfg!(feature = "ml"),
        models_loaded,
        models_dir: models_dir.to_string_lossy().to_string(),
        fft_available: true,
    }
}

fn get_models_dir() -> std::path::PathBuf {
    for dir in ["/home/sigint/ml/models", "/home/pi/sigint-pi/ml/models", "/home/deck/sigint-deck/ml/models"] {
        let p = std::path::PathBuf::from(dir);
        if p.parent().map(|p| p.exists()).unwrap_or(false) {
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
    }
    let fallback = std::path::PathBuf::from("./ml/models");
    let _ = std::fs::create_dir_all(&fallback);
    fallback
}

fn list_available_models(dir: &std::path::Path) -> Vec<ModelInfo> {
    let mut models = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "onnx").unwrap_or(false) {
                let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                models.push(ModelInfo {
                    name,
                    version: "unknown".into(),
                    input_size: 0,
                    num_classes: 0,
                    quantized: path.to_string_lossy().contains("int8"),
                    size_bytes: size,
                });
            }
        }
    }
    models
}
