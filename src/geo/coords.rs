//! Geographic coordinates for all region/port codes.
//! Ported from assets/js/coords.js

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coord {
    pub name: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

static REGION_COORDS: OnceLock<HashMap<i64, Coord>> = OnceLock::new();
pub fn region_coords() -> &'static HashMap<i64, Coord> {
    REGION_COORDS.get_or_init(build_region_coords)
}

static STATIC_WAYPOINTS: OnceLock<HashMap<i64, Vec<LatLng>>> = OnceLock::new();
pub fn static_waypoints() -> &'static HashMap<i64, Vec<LatLng>> {
    STATIC_WAYPOINTS.get_or_init(build_static_waypoints)
}

macro_rules! coord {
    ($name:expr, $lat:expr, $lng:expr) => {
        Coord {
            name: $name.to_string(),
            lat: $lat,
            lng: $lng,
        }
    };
}

/// Build the full region coordinates map (ported from regionCoords in coords.js)
// 6.28 is the latitude of Grand Popo, not the math constant TAU.
#[allow(clippy::approx_constant)]
pub fn build_region_coords() -> HashMap<i64, Coord> {
    let mut m = HashMap::new();

    // ── African embarkation regions ──────────────────────────────────────

    // Senegambia
    m.insert(60100, coord!("Senegambia", 14.5, -16.5));
    m.insert(60101, coord!("Albreda", 13.33, -16.38));
    m.insert(60102, coord!("Arguim", 20.60, -16.45));
    m.insert(60103, coord!("Bissagos", 11.30, -16.00));
    m.insert(60107, coord!("Galam", 14.90, -12.45));
    m.insert(60111, coord!("Joal, or Saloum River", 14.17, -16.85));
    m.insert(60113, coord!("Portudal", 14.45, -17.02));
    m.insert(60117, coord!("Rio Grande", 11.60, -15.50));
    m.insert(60104, coord!("Bissau", 11.86, -15.60));
    m.insert(60105, coord!("Cacheu", 12.27, -16.17));
    m.insert(60106, coord!("Casamance", 12.59, -16.27));
    m.insert(60108, coord!("Gambia", 13.45, -16.58));
    m.insert(60109, coord!("Goree", 14.67, -17.40));
    m.insert(60110, coord!("James Fort", 13.47, -16.56));
    m.insert(60114, coord!("Portuguese Guinea", 12.00, -14.95));
    m.insert(60118, coord!("Senegal", 16.02, -16.49));
    m.insert(60126, coord!("Cape Verde Islands", 14.93, -23.51));
    m.insert(60128, coord!("Madeira", 32.65, -16.91));
    m.insert(60141, coord!("Canary Islands", 28.10, -15.42));

    // Sierra Leone
    m.insert(60200, coord!("Sierra Leone", 8.5, -13.0));
    m.insert(60204, coord!("Cacandia", 10.95, -14.30));
    m.insert(60211, coord!("Iles Plantain", 7.90, -13.00));
    m.insert(60214, coord!("Scarcies", 8.85, -13.15));
    m.insert(60241, coord!("Mano", 6.92, -11.50));
    m.insert(60242, coord!("Kissey River", 8.48, -13.20));
    m.insert(60201, coord!("Banana Islands", 7.58, -12.87));
    m.insert(60202, coord!("Bance Island", 8.63, -13.02));
    m.insert(60207, coord!("Freetown", 8.49, -13.23));
    m.insert(60208, coord!("Gallinhas", 7.08, -11.41));
    m.insert(60210, coord!("Iles de Los", 9.47, -13.80));
    m.insert(60212, coord!("Rio Nunez", 10.55, -14.27));
    m.insert(60213, coord!("Rio Pongo", 10.12, -13.97));
    m.insert(60217, coord!("Sherbro", 7.44, -12.50));
    m.insert(60220, coord!("Sierra Leone estuary", 8.50, -13.10));

    // Windward Coast
    m.insert(60300, coord!("Windward Coast", 5.0, -9.0));
    m.insert(60310, coord!("Cess", 5.45, -9.58));
    m.insert(60314, coord!("Drouin", 4.90, -6.15));
    m.insert(60316, coord!("Little Bassa", 6.10, -10.20));
    m.insert(60318, coord!("Grand Junk", 6.10, -10.40));
    m.insert(60319, coord!("Little Junk", 6.15, -10.45));
    m.insert(60322, coord!("Petit Mesurado", 6.30, -10.78));
    m.insert(60324, coord!("Rio Assini", 5.12, -3.28));
    m.insert(60330, coord!("Rock Sestos", 5.47, -9.60));
    m.insert(60331, coord!("St. Paul", 6.38, -10.80));
    m.insert(60332, coord!("Tabou", 4.42, -7.35));
    m.insert(60333, coord!("Trade Town", 5.75, -9.90));
    m.insert(60340, coord!("Garroway Roads", 4.57, -7.75));
    m.insert(60302, coord!("Bassa", 5.90, -10.05));
    m.insert(60305, coord!("Cap des Palmes", 4.38, -7.71));
    m.insert(60306, coord!("Cape Mount", 6.76, -11.33));
    m.insert(60309, coord!("Cape Lahou", 5.13, -5.02));
    m.insert(60315, coord!("Grand Bassa", 5.93, -10.05));
    m.insert(60317, coord!("Grand Bassam", 5.20, -3.74));
    m.insert(60321, coord!("Grand Mesurado", 6.31, -10.81));
    m.insert(60328, coord!("Sassandra", 4.95, -6.09));
    m.insert(60334, coord!("Liberia", 6.30, -10.80));
    m.insert(60335, coord!("Ivory Coast", 5.32, -4.01));
    m.insert(60329, coord!("Grand Sestos", 4.67, -8.20));
    m.insert(60339, coord!("Cape Palmas", 4.38, -7.71));

    // Gold Coast
    m.insert(60400, coord!("Gold Coast", 5.2, -1.0));
    m.insert(60405, coord!("Alampo", 5.75, 0.45));
    m.insert(60413, coord!("Chama", 5.02, -1.63));
    m.insert(60425, coord!("Wiamba", 5.35, -0.62));
    m.insert(60402, coord!("Danish Gold Coast", 5.77, -0.18));
    m.insert(60404, coord!("Accra", 5.55, -0.20));
    m.insert(60407, coord!("Anomabu", 5.18, -1.12));
    m.insert(60408, coord!("Anomabu/Adja/Agga", 5.20, -1.10));
    m.insert(60409, coord!("Apammin", 5.20, -1.50));
    m.insert(60410, coord!("Apollonia", 5.01, -2.60));
    m.insert(60411, coord!("Axim", 4.87, -2.24));
    m.insert(60412, coord!("Cape Coast Castle", 5.10, -1.25));
    m.insert(60414, coord!("Christiansborg", 5.55, -0.18));
    m.insert(60415, coord!("Elmina", 5.08, -1.35));
    m.insert(60417, coord!("Kormantine", 5.20, -1.03));
    m.insert(60419, coord!("Mouree", 5.19, -1.13));
    m.insert(60422, coord!("Sekondi", 4.93, -1.71));
    m.insert(60428, coord!("Pokesoe", 5.01, -1.56));
    m.insert(60427, coord!("Gold Coast E of Kormantine", 5.30, -0.80));
    m.insert(60429, coord!("Boutry", 4.81, -1.97));

    // Bight of Benin
    m.insert(60500, coord!("Bight of Benin", 6.5, 2.5));
    m.insert(60503, coord!("Aghway", 6.25, 1.68));
    m.insert(60519, coord!("Lay", 5.75, 0.20));
    m.insert(60521, coord!("Oere", 5.52, 5.75));
    m.insert(60524, coord!("Rio Forcados", 5.35, 5.35));
    m.insert(60526, coord!("Rio Nun", 4.30, 6.10));
    m.insert(60501, coord!("Benin", 6.34, 5.63));
    m.insert(60502, coord!("Costa da Mina", 6.30, 2.50));
    m.insert(60506, coord!("Ardra", 6.40, 2.15));
    m.insert(60507, coord!("Badagry", 6.42, 2.89));
    m.insert(60510, coord!("Popo", 6.26, 1.80));
    m.insert(60511, coord!("Little Popo", 6.22, 1.63));
    m.insert(60512, coord!("Grand Popo", 6.28, 1.82));
    m.insert(60514, coord!("Jacquin", 6.36, 2.42));
    m.insert(60515, coord!("Whydah", 6.36, 2.08));
    m.insert(60516, coord!("Keta", 5.92, 0.99));
    m.insert(60517, coord!("Lagos", 6.45, 3.40));
    m.insert(60509, coord!("Epe", 6.58, 3.98));
    m.insert(60523, coord!("Porto Novo", 6.50, 2.63));

    // Bight of Biafra
    m.insert(60600, coord!("Bight of Biafra", 4.0, 8.5));
    m.insert(60615, coord!("Corisco", 0.92, 9.33));
    m.insert(60601, coord!("Andony", 4.35, 7.47));
    m.insert(60604, coord!("Bimbia", 4.01, 9.31));
    m.insert(60605, coord!("Bonny", 4.45, 7.17));
    m.insert(60608, coord!("Calabar", 4.96, 8.33));
    m.insert(60609, coord!("Cameroons", 4.05, 9.70));
    m.insert(60610, coord!("Cameroons River", 4.00, 9.60));
    m.insert(60612, coord!("Cap Lopez", -0.63, 8.71));
    m.insert(60620, coord!("Gabon", 0.39, 9.45));
    m.insert(60622, coord!("New Calabar", 4.72, 7.01));
    m.insert(60624, coord!("River Brass", 4.31, 6.24));
    m.insert(60625, coord!("River del Rey", 4.65, 8.76));
    m.insert(60663, coord!("Fernando Po", 3.45, 8.77));
    m.insert(60668, coord!("Princes Island", 1.62, 7.41));
    m.insert(60673, coord!("Sao Tome", 0.19, 6.61));
    m.insert(60676, coord!("Sao Tome or Princes Island", 1.00, 7.00));

    // West Central Africa
    m.insert(60700, coord!("West Central Africa", -6.5, 13.0));
    m.insert(60712, coord!("Cabo Lopo Goncalves", -0.63, 8.70));
    m.insert(60715, coord!("Coanza River", -9.35, 13.15));
    m.insert(60718, coord!("Rio Zaire", -6.05, 12.40));
    m.insert(60722, coord!("Kilongo", -4.30, 11.60));
    m.insert(60729, coord!("Mpinda", -6.12, 12.37));
    m.insert(60732, coord!("Quicombo", -11.33, 13.82));
    m.insert(60703, coord!("Ambriz", -7.86, 13.13));
    m.insert(60707, coord!("Benguela", -12.58, 13.41));
    m.insert(60709, coord!("Boary", -5.00, 12.00));
    m.insert(60711, coord!("Cabinda", -5.55, 12.19));
    m.insert(60716, coord!("Congo North", -4.32, 11.85));
    m.insert(60717, coord!("Congo River", -6.07, 12.43));
    m.insert(60721, coord!("Kacongo", -5.20, 12.15));
    m.insert(60724, coord!("Loango", -4.65, 11.81));
    m.insert(60725, coord!("Malembo", -5.36, 12.35));
    m.insert(60726, coord!("Mayumba", -3.43, 10.66));
    m.insert(60730, coord!("Nova Redonda", -11.03, 13.80));
    m.insert(60734, coord!("St. Paul de Loanda", -8.84, 13.23));
    m.insert(60740, coord!("St. Helena", -15.97, -5.71));
    m.insert(60741, coord!("Soyo", -6.13, 12.37));

    // Southeast Africa
    m.insert(60800, coord!("Southeast Africa", -18.0, 38.0));
    m.insert(60808, coord!("Quirimba", -12.40, 40.60));
    m.insert(60819, coord!("Momboza or Zanzibar", -5.00, 39.50));
    m.insert(60802, coord!("Cabo Delgado", -10.69, 40.64));
    m.insert(60803, coord!("Cape of Good Hope", -33.93, 18.42));
    m.insert(60806, coord!("Ibo", -12.34, 40.59));
    m.insert(60807, coord!("Inhambane", -23.86, 35.38));
    m.insert(60809, coord!("Kilwa", -8.95, 39.51));
    m.insert(60810, coord!("Lourenco Marques", -25.97, 32.58));
    m.insert(60811, coord!("Madagascar", -18.91, 47.52));
    m.insert(60820, coord!("Mozambique", -15.04, 40.73));
    m.insert(60822, coord!("Quilimane", -17.88, 36.89));
    m.insert(60823, coord!("Sofala", -20.12, 34.72));
    m.insert(60826, coord!("Zanzibar", -6.16, 39.19));
    m.insert(60831, coord!("Mascarene Islands", -20.35, 57.55));
    m.insert(60837, coord!("Mauritius", -20.35, 57.55));

    // Other Africa
    m.insert(60900, coord!("Other Africa", -2.5, 4.5));
    m.insert(60912, coord!("Windward+Ivory+Gold+Benin", 6.00, -2.00));
    m.insert(60914, coord!("Gold Coast+Benin+Biafra", 5.50, 3.00));
    m.insert(60915, coord!("Bights", 5.00, 5.50));
    m.insert(60916, coord!("Windward Coast (Nunez-Assini)", 7.50, -8.00));
    m.insert(60917, coord!("Gold Coast, Fr. definition", 5.50, -1.00));

    // ── American disembarkation regions ──────────────────────────────────

    // North America
    m.insert(20200, coord!("Maine", 44.0, -69.0));
    m.insert(20100, coord!("Rhode Island", 41.7, -71.4));
    m.insert(20101, coord!("Bristol (RI)", 41.67, -71.27));
    m.insert(20103, coord!("Newport", 41.49, -71.31));
    m.insert(20105, coord!("Providence", 41.82, -71.41));
    m.insert(20300, coord!("New Hampshire", 43.2, -71.5));
    m.insert(20400, coord!("Massachusetts", 42.4, -71.0));
    m.insert(20404, coord!("Boston", 42.36, -71.06));
    m.insert(20417, coord!("Salem", 42.52, -70.90));
    m.insert(20415, coord!("New Bedford", 41.64, -70.93));
    m.insert(20500, coord!("Connecticut", 41.6, -72.7));
    m.insert(20504, coord!("New Haven", 41.31, -72.93));
    m.insert(20505, coord!("New London", 41.36, -72.10));
    m.insert(20600, coord!("New York", 40.7, -74.0));
    m.insert(20700, coord!("Delaware", 39.2, -75.5));
    m.insert(20800, coord!("New Jersey", 40.2, -74.8));
    m.insert(20900, coord!("Pennsylvania", 40.0, -75.2));
    m.insert(20902, coord!("Philadelphia", 39.95, -75.16));
    m.insert(21000, coord!("Maryland", 39.0, -76.6));
    m.insert(21001, coord!("Annapolis", 38.97, -76.49));
    m.insert(21002, coord!("Baltimore", 39.29, -76.61));
    m.insert(21006, coord!("Patuxent", 38.45, -76.55));
    m.insert(21100, coord!("Virginia", 37.5, -77.4));
    m.insert(21102, coord!("Hampton", 37.03, -76.35));
    m.insert(21108, coord!("South Potomac", 38.00, -76.80));
    m.insert(21111, coord!("York River", 37.24, -76.51));
    m.insert(21104, coord!("Norfolk", 36.85, -76.29));
    m.insert(21107, coord!("Rappahannock", 37.60, -76.31));
    m.insert(21109, coord!("Upper James River", 37.53, -77.43));
    m.insert(21200, coord!("North Carolina", 35.8, -78.6));
    m.insert(21300, coord!("South Carolina", 32.8, -79.9));
    m.insert(21301, coord!("Beaufort", 32.43, -80.67));
    m.insert(21302, coord!("Charleston", 32.78, -79.93));
    m.insert(21400, coord!("Georgia", 31.5, -81.1));
    m.insert(21401, coord!("Savannah", 32.08, -81.09));
    m.insert(21500, coord!("Florida", 28.5, -81.4));
    m.insert(21501, coord!("St. Augustine", 29.89, -81.31));
    m.insert(21502, coord!("Pensacola", 30.43, -87.21));
    m.insert(21504, coord!("Key West", 24.6, -81.8));
    m.insert(21600, coord!("Gulf Coast", 30.0, -89.5));
    m.insert(21601, coord!("New Orleans", 29.95, -90.07));
    m.insert(21604, coord!("Biloxi", 30.40, -88.89));
    m.insert(21606, coord!("Mobile", 30.69, -88.04));
    m.insert(21700, coord!("Canada", 46.8, -71.2));
    m.insert(21701, coord!("Quebec", 46.81, -71.21));
    m.insert(21706, coord!("Halifax", 44.65, -63.57));
    m.insert(21800, coord!("Bermuda", 32.3, -64.8));
    m.insert(21900, coord!("Other North America", 38.0, -85.0));
    m.insert(22000, coord!("Other North America", 36.0, -87.0));
    m.insert(22100, coord!("Americas, unspecified", 20.0, -75.0));

    // Spanish Caribbean
    m.insert(31100, coord!("Hispaniola", 19.0, -70.0));
    m.insert(31106, coord!("Santo Domingo", 18.47, -69.89));
    m.insert(31200, coord!("Puerto Rico", 18.2, -66.5));
    m.insert(31201, coord!("San Juan", 18.47, -66.12));
    m.insert(31300, coord!("Cuba", 22.0, -79.5));
    m.insert(31301, coord!("Bahia Honda", 22.90, -83.16));
    m.insert(31303, coord!("Cabanas", 22.97, -82.93));
    m.insert(31306, coord!("Cardenas", 23.04, -81.20));
    m.insert(31307, coord!("Cienfuegos", 22.15, -80.44));
    m.insert(31312, coord!("Havana", 23.13, -82.38));
    m.insert(31316, coord!("Matanzas", 23.05, -81.58));
    m.insert(31319, coord!("Nuevitas", 21.55, -77.27));
    m.insert(31323, coord!("Santiago de Cuba", 20.02, -75.82));
    m.insert(31324, coord!("Trinidad de Cuba", 21.80, -79.98));
    m.insert(31327, coord!("Baracoa", 20.35, -74.51));
    m.insert(31308, coord!("Cuba, south coast", 21.70, -79.95));
    m.insert(31309, coord!("Cuba, west coast", 22.00, -84.00));
    m.insert(31313, coord!("Isla de Pinas", 21.61, -82.95));
    m.insert(31315, coord!("Mariel", 22.99, -82.75));
    m.insert(31320, coord!("Puerto Padre", 21.20, -76.60));
    m.insert(31321, coord!("Sagua", 22.81, -79.99));
    m.insert(31326, coord!("Batabano", 22.72, -82.29));
    m.insert(31400, coord!("Other Sp. Caribbean", 19.5, -69.0));

    // Dutch
    m.insert(32100, coord!("Dutch Caribbean", 12.2, -68.5));
    m.insert(32110, coord!("Curacao", 12.17, -68.98));
    m.insert(32150, coord!("St. Eustatius", 17.48, -62.97));
    m.insert(32140, coord!("St. Maarten", 18.04, -63.07));
    m.insert(32200, coord!("Dutch Guianas", 5.8, -55.0));
    m.insert(32230, coord!("Essequibo", 6.98, -58.47));
    m.insert(32240, coord!("Suriname", 5.83, -55.17));
    m.insert(32241, coord!("Paramaribo", 5.83, -55.17));
    m.insert(32250, coord!("Berbice", 6.25, -57.53));
    m.insert(32300, coord!("Other Dutch Americas", 12.0, -68.0));

    // British Caribbean
    m.insert(33200, coord!("Tortola", 18.4, -64.6));
    m.insert(33300, coord!("Anguilla", 18.2, -63.1));
    m.insert(33400, coord!("Antigua", 17.1, -61.8));
    m.insert(33401, coord!("Saint John", 17.12, -61.85));
    m.insert(33500, coord!("St. Kitts", 17.3, -62.7));
    m.insert(33600, coord!("Nevis", 17.2, -62.6));
    m.insert(33601, coord!("Newcastle", 17.15, -62.58));
    m.insert(33700, coord!("Montserrat", 16.7, -62.2));
    m.insert(33800, coord!("Dominica", 15.4, -61.4));
    m.insert(34100, coord!("St. Lucia", 14.0, -61.0));
    m.insert(34200, coord!("Barbados", 13.2, -59.5));
    m.insert(34201, coord!("Bridgetown", 13.10, -59.61));
    m.insert(34300, coord!("St. Vincent", 13.2, -61.2));
    m.insert(34400, coord!("Grenada", 12.0, -61.7));
    m.insert(34500, coord!("Trinidad", 10.4, -61.3));
    m.insert(34600, coord!("Tobago", 11.2, -60.7));
    m.insert(34601, coord!("Zion Hill", 11.19, -60.73));
    m.insert(35100, coord!("Jamaica", 18.1, -77.3));
    m.insert(35101, coord!("Annotta Bay", 18.27, -76.45));
    m.insert(35104, coord!("Falmouth (Jam.)", 18.49, -77.66));
    m.insert(35107, coord!("Montego Bay", 18.47, -77.92));
    m.insert(35110, coord!("Port Royal", 17.94, -76.84));
    m.insert(35112, coord!("Savanna la Mar", 18.22, -78.13));
    m.insert(35114, coord!("Kingston", 17.97, -76.79));
    m.insert(35200, coord!("Bahamas", 25.0, -77.4));
    m.insert(35203, coord!("Nassau", 25.06, -77.35));
    m.insert(35300, coord!("British Guiana", 6.8, -58.2));
    m.insert(35304, coord!("Demerara", 6.80, -58.16));
    m.insert(35400, coord!("Honduras", 15.5, -88.0));
    m.insert(35500, coord!("Other Br. Caribbean", 15.0, -63.0));

    // French
    m.insert(36100, coord!("Martinique", 14.6, -61.0));
    m.insert(36101, coord!("Fort-Royal", 14.60, -61.07));
    m.insert(36102, coord!("Saint-Pierre", 14.75, -61.18));
    m.insert(36200, coord!("Guadeloupe", 16.2, -61.6));
    m.insert(36201, coord!("Basse-Terre", 15.99, -61.73));
    m.insert(36202, coord!("Pointe-a-Pitre", 16.24, -61.53));
    m.insert(36300, coord!("French Guiana", 4.9, -52.3));
    m.insert(36301, coord!("Cayenne", 4.94, -52.33));
    m.insert(36400, coord!("Saint-Domingue", 19.8, -72.3));
    m.insert(36403, coord!("Cap Francais", 19.76, -72.20));
    m.insert(36404, coord!("Cayes (Les)", 18.19, -73.75));
    m.insert(36407, coord!("Jacmel", 18.23, -72.53));
    m.insert(36408, coord!("Jeremie", 18.65, -74.11));
    m.insert(36409, coord!("Leogane", 18.51, -72.63));
    m.insert(36412, coord!("Port-au-Prince", 18.54, -72.34));
    m.insert(36413, coord!("Port-de-Paix", 19.94, -72.83));
    m.insert(36405, coord!("Cul-de-Sac", 18.56, -72.31));
    m.insert(36411, coord!("Petit-Goave", 18.43, -72.87));
    m.insert(36414, coord!("Saint-Marc", 19.11, -72.70));
    m.insert(36500, coord!("Other French Caribbean", 16.0, -62.0));

    // Danish West Indies
    m.insert(37000, coord!("Danish W. Indies", 17.7, -64.8));
    m.insert(37010, coord!("St. Croix", 17.73, -64.75));
    m.insert(37020, coord!("St. Thomas", 18.34, -64.93));

    // Other Caribbean
    m.insert(38100, coord!("St. Barthelemy", 17.9, -62.8));
    m.insert(39000, coord!("Other Caribbean", 16.0, -63.0));

    // Spanish mainland
    m.insert(41200, coord!("Sp. Circum-Caribbean", 10.5, -75.0));
    m.insert(41201, coord!("New Spain", 19.40, -96.00));
    m.insert(41202, coord!("Campeche", 19.85, -90.53));
    m.insert(41203, coord!("Veracruz", 19.19, -96.13));
    m.insert(41204, coord!("Guatemala", 13.92, -90.72));
    m.insert(41206, coord!("Portobelo", 9.55, -79.65));
    m.insert(41207, coord!("Cartagena", 10.39, -75.51));
    m.insert(41208, coord!("Santa Marta", 11.25, -74.20));
    m.insert(41210, coord!("Venezuela", 10.50, -66.92));
    m.insert(41211, coord!("Borburata", 10.47, -68.13));
    m.insert(41212, coord!("Caracas", 10.49, -66.88));
    m.insert(41213, coord!("Cumana", 10.46, -64.17));
    m.insert(41214, coord!("La Guaira", 10.60, -66.93));
    m.insert(41215, coord!("Margarita", 11.00, -63.87));
    m.insert(41217, coord!("Rio de la Hacha", 11.54, -72.91));

    // Rio de la Plata
    m.insert(42000, coord!("Rio de la Plata", -34.6, -58.4));
    m.insert(42001, coord!("Buenos Aires", -34.60, -58.38));
    m.insert(42002, coord!("Montevideo", -34.91, -56.19));
    m.insert(42003, coord!("Colonia de Sacramento", -34.47, -57.84));

    // Peru
    m.insert(43000, coord!("Peru", -12.0, -77.0));
    m.insert(43001, coord!("Lima", -12.05, -77.04));

    // Chile
    m.insert(44000, coord!("Chile", -33.4, -70.6));
    m.insert(44001, coord!("Valparaiso", -33.05, -71.62));
    m.insert(44002, coord!("Talcahuano", -36.72, -73.12));

    // Ecuador
    m.insert(45000, coord!("Ecuador", -1.0, -78.5));

    // Pacific Central America
    m.insert(46000, coord!("Pacific Central America", 9.0, -79.5));
    m.insert(46001, coord!("Panama City", 8.97, -79.53));

    // Brazil — Amazonia
    m.insert(50100, coord!("Amazonia", -2.0, -49.0));
    m.insert(50102, coord!("Maranhao", -2.53, -44.28));
    m.insert(50109, coord!("São Luis", -2.50, -44.28));
    m.insert(50103, coord!("Para", -1.46, -48.50));

    // Brazil — Bahia
    m.insert(50200, coord!("Bahia", -13.0, -38.5));
    m.insert(50201, coord!("Porto Seguro", -16.45, -39.07));
    m.insert(50202, coord!("Rio Real", -11.48, -37.58));
    m.insert(50204, coord!("Taipu", -12.08, -38.05));
    m.insert(50299, coord!("Bahia, port unspecified", -12.97, -38.51));

    // Brazil — Pernambuco
    m.insert(50300, coord!("Pernambuco", -8.0, -35.0));
    m.insert(50303, coord!("Maceio", -9.67, -35.74));
    m.insert(50304, coord!("Catuamo and Maria Farinha", -7.83, -34.83));
    m.insert(50306, coord!("Paraiba", -7.12, -34.86));
    m.insert(50399, coord!("Pernambuco, port unspecified", -8.05, -34.87));

    // Brazil — SE Brazil
    m.insert(50400, coord!("SE Brazil", -23.0, -43.0));
    m.insert(50405, coord!("Cabo Frio", -22.88, -42.02));
    m.insert(50406, coord!("Campos", -21.75, -41.32));
    m.insert(50412, coord!("Ilha Grande", -23.15, -44.23));
    m.insert(50416, coord!("Macae", -22.37, -41.79));
    m.insert(50419, coord!("Paranagua", -25.52, -48.51));
    m.insert(50421, coord!("Ponta Negro", -22.90, -43.10));
    m.insert(50422, coord!("Rio de Janeiro", -22.91, -43.17));
    m.insert(50423, coord!("Rio de Janeiro province", -22.90, -43.20));
    m.insert(50424, coord!("RJ/SP/SC composite", -23.00, -43.00));
    m.insert(50428, coord!("Santos", -23.96, -46.33));
    m.insert(50430, coord!("Sao Sebastiao", -23.79, -45.41));
    m.insert(50431, coord!("Santa Catarina", -27.59, -48.55));
    m.insert(50434, coord!("Vitoria", -20.32, -40.34));

    // Brazil — Other
    m.insert(50500, coord!("Other Brazil", -15.0, -47.0));

    // Unspecified
    m.insert(80200, coord!("Americas, unspec.", 15.0, -65.0));
    m.insert(80100, coord!("East Indies", -6.0, 110.0));
    m.insert(80300, coord!("Asia e Africa", 10.0, 50.0));
    m.insert(80400, coord!("Spanish Americas", 10.0, -74.0));
    m.insert(80499, coord!("Spanish Americas, unspec.", 10.0, -74.0));
    m.insert(80500, coord!("British Americas", 30.0, -78.0));
    m.insert(80600, coord!("India", 20.0, 78.0));
    m.insert(80700, coord!("Non-geographical", 10.0, -50.0));
    m.insert(80713, coord!("Rescued at sea (shipwreck)", 10.0, -50.0));

    // ── European ─────────────────────────────────────────────────────────
    m.insert(10100, coord!("Spain", 39.5, -3.7));
    m.insert(10200, coord!("Portugal", 39.4, -8.2));
    m.insert(10300, coord!("Great Britain", 54.5, -2.0));
    m.insert(10400, coord!("England", 51.5, -2.6));
    m.insert(10500, coord!("Scotland", 56.5, -4.0));
    m.insert(10700, coord!("France", 47.2, 2.2));
    m.insert(10800, coord!("Netherlands", 52.0, 5.3));
    m.insert(10600, coord!("Ireland", 53.0, -8.0));
    m.insert(10900, coord!("Denmark", 55.7, 12.6));
    m.insert(11000, coord!("Northern Germany", 53.5, 10.0));
    m.insert(11100, coord!("Belgium", 50.8, 4.4));
    m.insert(11200, coord!("Norway", 60.0, 10.7));
    m.insert(11300, coord!("Sweden", 59.3, 18.1));
    m.insert(11400, coord!("Italy", 42.5, 12.5));
    m.insert(11500, coord!("Russia", 59.9, 30.3));
    m.insert(11600, coord!("Mediterranean", 38.0, 15.0));

    // European ports
    // Spain
    m.insert(10104, coord!("Cadiz", 36.53, -6.29));
    m.insert(10107, coord!("La Coruna", 43.37, -8.40));
    m.insert(10111, coord!("Seville", 37.39, -5.99));
    m.insert(10113, coord!("Vigo", 42.24, -8.72));
    // Portugal
    m.insert(10202, coord!("Lagos (Portugal)", 37.10, -8.67));
    m.insert(10203, coord!("Lisbon", 38.72, -9.14));
    m.insert(10204, coord!("Oporto", 41.15, -8.61));
    m.insert(10299, coord!("Portugal unspec.", 39.4, -8.2));
    // England
    m.insert(10404, coord!("Bristol", 51.45, -2.58));
    m.insert(10432, coord!("Liverpool", 53.41, -2.98));
    m.insert(10433, coord!("London", 51.51, -0.08));
    m.insert(10447, coord!("Plymouth", 50.37, -4.14));
    m.insert(10480, coord!("Falmouth (Eng.)", 50.15, -5.07));
    m.insert(10430, coord!("Lancaster", 54.05, -2.80));
    m.insert(10474, coord!("Whitehaven", 54.55, -3.59));
    // Scotland
    m.insert(10504, coord!("Glasgow", 55.86, -4.25));
    // France
    m.insert(10703, coord!("Bordeaux", 44.84, -0.57));
    m.insert(10710, coord!("Honfleur", 49.42, 0.23));
    m.insert(10712, coord!("La Rochelle", 46.16, -1.15));
    m.insert(10713, coord!("Le Havre", 49.49, 0.11));
    m.insert(10720, coord!("Nantes", 47.22, -1.55));
    m.insert(10726, coord!("Saint-Malo", 48.65, -2.00));
    m.insert(10715, coord!("Lorient", 47.75, -3.37));
    m.insert(10799, coord!("France unspec.", 47.2, 2.2));
    // Netherlands
    m.insert(10810, coord!("Amsterdam", 52.37, 4.89));
    m.insert(10830, coord!("Rotterdam", 51.92, 4.48));
    m.insert(10850, coord!("Middelburg", 51.50, 3.61));
    m.insert(10851, coord!("Vlissingen", 51.44, 3.57));
    // Denmark
    m.insert(10901, coord!("Copenhagen", 55.68, 12.57));
    // Germany
    m.insert(11004, coord!("Hamburg", 53.55, 9.99));

    m
}

/// Build static route waypoints map (ported from routeWaypoints in coords.js)
fn build_static_waypoints() -> HashMap<i64, Vec<LatLng>> {
    let mut m = HashMap::new();

    // Southeast Africa -> around Cape, past St. Helena toward Brazil
    m.insert(
        60800,
        vec![
            LatLng {
                lat: -34.5,
                lng: 26.0,
            }, // Cape Agulhas
            LatLng {
                lat: -33.0,
                lng: 10.0,
            }, // West of Cape
            LatLng {
                lat: -15.97,
                lng: -5.71,
            }, // St. Helena
            LatLng {
                lat: -10.0,
                lng: -20.0,
            }, // Mid-Atlantic approach
        ],
    );

    // West Central Africa -> offshore via St. Helena
    m.insert(
        60700,
        vec![
            LatLng {
                lat: -12.0,
                lng: 5.0,
            }, // Offshore West Central Africa
            LatLng {
                lat: -15.97,
                lng: -5.71,
            }, // St. Helena
        ],
    );

    // Other Africa (inland) -> route to West African coast first
    m.insert(
        60900,
        vec![
            LatLng {
                lat: -2.0,
                lng: 8.0,
            }, // West coast approach
            LatLng {
                lat: -5.0,
                lng: 5.0,
            }, // Gulf of Guinea offshore
        ],
    );

    // Bight of Benin -> West Africa offshore, Windward Islands approach
    m.insert(
        60500,
        vec![
            LatLng {
                lat: 6.0,
                lng: -5.0,
            }, // Offshore West Africa
            LatLng {
                lat: 13.0,
                lng: -40.0,
            }, // Mid-Atlantic toward Windward Islands
        ],
    );

    // Bight of Biafra -> West Africa offshore, Windward Islands approach
    m.insert(
        60600,
        vec![
            LatLng { lat: 4.0, lng: 2.0 }, // Offshore West Africa
            LatLng {
                lat: 10.0,
                lng: -35.0,
            }, // Mid-Atlantic
        ],
    );

    m
}

/// Look up a coordinate by code, falling back to parent region (floor(code/100)*100).
/// Available for direct Rust use; the WASM path exposes coords via `window.__coords` JSON.
#[allow(dead_code)]
pub fn get_coord(code: i64, map: &HashMap<i64, Coord>) -> Option<&Coord> {
    if let Some(c) = map.get(&code) {
        return Some(c);
    }
    let parent = (code / 100) * 100;
    map.get(&parent)
}
