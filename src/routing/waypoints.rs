//! Build waypoint chains for slave trade routes.
//! Ported from buildWaypointChain in assets/js/route-planner.js

use crate::geo::coords::{Coord, LatLng};
use crate::routing::landmarks::LANDMARKS;
use crate::routing::quadrant::{classify_source_quadrant, Quadrant};
use std::collections::HashMap;

fn ll(lat: f64, lng: f64) -> LatLng {
    LatLng { lat, lng }
}

/// Build a waypoint chain for a given source code and destination quadrant.
/// Returns None for routes that need only a simple arc (no waypoints needed).
/// Ported from buildWaypointChain() in route-planner.js.
pub fn build_waypoint_chain(
    source_code: i64,
    dest_quadrant: &Quadrant,
    coords: &HashMap<i64, Coord>,
) -> Option<Vec<LatLng>> {
    let src = (source_code / 100) * 100;
    let m = &*LANDMARKS;

    match src {
        // SE Africa
        60800 => {
            let mut chain = vec![
                m.cape_agulhas.clone(),
                m.west_of_cape.clone(),
                m.st_helena.clone(),
            ];
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                chain.push(ll(-14.0, -20.0));
            } else if dest_quadrant.is_westward() {
                chain.push(ll(-6.0, -18.0));
                chain.push(ll(-4.0, -26.0));
            } else if *dest_quadrant == Quadrant::Europe {
                chain.push(ll(0.0, -16.0));
                chain.push(ll(10.0, -20.0));
            }
            Some(chain)
        }

        // W Central Africa
        60700 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.wca_offshore.clone(),
                    m.st_helena.clone(),
                    ll(-11.0, -18.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.wca_offshore.clone(),
                    ll(-1.0, -12.0),
                    ll(2.0, -32.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.wca_offshore.clone(),
                    m.st_helena.clone(),
                    ll(8.0, -16.0),
                    ll(16.0, -18.0),
                ]);
            }
            Some(vec![m.wca_offshore.clone(), ll(0.0, -15.0)])
        }

        // Bight of Biafra
        60600 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.biafra_offshore.clone(),
                    ll(-5.0, -5.0),
                    ll(-8.0, -17.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.biafra_offshore.clone(),
                    ll(7.0, -20.0),
                    ll(8.0, -38.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.biafra_offshore.clone(),
                    ll(14.0, -10.0),
                    ll(25.0, -14.0),
                ]);
            }
            Some(vec![m.biafra_offshore.clone(), ll(0.0, -10.0)])
        }

        // Bight of Benin
        60500 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.benin_offshore.clone(),
                    ll(0.0, -10.0),
                    ll(-4.0, -18.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.benin_offshore.clone(),
                    ll(11.0, -25.0),
                    ll(14.0, -42.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.benin_offshore.clone(),
                    ll(16.0, -10.0),
                    ll(34.0, -12.0),
                ]);
            }
            Some(vec![m.benin_offshore.clone(), ll(5.0, -15.0)])
        }

        // Gold Coast
        60400 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.gold_coast_offshore.clone(),
                    ll(-2.0, -14.0),
                    ll(-6.0, -18.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.gold_coast_offshore.clone(),
                    ll(9.0, -28.0),
                    ll(10.0, -43.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.gold_coast_offshore.clone(),
                    ll(16.0, -12.0),
                    ll(28.0, -11.0),
                ]);
            }
            Some(vec![m.gold_coast_offshore.clone(), ll(5.0, -20.0)])
        }

        // Windward Coast
        60300 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.windward_offshore.clone(),
                    ll(-1.0, -18.0),
                    ll(-5.0, -20.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.windward_offshore.clone(),
                    ll(10.0, -30.0),
                    ll(12.0, -45.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.windward_offshore.clone(),
                    ll(18.0, -16.0),
                    ll(31.0, -13.0),
                ]);
            }
            Some(vec![m.windward_offshore.clone(), ll(5.0, -25.0)])
        }

        // Sierra Leone
        60200 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.sierra_leone_offshore.clone(),
                    ll(2.0, -20.0),
                    ll(-2.0, -22.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.sierra_leone_offshore.clone(),
                    ll(13.0, -35.0),
                    ll(16.0, -50.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.sierra_leone_offshore.clone(),
                    ll(20.0, -20.0),
                    ll(37.0, -13.0),
                ]);
            }
            Some(vec![m.sierra_leone_offshore.clone(), ll(5.0, -28.0)])
        }

        // Senegambia
        60100 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.senegambia_offshore.clone(),
                    m.cape_verde.clone(),
                    ll(5.0, -28.0),
                    ll(0.0, -24.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.senegambia_offshore.clone(),
                    m.cape_verde.clone(),
                    ll(18.0, -45.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.senegambia_offshore.clone(),
                    ll(28.0, -18.0),
                    ll(40.0, -12.0),
                ]);
            }
            Some(vec![m.senegambia_offshore.clone(), m.cape_verde.clone()])
        }

        // Other Africa / Interior
        60900 => {
            if dest_quadrant.is_brazil() || *dest_quadrant == Quadrant::RioPlata {
                return Some(vec![
                    m.other_africa_coast.clone(),
                    ll(0.0, 0.0),
                    ll(-7.0, -8.0),
                    ll(-9.0, -16.0),
                ]);
            }
            if dest_quadrant.is_westward() {
                return Some(vec![
                    m.other_africa_coast.clone(),
                    ll(2.0, 0.0),
                    ll(5.0, -20.0),
                    ll(6.0, -36.0),
                ]);
            }
            if *dest_quadrant == Quadrant::Europe {
                return Some(vec![
                    m.other_africa_coast.clone(),
                    ll(2.0, 0.0),
                    ll(12.0, -10.0),
                    ll(22.0, -15.0),
                ]);
            }
            Some(vec![
                m.other_africa_coast.clone(),
                ll(0.0, 0.0),
                ll(-2.0, -10.0),
            ])
        }

        // Inter-American sources (default case in JS)
        _ => build_interamerican_chain(source_code, src, dest_quadrant, coords),
    }
}

/// Handle inter-American routing (the default case in the JS switch).
fn build_interamerican_chain(
    source_code: i64,
    src: i64,
    dest_quadrant: &Quadrant,
    coords: &HashMap<i64, Coord>,
) -> Option<Vec<LatLng>> {
    let src_quadrant = classify_source_quadrant(source_code, coords);
    if src_quadrant == Quadrant::Unknown {
        return None;
    }

    let m = &*LANDMARKS;

    // Brazilian sources
    if src_quadrant == Quadrant::BrazilNorth || src_quadrant == Quadrant::BrazilSouth {
        let src_coord = coords.get(&source_code).or_else(|| coords.get(&src));
        let far_west = src_coord.is_some_and(|c| c.lng < -42.0);

        match dest_quadrant {
            Quadrant::RioPlata => {
                if far_west {
                    return Some(vec![
                        ll(-12.0, -36.0),
                        ll(-20.0, -39.0),
                        ll(-28.0, -46.0),
                        ll(-30.0, -48.0),
                    ]);
                }
                if src_quadrant == Quadrant::BrazilSouth {
                    return Some(vec![ll(-25.0, -44.0), ll(-30.0, -48.0)]);
                }
                return Some(vec![
                    m.ponta_do_seixas.clone(),
                    ll(-13.0, -35.0),
                    ll(-20.0, -38.0),
                    ll(-25.0, -44.0),
                    ll(-30.0, -48.0),
                ]);
            }
            Quadrant::PacificSouth => {
                if far_west {
                    return Some(vec![
                        ll(-15.0, -42.0),
                        ll(-30.0, -48.0),
                        ll(-45.0, -60.0),
                        ll(-50.0, -72.0),
                    ]);
                }
                if src_quadrant == Quadrant::BrazilSouth {
                    return Some(vec![ll(-30.0, -48.0), ll(-45.0, -60.0), ll(-50.0, -72.0)]);
                }
                return Some(vec![
                    m.ponta_do_seixas.clone(),
                    ll(-20.0, -38.0),
                    ll(-35.0, -52.0),
                    ll(-45.0, -60.0),
                    ll(-50.0, -72.0),
                ]);
            }
            q if q.is_westward() => {
                if far_west {
                    return Some(vec![ll(0.0, -42.0), ll(10.0, -50.0)]);
                }
                if src_quadrant == Quadrant::BrazilSouth {
                    return Some(vec![
                        ll(-15.0, -36.0),
                        m.ponta_do_seixas.clone(),
                        ll(0.0, -40.0),
                    ]);
                }
                return Some(vec![
                    m.ponta_do_seixas.clone(),
                    ll(0.0, -40.0),
                    ll(10.0, -50.0),
                ]);
            }
            q if q.is_brazil() => {
                if far_west {
                    return Some(vec![ll(-10.0, -36.0)]);
                }
                if src_quadrant != *dest_quadrant {
                    return Some(vec![m.ponta_do_seixas.clone()]);
                }
                return None; // same sub-region: simple arc
            }
            _ => return None,
        }
    }

    // Caribbean sources
    if src_quadrant == Quadrant::Caribbean {
        match dest_quadrant {
            Quadrant::Caribbean | Quadrant::NorthAmerica => return None,
            Quadrant::RioPlata | Quadrant::BrazilSouth => {
                return Some(vec![
                    ll(8.0, -58.0),
                    ll(0.0, -45.0),
                    m.ponta_do_seixas.clone(),
                    ll(-15.0, -36.0),
                    ll(-25.0, -44.0),
                ]);
            }
            Quadrant::BrazilNorth => {
                return Some(vec![ll(8.0, -55.0), ll(0.0, -42.0)]);
            }
            Quadrant::PacificSouth => {
                return Some(vec![
                    ll(10.0, -72.0),
                    ll(5.0, -78.0),
                    ll(-10.0, -80.0),
                    ll(-35.0, -75.0),
                    ll(-45.0, -73.0),
                ]);
            }
            _ => return None,
        }
    }

    // North American sources
    if src_quadrant == Quadrant::NorthAmerica {
        match dest_quadrant {
            Quadrant::Caribbean | Quadrant::NorthAmerica => return None,
            Quadrant::RioPlata | Quadrant::BrazilSouth => {
                return Some(vec![
                    ll(25.0, -75.0),
                    ll(15.0, -65.0),
                    ll(5.0, -50.0),
                    m.ponta_do_seixas.clone(),
                    ll(-20.0, -38.0),
                    ll(-30.0, -48.0),
                ]);
            }
            Quadrant::BrazilNorth => {
                return Some(vec![ll(25.0, -72.0), ll(15.0, -60.0), ll(5.0, -45.0)]);
            }
            Quadrant::PacificSouth => {
                return Some(vec![
                    ll(25.0, -82.0),
                    ll(15.0, -80.0),
                    ll(5.0, -78.0),
                    ll(-10.0, -80.0),
                    ll(-35.0, -75.0),
                    ll(-45.0, -73.0),
                ]);
            }
            _ => return None,
        }
    }

    // European sources — simple arc
    if src_quadrant == Quadrant::Europe {
        return None;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::coords::build_region_coords;

    #[test]
    fn test_se_africa_to_caribbean() {
        let coords = build_region_coords();
        let chain = build_waypoint_chain(60800, &Quadrant::Caribbean, &coords);
        assert!(chain.is_some());
        let chain = chain.unwrap();
        // Should have cape_agulhas, west_of_cape, st_helena, plus 2 westward waypoints
        assert!(chain.len() >= 4);
        // First waypoint is cape_agulhas
        assert!((chain[0].lat - (-34.83)).abs() < 0.01);
    }

    #[test]
    fn test_se_africa_to_brazil() {
        let coords = build_region_coords();
        let chain = build_waypoint_chain(60800, &Quadrant::BrazilSouth, &coords);
        assert!(chain.is_some());
        let chain = chain.unwrap();
        assert_eq!(chain.len(), 4); // cape, west, st_helena, mid-point
    }

    #[test]
    fn test_senegambia_to_caribbean() {
        let coords = build_region_coords();
        let chain = build_waypoint_chain(60100, &Quadrant::Caribbean, &coords);
        assert!(chain.is_some());
        let chain = chain.unwrap();
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_caribbean_to_caribbean_is_none() {
        let coords = build_region_coords();
        let chain = build_waypoint_chain(31300, &Quadrant::Caribbean, &coords);
        assert!(chain.is_none());
    }

    #[test]
    fn test_wca_to_europe() {
        let coords = build_region_coords();
        let chain = build_waypoint_chain(60700, &Quadrant::Europe, &coords);
        assert!(chain.is_some());
        let chain = chain.unwrap();
        assert_eq!(chain.len(), 4);
    }

    #[test]
    fn test_port_level_code() {
        let coords = build_region_coords();
        // 60805 / 100 * 100 = 60800 — should use SE Africa chain
        let chain = build_waypoint_chain(60805, &Quadrant::Caribbean, &coords);
        assert!(chain.is_some());
    }
}
