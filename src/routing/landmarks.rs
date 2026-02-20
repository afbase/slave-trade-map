//! Maritime landmark coordinates used for route planning.
//! Ported from MARITIME_LANDMARKS in assets/js/route-planner.js

use crate::geo::coords::LatLng;

pub struct MaritimeLandmarks {
    pub cape_agulhas: LatLng,
    pub west_of_cape: LatLng,
    pub st_helena: LatLng,
    pub cape_verde: LatLng,
    pub ponta_do_seixas: LatLng,
    pub senegambia_offshore: LatLng,
    pub sierra_leone_offshore: LatLng,
    pub windward_offshore: LatLng,
    pub gold_coast_offshore: LatLng,
    pub benin_offshore: LatLng,
    pub biafra_offshore: LatLng,
    pub wca_offshore: LatLng,
    pub other_africa_coast: LatLng,
}

pub static LANDMARKS: std::sync::LazyLock<MaritimeLandmarks> =
    std::sync::LazyLock::new(|| MaritimeLandmarks {
        cape_agulhas: LatLng {
            lat: -34.83,
            lng: 20.0,
        },
        west_of_cape: LatLng {
            lat: -33.0,
            lng: 10.0,
        },
        st_helena: LatLng {
            lat: -15.97,
            lng: -5.71,
        },
        cape_verde: LatLng {
            lat: 14.93,
            lng: -23.51,
        },
        ponta_do_seixas: LatLng {
            lat: -7.15,
            lng: -33.8,
        },
        senegambia_offshore: LatLng {
            lat: 14.5,
            lng: -20.0,
        },
        sierra_leone_offshore: LatLng {
            lat: 8.5,
            lng: -16.0,
        },
        windward_offshore: LatLng {
            lat: 6.0,
            lng: -12.0,
        },
        gold_coast_offshore: LatLng {
            lat: 4.0,
            lng: -7.0,
        },
        benin_offshore: LatLng {
            lat: 5.0,
            lng: -4.0,
        },
        biafra_offshore: LatLng { lat: 4.0, lng: 2.0 },
        wca_offshore: LatLng {
            lat: -8.0,
            lng: 5.0,
        },
        other_africa_coast: LatLng { lat: 2.0, lng: 8.0 },
    });
