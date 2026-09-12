/// Model registry: loads model specs from models.toml
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec {
    pub name: String,
    pub provider: String,
    pub context_window: usize,
    pub max_output: usize,
    pub input_price_per_1m: f64,
    pub output_price_per_1m: f64,
    pub released: String,
    pub description: String,
}

pub struct ModelRegistry {
    models: HashMap<String, ModelSpec>,
}

impl ModelRegistry {
    /// Embedded models.toml: always bundled in the binary via include_str!.
    /// This guarantees `cargo install ctx-budget` works standalone, with no
    /// external file dependency at runtime.
    const EMBEDDED_MODELS_TOML: &'static str = include_str!("../models.toml");

    /// Load models from the embedded models.toml (bundled at compile time)
    pub fn load_embedded() -> anyhow::Result<Self> {
        Self::from_str(Self::EMBEDDED_MODELS_TOML)
    }

    /// Load models from an external models.toml path (overrides embedded data)
    pub fn from_toml(toml_path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(toml_path)?;
        Self::from_str(&content)
    }

    /// Parse models from a TOML string
    fn from_str(content: &str) -> anyhow::Result<Self> {
        let table: toml::Table = toml::from_str(content)?;

        let mut models = HashMap::new();

        if let Some(models_table) = table.get("models").and_then(|m| m.as_table()) {
            for (model_id, model_value) in models_table {
                let spec: ModelSpec = model_value.clone().try_into()?;
                models.insert(model_id.clone(), spec);
            }
        }

        Ok(ModelRegistry { models })
    }

    /// Get model by ID
    pub fn get(&self, model_id: &str) -> Option<&ModelSpec> {
        self.models.get(model_id)
    }

    /// List all available models (sorted by context window descending)
    pub fn list_models(&self) -> Vec<(String, &ModelSpec)> {
        let mut list: Vec<_> = self
            .models
            .iter()
            .map(|(id, spec)| (id.clone(), spec))
            .collect();
        list.sort_by(|a, b| b.1.context_window.cmp(&a.1.context_window));
        list
    }

    /// Get default model ID (gpt-6-astra, fall back to gpt-4o)
    pub fn default_model(&self) -> String {
        if self.models.contains_key("gpt-6-astra") {
            "gpt-6-astra".to_string()
        } else if self.models.contains_key("gpt-4o") {
            "gpt-4o".to_string()
        } else {
            self.models
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| "unknown".to_string())
        }
    }

    /// Create empty default registry (fallback when models.toml not found)
    pub fn new_default() -> Self {
        ModelRegistry {
            models: HashMap::new(),
        }
    }
}

impl std::fmt::Debug for ModelRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelRegistry")
            .field("models_count", &self.models.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registry_load() {
        // Will test once models.toml is in place
        // let registry = ModelRegistry::from_toml(Path::new("models.toml"))
        //     .expect("Failed to load models");
        // assert!(registry.models.len() > 0);
    }
}
