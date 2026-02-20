//! Destination quadrant classification for route planning.
//! Ported from assets/js/route-planner.js

use crate::geo::coords::Coord;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quadrant {
    Caribbean,
    NorthAmerica,
    BrazilNorth,
    BrazilSouth,
    RioPlata,
    Europe,
    PacificSouth,
    MidAtlantic,
    Unknown,
}

impl Quadrant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Quadrant::Caribbean => "caribbean",
            Quadrant::NorthAmerica => "north-america",
            Quadrant::BrazilNorth => "brazil-north",
            Quadrant::BrazilSouth => "brazil-south",
            Quadrant::RioPlata => "rio-plata",
            Quadrant::Europe => "europe",
            Quadrant::PacificSouth => "pacific-south",
            Quadrant::MidAtlantic => "mid-atlantic",
            Quadrant::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "caribbean" => Quadrant::Caribbean,
            "north-america" => Quadrant::NorthAmerica,
            "brazil-north" => Quadrant::BrazilNorth,
            "brazil-south" => Quadrant::BrazilSouth,
            "rio-plata" => Quadrant::RioPlata,
            "europe" => Quadrant::Europe,
            "pacific-south" => Quadrant::PacificSouth,
            "mid-atlantic" => Quadrant::MidAtlantic,
            _ => Quadrant::Unknown,
        }
    }

    pub fn is_brazil(&self) -> bool {
        matches!(self, Quadrant::BrazilNorth | Quadrant::BrazilSouth)
    }

    pub fn is_westward(&self) -> bool {
        matches!(self, Quadrant::Caribbean | Quadrant::NorthAmerica)
    }
}

/// Classify a destination code into a routing quadrant.
/// Ported from classifyDestQuadrant() in route-planner.js.
pub fn classify_dest_quadrant(code: i64, coords: &HashMap<i64, Coord>) -> Quadrant {
    let dc = match coords.get(&code) {
        Some(c) => c,
        None => return Quadrant::MidAtlantic,
    };
    let lat = dc.lat;
    let lng = dc.lng;

    if (8.0..=25.0).contains(&lat) && (-100.0..=-59.0).contains(&lng) {
        return Quadrant::Caribbean;
    }
    if lat > 25.0 && (-100.0..=-60.0).contains(&lng) {
        return Quadrant::NorthAmerica;
    }
    if (-10.0..=5.0).contains(&lat) && (-55.0..=-30.0).contains(&lng) {
        return Quadrant::BrazilNorth;
    }
    if (-25.0..-10.0).contains(&lat) && (-55.0..=-30.0).contains(&lng) {
        return Quadrant::BrazilSouth;
    }
    if lat < -25.0 && (-65.0..=-50.0).contains(&lng) {
        return Quadrant::RioPlata;
    }
    if lat > 35.0 && lng > -15.0 {
        return Quadrant::Europe;
    }
    if lat < -10.0 && lng < -65.0 {
        return Quadrant::PacificSouth;
    }
    Quadrant::MidAtlantic
}

/// Classify a source code into a routing quadrant.
/// Ported from classifySourceQuadrant() in route-planner.js.
pub fn classify_source_quadrant(code: i64, coords: &HashMap<i64, Coord>) -> Quadrant {
    let sc = coords
        .get(&code)
        .or_else(|| coords.get(&((code / 100) * 100)));
    let sc = match sc {
        Some(c) => c,
        None => return Quadrant::Unknown,
    };
    let lat = sc.lat;
    let lng = sc.lng;

    if (8.0..=25.0).contains(&lat) && (-100.0..=-59.0).contains(&lng) {
        return Quadrant::Caribbean;
    }
    if lat > 25.0 && (-100.0..=-60.0).contains(&lng) {
        return Quadrant::NorthAmerica;
    }
    // Note: JS uses -15 for source but -10 for dest
    if (-15.0..=5.0).contains(&lat) && (-55.0..=-30.0).contains(&lng) {
        return Quadrant::BrazilNorth;
    }
    if (-25.0..-15.0).contains(&lat) && (-55.0..=-30.0).contains(&lng) {
        return Quadrant::BrazilSouth;
    }
    if lat < -25.0 && (-65.0..=-50.0).contains(&lng) {
        return Quadrant::RioPlata;
    }
    if lat > 35.0 && lng > -15.0 {
        return Quadrant::Europe;
    }
    Quadrant::MidAtlantic
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::coords::build_region_coords;

    #[test]
    fn test_quadrant_roundtrip() {
        assert_eq!(Quadrant::Caribbean.as_str(), "caribbean");
        assert_eq!(Quadrant::from_str("caribbean"), Quadrant::Caribbean);
        assert_eq!(Quadrant::BrazilSouth.as_str(), "brazil-south");
        assert_eq!(Quadrant::from_str("brazil-south"), Quadrant::BrazilSouth);
        assert_eq!(Quadrant::PacificSouth.as_str(), "pacific-south");
        assert_eq!(Quadrant::from_str("pacific-south"), Quadrant::PacificSouth);
        assert_eq!(Quadrant::MidAtlantic.as_str(), "mid-atlantic");
        assert_eq!(Quadrant::from_str("mid-atlantic"), Quadrant::MidAtlantic);
    }

    #[test]
    fn test_jamaica_is_caribbean() {
        let coords = build_region_coords();
        assert_eq!(classify_dest_quadrant(35100, &coords), Quadrant::Caribbean);
    }

    #[test]
    fn test_se_brazil() {
        let coords = build_region_coords();
        assert_eq!(
            classify_dest_quadrant(50400, &coords),
            Quadrant::BrazilSouth
        );
    }

    #[test]
    fn test_europe() {
        let coords = build_region_coords();
        // Lisbon is 10203
        assert_eq!(classify_dest_quadrant(10203, &coords), Quadrant::Europe);
    }

    #[test]
    fn test_cuba_is_caribbean() {
        let coords = build_region_coords();
        assert_eq!(classify_dest_quadrant(31300, &coords), Quadrant::Caribbean);
    }

    #[test]
    fn test_unknown_code_is_mid_atlantic() {
        let coords = build_region_coords();
        assert_eq!(
            classify_dest_quadrant(99999, &coords),
            Quadrant::MidAtlantic
        );
    }

    #[test]
    fn test_rio_plata() {
        let coords = build_region_coords();
        // Buenos Aires: lat -34.6, lng -58.4
        assert_eq!(classify_dest_quadrant(42000, &coords), Quadrant::RioPlata);
    }

    #[test]
    fn test_north_america() {
        let coords = build_region_coords();
        // Charleston SC: lat 32.78, lng -79.93
        assert_eq!(
            classify_dest_quadrant(21300, &coords),
            Quadrant::NorthAmerica
        );
    }

    #[test]
    fn test_brazil_north() {
        let coords = build_region_coords();
        // Pernambuco: lat -8, lng -35
        assert_eq!(
            classify_dest_quadrant(50300, &coords),
            Quadrant::BrazilNorth
        );
    }

    #[test]
    fn test_source_quadrant_caribbean() {
        let coords = build_region_coords();
        assert_eq!(
            classify_source_quadrant(31300, &coords),
            Quadrant::Caribbean
        );
    }

    #[test]
    fn test_source_quadrant_unknown() {
        let coords = build_region_coords();
        assert_eq!(classify_source_quadrant(99999, &coords), Quadrant::Unknown);
    }
}
