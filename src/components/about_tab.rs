//! Static "About" documentation content.

use dioxus::prelude::*;

/// Renders the About tab with usage instructions, data interpretation, and source info.
#[component]
pub fn AboutTab() -> Element {
    rsx! {
        div { class: "about-container",
            div { class: "about-section",
                h2 { "How to Use This Tool" }
                div { class: "about-body",
                    h3 { "Dataset Modes" }
                    p {
                        strong { "Transatlantic" }
                        " \u{2014} Forced transport from Africa to the Americas. 36,108 documented voyages, 1514\u{2013}1866."
                    }
                    p {
                        strong { "Inter-American" }
                        " \u{2014} Redistribution and sale of enslaved people within the Americas. 11,521 voyages, 1550\u{2013}1841."
                    }
                    p {
                        strong { "Combined" }
                        " \u{2014} Both datasets overlaid. Reveals how destinations received enslaved people from both Africa directly and via redistribution from other American colonies."
                    }

                    h3 { "Regions vs. Ports" }
                    p { "Regions aggregate multiple ports into broader geographic areas. Ports show individual landing/departure points. Region mode is better for seeing macro patterns; port mode reveals specific trade routes." }

                    h3 { "Reading the Map" }
                    p { "Stems = individual origin flows (thickness = volume, color = origin region). Trunk = merged flow to destination. Arrowhead = destination. Warm colors = African origins (transatlantic), cool blue-teal = American origins (inter-American)." }

                    h3 { "Time Slider" }
                    p { "Filters all data to a specific time window. Useful for examining how trade patterns shifted over centuries." }

                    h3 { "Flow Detail Pane" }
                    p { "Shows the breakdown of where enslaved people came from for the selected destination, with volume counts and percentages. In combined mode, origins are grouped by dataset." }

                    h3 { "Voyage Details" }
                    p { "Individual voyage records with ship names, captains, multi-stop embarkation/disembarkation data. Click a row to expand per-stop details. I-Am voyages show linked TASTD voyage IDs when available." }
                }
            }

            div { class: "about-section",
                h2 { "Data Interpretation" }
                table { class: "interpretation-table",
                    thead {
                        tr {
                            th { "Concept" }
                            th { "What the Tool Shows" }
                            th { "What This Means" }
                            th { "Caveats" }
                        }
                    }
                    tbody {
                        tr {
                            td { "Principal region of purchase" }
                            td { "Single origin stem per embarkation region" }
                            td { "The imputed primary region where enslaved people were purchased for this voyage" }
                            td { "Many voyages stopped at 2\u{2013}3 ports. The \"principal\" region is an imputation\u{2014}it represents the most likely primary source, not necessarily the only one." }
                        }
                        tr {
                            td { "Principal port of disembarkation" }
                            td { "Single destination point on the map" }
                            td { "The imputed primary port where enslaved people were first landed" }
                            td { "Some voyages disembarked at multiple ports. The tool shows the principal disembarkation point." }
                        }
                        tr {
                            td { "Multi-stop embarkation" }
                            td { "Visible in voyage detail expansion" }
                            td { "A single voyage often purchased enslaved people at multiple ports" }
                            td { "The stem visualization aggregates to the principal region. Expand individual voyages to see all stops." }
                        }
                        tr {
                            td { "Embarked vs. disembarked" }
                            td { "\"Enslaved persons\" volume metric" }
                            td { "Embarked = number who boarded. Disembarked = number who arrived alive." }
                            td { "Transatlantic data primarily uses embarked counts. Inter-American data primarily uses disembarked counts. The difference reflects mortality during the Middle Passage." }
                        }
                        tr {
                            td { "Inter-American redistribution" }
                            td { "Cool-toned stems in I-Am and Combined modes" }
                            td { "Enslaved people were frequently resold and transported between American colonies after initial arrival" }
                            td { "An enslaved person arriving in Jamaica via the transatlantic trade might later be shipped to Cuba. The voyageID2 field links these journeys when known." }
                        }
                        tr {
                            td { "Combined mode overlaps" }
                            td { "Two trunk groups (warm + cool) at same destination" }
                            td { "A destination like Cuba received enslaved people directly from Africa AND via redistribution from other colonies" }
                            td { "The two trade systems operated simultaneously. Combined mode reveals the full scope of forced migration." }
                        }
                        tr {
                            td { "Imputed data" }
                            td { "All volume numbers" }
                            td { "Many voyage records have incomplete data. Scholars use statistical imputation to estimate missing values." }
                            td { "Numbers (SLAXIMP, SLAMIMP) are best estimates, not precise counts. They are the scholarly standard." }
                        }
                        tr {
                            td { "Year of arrival" }
                            td { "Time slider filtering" }
                            td { "YEARAM = the imputed year enslaved people arrived at their destination" }
                            td { "Some voyages span 2+ years. The year shown is arrival, not departure." }
                        }
                    }
                }
            }

            div { class: "about-section",
                h2 { "About the Data Sources" }
                div { class: "about-body",
                    p {
                        strong { "Trans-Atlantic Slave Trade Database (TASTD)" }
                        " \u{2014} SlaveVoyages.org. 36,108 documented voyages, 274 variables per voyage. Covers 1514\u{2013}1866."
                    }
                    p {
                        strong { "Intra-American Slave Trade Database (IAMSTD)" }
                        " \u{2014} SlaveVoyages.org. 11,521 documented voyages, 169 variables per voyage. Covers 1550\u{2013}1841."
                    }
                    p {
                        strong { "Linking" }
                        " \u{2014} Some inter-American voyages are linked to their originating transatlantic voyage via voyageID2, enabling researchers to trace the full forced migration chain."
                    }
                    p {
                        strong { "Geographic coding" }
                        " \u{2014} Both databases use a shared 5-digit hierarchical code system. First 2 digits = broad region, next digits = specific port/place."
                    }
                    p { class: "citation",
                        "Data from SlaveVoyages.org. David Eltis, et al., "
                        em { "Trans-Atlantic Slave Trade Database" }
                        " and "
                        em { "Intra-American Slave Trade Database" }
                        "."
                    }
                }
            }
        }
    }
}
