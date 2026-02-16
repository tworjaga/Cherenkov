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

/// Rule-based isotope classifier using peak matching
pub struct IsotopeClassifier {
    library: IsotopeLibrary,
    tolerance_kev: f64,
}

#[derive(Debug, Clone)]
pub struct IsotopeMatch {
    pub symbol: String,
    pub confidence: f64,
    pub matched_peaks: Vec<MatchedPeak>,
}

#[derive(Debug, Clone)]
pub struct MatchedPeak {
    pub energy_kev: f64,
    pub intensity: f64,
    pub library_energy: f64,
}

impl IsotopeClassifier {
    pub fn new(tolerance_kev: f64) -> Self {
        Self {
            library: IsotopeLibrary::standard(),
            tolerance_kev,
        }
    }

    /// Classify isotopes from spectral peaks
    /// energies: detected gamma peak energies in keV
    /// counts: peak intensities/counts
    pub fn classify(&self, energies: &[f64], counts: &[f64]) -> Vec<IsotopeMatch> {
        if energies.len() != counts.len() || energies.is_empty() {
            return vec![];
        }

        let mut matches: Vec<IsotopeMatch> = vec![];

        for isotope in &self.library.isotopes {
            let mut matched_peaks: Vec<MatchedPeak> = vec![];
            let mut total_confidence = 0.0;

            for (lib_energy, &detected_energy) in isotope.gamma_energies.iter().zip(energies.iter()) {
                let distance = (lib_energy - detected_energy).abs();
                
                if distance < self.tolerance_kev {
                    // Calculate confidence based on distance (closer = higher confidence)
                    let confidence = 1.0 - (distance / self.tolerance_kev);
                    let intensity = counts.get(matched_peaks.len()).copied().unwrap_or(1.0);
                    
                    matched_peaks.push(MatchedPeak {
                        energy_kev: detected_energy,
                        intensity,
                        library_energy: *lib_energy,
                    });
                    
                    total_confidence += confidence;
                }
            }

            // Require at least one peak match
            if !matched_peaks.is_empty() {
                let avg_confidence = total_confidence / isotope.gamma_energies.len() as f64;
                
                matches.push(IsotopeMatch {
                    symbol: isotope.symbol.clone(),
                    confidence: avg_confidence.min(1.0),
                    matched_peaks,
                });
            }
        }

        // Sort by confidence (highest first)
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        matches
    }

    /// Quick check for specific isotope signature
    pub fn check_isotope(&self, symbol: &str, energies: &[f64]) -> Option<IsotopeMatch> {
        let isotope = self.library.find_by_symbol(symbol)?;
        let mut matched_peaks = vec![];
        let mut total_confidence = 0.0;

        for lib_energy in &isotope.gamma_energies {
            for &detected in energies {
                let distance = (lib_energy - detected).abs();
                if distance < self.tolerance_kev {
                    let confidence = 1.0 - (distance / self.tolerance_kev);
                    matched_peaks.push(MatchedPeak {
                        energy_kev: detected,
                        intensity: 1.0,
                        library_energy: *lib_energy,
                    });
                    total_confidence += confidence;
                    break;
                }
            }
        }

        if matched_peaks.is_empty() {
            return None;
        }

        let avg_confidence = total_confidence / isotope.gamma_energies.len() as f64;

        Some(IsotopeMatch {
            symbol: isotope.symbol.clone(),
            confidence: avg_confidence.min(1.0),
            matched_peaks,
        })
    }
}
