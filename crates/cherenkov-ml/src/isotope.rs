use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsotopeLibrary {
    pub isotopes: Vec<Isotope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Isotope {
    pub symbol: String,
    pub name: String,
    pub half_life_seconds: f64,
    pub decay_mode: DecayMode,
    pub gamma_energies: Vec<f64>,
    pub beta_energies: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayMode {
    Alpha,
    Beta,
    Gamma,
    ElectronCapture,
    IsomericTransition,
}

impl IsotopeLibrary {
    pub fn standard() -> Self {
        Self {
            isotopes: vec![
                Isotope {
                    symbol: "Cs-137".to_string(),
                    name: "Cesium-137".to_string(),
                    half_life_seconds: 30.17 * 365.25 * 24.0 * 3600.0,
                    decay_mode: DecayMode::Beta,
                    gamma_energies: vec![661.7],
                    beta_energies: vec![1173.2, 514.0],
                },
                Isotope {
                    symbol: "Co-60".to_string(),
                    name: "Cobalt-60".to_string(),
                    half_life_seconds: 5.27 * 365.25 * 24.0 * 3600.0,
                    decay_mode: DecayMode::Beta,
                    gamma_energies: vec![1173.2, 1332.5],
                    beta_energies: vec![317.9],
                },
                Isotope {
                    symbol: "Am-241".to_string(),
                    name: "Americium-241".to_string(),
                    half_life_seconds: 432.2 * 365.25 * 24.0 * 3600.0,
                    decay_mode: DecayMode::Alpha,
                    gamma_energies: vec![59.5],
                    beta_energies: vec![],
                },
                Isotope {
                    symbol: "I-131".to_string(),
                    name: "Iodine-131".to_string(),
                    half_life_seconds: 8.02 * 24.0 * 3600.0,
                    decay_mode: DecayMode::Beta,
                    gamma_energies: vec![364.5, 637.0],
                    beta_energies: vec![606.3, 333.8],
                },
            ],
        }
    }
    
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&Isotope> {
        self.isotopes.iter().find(|i| i.symbol == symbol)
    }
    
    pub fn find_by_energy(&self, energy: f64, tolerance: f64) -> Vec<&Isotope> {
        self.isotopes.iter()
            .filter(|i| i.gamma_energies.iter().any(|&e| (e - energy).abs() < tolerance))
            .collect()
    }
}
