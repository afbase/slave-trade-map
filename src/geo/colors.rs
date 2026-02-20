//! Color mappings for African and American source regions.
//! Ported from assets/js/colors.js

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct ColorScheme {
    pub african: HashMap<i64, &'static str>,
    pub american: HashMap<i64, &'static str>,
}

impl ColorScheme {
    pub fn new() -> Self {
        let mut african = HashMap::new();
        african.insert(60100_i64, "#c8a050");
        african.insert(60200, "#b89040");
        african.insert(60300, "#a88838");
        african.insert(60400, "#d4a856");
        african.insert(60500, "#c09848");
        african.insert(60600, "#b08838");
        african.insert(60700, "#9a7830");
        african.insert(60800, "#8a6828");
        african.insert(60900, "#7a5820");

        let mut american = HashMap::new();
        // North America (20xxx)
        american.insert(20100_i64, "#4a7a9a");
        american.insert(20300, "#4878a0");
        american.insert(20400, "#4670a0");
        american.insert(20500, "#4468a0");
        american.insert(20600, "#4a7090");
        american.insert(20700, "#486888");
        american.insert(20800, "#466080");
        american.insert(20900, "#445878");
        american.insert(21000, "#425070");
        american.insert(21100, "#405868");
        american.insert(21200, "#3e5060");
        american.insert(21300, "#3c4858");
        american.insert(21400, "#3a4050");
        american.insert(21500, "#384860");
        american.insert(21600, "#365070");
        american.insert(21700, "#345878");
        american.insert(21800, "#326080");
        american.insert(21900, "#305878");
        american.insert(22000, "#305878");
        american.insert(22100, "#305878");
        // Spanish Caribbean (31xxx)
        american.insert(31100, "#2a8a7a");
        american.insert(31200, "#288678");
        american.insert(31300, "#268270");
        american.insert(31400, "#247e68");
        // Dutch (32xxx)
        american.insert(32100, "#3a9a8a");
        american.insert(32200, "#409890");
        // British Caribbean (33-35xxx)
        american.insert(33200, "#3a8878");
        american.insert(33300, "#388474");
        american.insert(33400, "#368070");
        american.insert(33500, "#347c6c");
        american.insert(33600, "#327868");
        american.insert(33700, "#307464");
        american.insert(33800, "#2e7060");
        american.insert(34100, "#2c6c5c");
        american.insert(34200, "#2a6858");
        american.insert(34300, "#286454");
        american.insert(34400, "#266050");
        american.insert(34500, "#245c4c");
        american.insert(34600, "#225848");
        american.insert(35100, "#3a8070");
        american.insert(35200, "#387c6c");
        american.insert(35300, "#367868");
        american.insert(35400, "#347464");
        american.insert(35500, "#327060");
        // French Caribbean (36xxx)
        american.insert(36100, "#4878a0");
        american.insert(36200, "#4674a0");
        american.insert(36300, "#4470a0");
        american.insert(36400, "#3c6898");
        american.insert(36500, "#3a6498");
        // Danish (37xxx)
        american.insert(37000, "#506a90");
        // Other Caribbean
        american.insert(38100, "#4e6688");
        american.insert(39000, "#4c6280");
        // Spanish mainland (41-44xxx)
        american.insert(41200, "#2a7868");
        american.insert(42000, "#287060");
        american.insert(43000, "#266858");
        american.insert(44000, "#246050");
        // Brazil (50xxx)
        american.insert(50100, "#2a8070");
        american.insert(50200, "#288078");
        american.insert(50300, "#207060");
        american.insert(50400, "#186858");
        american.insert(50500, "#106050");
        // Unspecified
        american.insert(80200, "#607888");
        american.insert(80400, "#586878");
        american.insert(80700, "#506070");
        // European (as I-Am sources)
        american.insert(10100, "#5070a0");
        american.insert(10200, "#4e6c9a");
        american.insert(10300, "#4c6894");
        american.insert(10400, "#4a648e");
        american.insert(10500, "#486088");
        american.insert(10700, "#465c82");
        american.insert(10800, "#44587c");

        ColorScheme { african, american }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}

/// Get source color with parent-code fallback (same logic as JS getSourceColor).
/// Available for direct Rust use; the WASM path pushes colors to `window.__africanColors` /
/// `window.__americanColors` instead.
#[allow(dead_code)]
pub fn get_source_color(code: i64, dataset: &str) -> &'static str {
    let scheme = ColorScheme::new();
    let map = if dataset == "transatlantic" {
        &scheme.african
    } else {
        &scheme.american
    };
    if let Some(c) = map.get(&code) {
        return c;
    }
    let parent = (code / 100) * 100;
    if let Some(c) = map.get(&parent) {
        return c;
    }
    if dataset == "transatlantic" {
        "#8a7848"
    } else {
        "#4a7090"
    }
}
