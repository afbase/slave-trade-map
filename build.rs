use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=tastdb-exp-2020-raw.csv");
    println!("cargo::rerun-if-changed=tastdb-exp-2020-value-labels.csv");
    println!("cargo::rerun-if-changed=I-Am1.0.csv");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // --- Parse value labels for our key variables ---
    let labels_data = fs::read_to_string("tastdb-exp-2020-value-labels.csv")
        .expect("Cannot read value-labels CSV");
    let mut labels_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(labels_data.as_bytes());

    let mut labels_out = String::from("variable,code,label\n");
    let needed_vars = [
        "MAJBYIMP",
        "MJBYPTIMP",
        "MJSELIMP",
        "MJSLPTIMP",
        "PLAC1TRA",
        "PLAC2TRA",
        "PLAC3TRA",
        "SLA1PORT",
        "ADPSALE1",
        "ADPSALE2",
        "EMBPORT",
        "EMBPORT2",
        "ARRPORT",
        "ARRPORT2",
        "MAJBUYPT",
        "MAJSELPT",
        "DEPTREGIMP",
        "DEPTREGIMP1",
        "REGARR",
        "REGARR2",
        "REGDIS1",
        "REGDIS2",
        "REGDIS3",
        "REGEM1",
        "REGEM2",
        "REGEM3",
    ];

    // Also collect deduplicated geo labels (code -> label) across all geo variables
    let mut geo_labels: HashMap<i64, String> = HashMap::new();

    for result in labels_reader.records() {
        let record = result.expect("bad label record");
        let variable = record.get(0).unwrap_or("").trim();
        if needed_vars.contains(&variable) {
            let code_str = record.get(1).unwrap_or("").trim();
            let label = record.get(2).unwrap_or("").trim();
            // Parse float code to integer
            if let Ok(code_f) = code_str.parse::<f64>() {
                let code_i = code_f as i64;
                // Escape label for CSV
                if label.contains(',') || label.contains('"') {
                    labels_out.push_str(&format!(
                        "{},{},\"{}\"\n",
                        variable,
                        code_i,
                        label.replace('"', "\"\"")
                    ));
                } else {
                    labels_out.push_str(&format!("{},{},{}\n", variable, code_i, label));
                }
                // Collect into geo_labels (first label wins for a given code)
                geo_labels
                    .entry(code_i)
                    .or_insert_with(|| label.to_string());
            }
        }
    }

    // Add I-Am specific region labels not in TASTD
    let iam_extra_labels: &[(i64, &str)] = &[
        (21800, "Bermuda"),
        (22000, "Other North America"),
        (22100, "Americas, unspecified"),
        (44000, "Tierra Firme"),
        (80400, "Africa, port unspec."),
        (80700, "Other"),
    ];
    for &(code, label) in iam_extra_labels {
        geo_labels.entry(code).or_insert_with(|| label.to_string());
        // Add to labels for MAJBYIMP and MJSELIMP
        for var in &["MAJBYIMP", "MJSELIMP"] {
            labels_out.push_str(&format!("{},{},{}\n", var, code, csv_escape(label)));
        }
    }

    fs::write(out_path.join("labels.csv"), &labels_out).expect("Cannot write labels.csv");

    // Write geo_labels.csv — deduplicated code,label pairs for universal port/place lookups
    let mut geo_out = String::from("code,label\n");
    let mut geo_keys: Vec<i64> = geo_labels.keys().cloned().collect();
    geo_keys.sort();
    for code in geo_keys {
        let label = &geo_labels[&code];
        if label.contains(',') || label.contains('"') {
            geo_out.push_str(&format!("{},\"{}\"\n", code, label.replace('"', "\"\"")));
        } else {
            geo_out.push_str(&format!("{},{}\n", code, label));
        }
    }
    fs::write(out_path.join("geo_labels.csv"), &geo_out).expect("Cannot write geo_labels.csv");

    // --- Helper functions ---
    fn csv_escape(s: &str) -> String {
        let s = s.trim();
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    fn get_str(record: &csv::StringRecord, idx: usize) -> &str {
        record.get(idx).unwrap_or("").trim()
    }

    fn parse_code(s: &str) -> Option<i64> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        s.parse::<f64>().ok().map(|f| f as i64)
    }

    fn parse_float(s: &str) -> f64 {
        let s = s.trim();
        if s.is_empty() {
            return 0.0;
        }
        s.parse::<f64>().unwrap_or(0.0)
    }

    #[derive(Default)]
    struct Agg {
        embarked: f64,
        disembarked: f64,
        voyages: i64,
    }

    // ===== TRANSATLANTIC (TASTD) =====
    let raw_data = fs::read_to_string("tastdb-exp-2020-raw.csv").expect("Cannot read raw CSV");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(raw_data.as_bytes());

    let headers = reader.headers().expect("no headers").clone();
    let col = |name: &str| -> Option<usize> { headers.iter().position(|h| h == name) };
    let idx_majbyimp = col("MAJBYIMP").expect("MAJBYIMP not found");
    let idx_mjbyptimp = col("MJBYPTIMP").expect("MJBYPTIMP not found");
    let idx_mjselimp = col("MJSELIMP").expect("MJSELIMP not found");
    let idx_mjslptimp = col("MJSLPTIMP").expect("MJSLPTIMP not found");
    let idx_slaximp = col("SLAXIMP").expect("SLAXIMP not found");
    let idx_slamimp = col("SLAMIMP").expect("SLAMIMP not found");
    let idx_yearam = col("YEARAM").expect("YEARAM not found");
    let idx_voyageid = col("VOYAGEID").expect("VOYAGEID not found");
    let idx_shipname = col("SHIPNAME").expect("SHIPNAME not found");
    let idx_captaina = col("CAPTAINA").expect("CAPTAINA not found");
    let idx_captainb = col("CAPTAINB").expect("CAPTAINB not found");
    let idx_captainc = col("CAPTAINC").expect("CAPTAINC not found");
    let idx_vymrtimp = col("VYMRTIMP").expect("VYMRTIMP not found");
    let idx_plac1tra = col("PLAC1TRA").expect("PLAC1TRA not found");
    let idx_plac2tra = col("PLAC2TRA").expect("PLAC2TRA not found");
    let idx_plac3tra = col("PLAC3TRA").expect("PLAC3TRA not found");
    let idx_ncar13 = col("NCAR13").expect("NCAR13 not found");
    let idx_ncar15 = col("NCAR15").expect("NCAR15 not found");
    let idx_ncar17 = col("NCAR17").expect("NCAR17 not found");
    let idx_sla1port = col("SLA1PORT").expect("SLA1PORT not found");
    let idx_adpsale1 = col("ADPSALE1").expect("ADPSALE1 not found");
    let idx_adpsale2 = col("ADPSALE2").expect("ADPSALE2 not found");
    let idx_slas32 = col("SLAS32").expect("SLAS32 not found");
    let idx_slas36 = col("SLAS36").expect("SLAS36 not found");
    let idx_slas39 = col("SLAS39").expect("SLAS39 not found");

    let mut region_flows: HashMap<(i64, i64, i64), Agg> = HashMap::new();
    let mut port_flows: HashMap<(i64, i64, i64), Agg> = HashMap::new();

    // Port-level origins: (embark_port, disembark_region, disembark_port, year).
    // embark_port is MJBYPTIMP, the imputed principal port of purchase; its
    // region (port / 100 * 100) always equals MAJBYIMP. 0 stands for "missing"
    // on the destination side.
    let mut origin_flows: HashMap<(i64, i64, i64, i64), Agg> = HashMap::new();

    let mut voyages_csv = String::from(
        "voyage_id,ship_name,captain_a,captain_b,captain_c,\
         embark_region,disembark_region,disembark_port,year_arrived,\
         embarked,disembarked,deaths,\
         embark_port1,embark_port2,embark_port3,\
         embarked_port1,embarked_port2,embarked_port3,\
         disembark_place1,disembark_place2,disembark_place3,\
         disembarked_place1,disembarked_place2,disembarked_place3\n",
    );

    for result in reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        let embark_region = match parse_code(record.get(idx_majbyimp).unwrap_or("")) {
            Some(c) => c,
            None => continue,
        };
        let disembark_region = parse_code(record.get(idx_mjselimp).unwrap_or("")).map(|c| {
            // Normalize port-like codes to parent region (e.g. 50311 → 50300)
            if c % 100 != 0 {
                (c / 100) * 100
            } else {
                c
            }
        });
        let disembark_port = parse_code(record.get(idx_mjslptimp).unwrap_or(""));
        let year = match parse_code(record.get(idx_yearam).unwrap_or("")) {
            Some(y) => y,
            None => continue,
        };
        let embarked = parse_float(record.get(idx_slaximp).unwrap_or(""));
        let disembarked = parse_float(record.get(idx_slamimp).unwrap_or(""));

        if let Some(dr) = disembark_region {
            let entry = region_flows.entry((embark_region, dr, year)).or_default();
            entry.embarked += embarked;
            entry.disembarked += disembarked;
            entry.voyages += 1;
        }

        if let Some(dp) = disembark_port {
            let entry = port_flows.entry((embark_region, dp, year)).or_default();
            entry.embarked += embarked;
            entry.disembarked += disembarked;
            entry.voyages += 1;
        }

        if let Some(ep) = parse_code(record.get(idx_mjbyptimp).unwrap_or("")) {
            let entry = origin_flows
                .entry((
                    ep,
                    disembark_region.unwrap_or(0),
                    disembark_port.unwrap_or(0),
                    year,
                ))
                .or_default();
            entry.embarked += embarked;
            entry.disembarked += disembarked;
            entry.voyages += 1;
        }

        let voyage_id = parse_code(get_str(&record, idx_voyageid)).unwrap_or(0);
        let ship_name = csv_escape(get_str(&record, idx_shipname));
        let captain_a = csv_escape(get_str(&record, idx_captaina));
        let captain_b = csv_escape(get_str(&record, idx_captainb));
        let captain_c = csv_escape(get_str(&record, idx_captainc));
        let dr_str = disembark_region.map(|v| v.to_string()).unwrap_or_default();
        let dp_str = disembark_port.map(|v| v.to_string()).unwrap_or_default();
        let deaths_str = parse_code(get_str(&record, idx_vymrtimp))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let embarked_i = if embarked > 0.0 {
            format!("{}", embarked as i64)
        } else {
            String::new()
        };
        let disembarked_i = if disembarked > 0.0 {
            format!("{}", disembarked as i64)
        } else {
            String::new()
        };

        let ep1 = parse_code(get_str(&record, idx_plac1tra))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ep2 = parse_code(get_str(&record, idx_plac2tra))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ep3 = parse_code(get_str(&record, idx_plac3tra))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ne1 = parse_code(get_str(&record, idx_ncar13))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ne2 = parse_code(get_str(&record, idx_ncar15))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ne3 = parse_code(get_str(&record, idx_ncar17))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let dp1 = parse_code(get_str(&record, idx_sla1port))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let dp2 = parse_code(get_str(&record, idx_adpsale1))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let dp3 = parse_code(get_str(&record, idx_adpsale2))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd1 = parse_code(get_str(&record, idx_slas32))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd2 = parse_code(get_str(&record, idx_slas36))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd3 = parse_code(get_str(&record, idx_slas39))
            .map(|v| v.to_string())
            .unwrap_or_default();

        voyages_csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            voyage_id,
            ship_name,
            captain_a,
            captain_b,
            captain_c,
            embark_region,
            dr_str,
            dp_str,
            year,
            embarked_i,
            disembarked_i,
            deaths_str,
            ep1,
            ep2,
            ep3,
            ne1,
            ne2,
            ne3,
            dp1,
            dp2,
            dp3,
            nd1,
            nd2,
            nd3,
        ));
    }

    // Write TASTD files
    let mut region_csv =
        String::from("embark_region,disembark_region,year,embarked,disembarked,voyages\n");
    let mut region_keys: Vec<_> = region_flows.keys().cloned().collect();
    region_keys.sort();
    for key in region_keys {
        let agg = &region_flows[&key];
        region_csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            key.0, key.1, key.2, agg.embarked as i64, agg.disembarked as i64, agg.voyages
        ));
    }
    fs::write(out_path.join("region_flows.csv"), &region_csv)
        .expect("Cannot write region_flows.csv");

    let mut port_csv =
        String::from("embark_region,disembark_port,year,embarked,disembarked,voyages\n");
    let mut port_keys: Vec<_> = port_flows.keys().cloned().collect();
    port_keys.sort();
    for key in port_keys {
        let agg = &port_flows[&key];
        port_csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            key.0, key.1, key.2, agg.embarked as i64, agg.disembarked as i64, agg.voyages
        ));
    }
    fs::write(out_path.join("port_flows.csv"), &port_csv).expect("Cannot write port_flows.csv");

    fs::write(out_path.join("voyages.csv"), &voyages_csv).expect("Cannot write voyages.csv");

    let mut origin_csv = String::from(
        "embark_port,disembark_region,disembark_port,year,embarked,disembarked,voyages\n",
    );
    let mut origin_keys: Vec<_> = origin_flows.keys().cloned().collect();
    origin_keys.sort();
    let opt = |v: i64| if v == 0 { String::new() } else { v.to_string() };
    for key in origin_keys {
        let agg = &origin_flows[&key];
        origin_csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            key.0,
            opt(key.1),
            opt(key.2),
            key.3,
            agg.embarked as i64,
            agg.disembarked as i64,
            agg.voyages
        ));
    }
    fs::write(out_path.join("origin_flows.csv"), &origin_csv)
        .expect("Cannot write origin_flows.csv");

    // ===== INTER-AMERICAN (I-Am) =====
    // I-Am1.0.csv is latin-1 encoded — read as bytes and convert
    let iam_bytes = fs::read("I-Am1.0.csv").expect("Cannot read I-Am1.0.csv");
    let iam_data: String = iam_bytes.iter().map(|&b| b as char).collect();

    let mut iam_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(iam_data.as_bytes());

    let iam_headers = iam_reader.headers().expect("no I-Am headers").clone();
    let iam_col = |name: &str| -> Option<usize> { iam_headers.iter().position(|h| h == name) };

    let iam_idx_voyageid = iam_col("voyageid").expect("I-Am voyageid not found");
    let iam_idx_yearam = iam_col("yearam").expect("I-Am yearam not found");
    let iam_idx_shipname = iam_col("shipname").expect("I-Am shipname not found");
    let iam_idx_captaina = iam_col("captaina").expect("I-Am captaina not found");
    let iam_idx_captainb = iam_col("captainb").expect("I-Am captainb not found");
    let iam_idx_majbyimp = iam_col("majbyimp").expect("I-Am majbyimp not found");
    let iam_idx_mjselimp = iam_col("mjselimp").expect("I-Am mjselimp not found");
    let iam_idx_mjslptimp = iam_col("mjslptimp").expect("I-Am mjslptimp not found");
    let iam_idx_slaximp = iam_col("slaximp").expect("I-Am slaximp not found");
    let iam_idx_slamimp = iam_col("slamimp").expect("I-Am slamimp not found");
    let iam_idx_vymrtimp = iam_col("vymrtimp").expect("I-Am vymrtimp not found");
    let iam_idx_plac1tra = iam_col("plac1tra").expect("I-Am plac1tra not found");
    let iam_idx_plac2tra = iam_col("plac2tra").expect("I-Am plac2tra not found");
    let iam_idx_ncar13 = iam_col("ncar13").expect("I-Am ncar13 not found");
    let iam_idx_ncar15 = iam_col("ncar15").expect("I-Am ncar15 not found");
    let iam_idx_sla1port = iam_col("sla1port").expect("I-Am sla1port not found");
    let iam_idx_adpsale1 = iam_col("adpsale1").expect("I-Am adpsale1 not found");
    let iam_idx_adpsale2 = iam_col("adpsale2").expect("I-Am adpsale2 not found");
    let iam_idx_slas32 = iam_col("slas32").expect("I-Am slas32 not found");
    let iam_idx_slas36 = iam_col("slas36").expect("I-Am slas36 not found");
    let iam_idx_slas39 = iam_col("slas39").expect("I-Am slas39 not found");
    let iam_idx_voyageid2 = iam_col("voyageID2").expect("I-Am voyageID2 not found");

    let mut iam_region_flows: HashMap<(i64, i64, i64), Agg> = HashMap::new();
    let mut iam_port_flows: HashMap<(i64, i64, i64), Agg> = HashMap::new();

    // I-Am voyages CSV — same schema as TASTD voyages + voyage_id2 column
    let mut iam_voyages_csv = String::from(
        "voyage_id,ship_name,captain_a,captain_b,captain_c,\
         embark_region,disembark_region,disembark_port,year_arrived,\
         embarked,disembarked,deaths,\
         embark_port1,embark_port2,embark_port3,\
         embarked_port1,embarked_port2,embarked_port3,\
         disembark_place1,disembark_place2,disembark_place3,\
         disembarked_place1,disembarked_place2,disembarked_place3,\
         voyage_id2\n",
    );

    for result in iam_reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        let embark_region = match parse_code(record.get(iam_idx_majbyimp).unwrap_or("")) {
            Some(c) => c,
            None => continue,
        };
        let disembark_region = parse_code(record.get(iam_idx_mjselimp).unwrap_or("")).map(|c| {
            // Normalize port-like codes to parent region (e.g. 50311 → 50300)
            if c % 100 != 0 {
                (c / 100) * 100
            } else {
                c
            }
        });
        let disembark_port = parse_code(record.get(iam_idx_mjslptimp).unwrap_or(""));
        let year = match parse_code(record.get(iam_idx_yearam).unwrap_or("")) {
            Some(y) => y,
            None => continue,
        };
        let embarked = parse_float(record.get(iam_idx_slaximp).unwrap_or(""));
        let disembarked = parse_float(record.get(iam_idx_slamimp).unwrap_or(""));

        if let Some(dr) = disembark_region {
            let entry = iam_region_flows
                .entry((embark_region, dr, year))
                .or_default();
            entry.embarked += embarked;
            entry.disembarked += disembarked;
            entry.voyages += 1;
        }

        if let Some(dp) = disembark_port {
            let entry = iam_port_flows.entry((embark_region, dp, year)).or_default();
            entry.embarked += embarked;
            entry.disembarked += disembarked;
            entry.voyages += 1;
        }

        let voyage_id = parse_code(get_str(&record, iam_idx_voyageid)).unwrap_or(0);
        let ship_name = csv_escape(get_str(&record, iam_idx_shipname));
        let captain_a = csv_escape(get_str(&record, iam_idx_captaina));
        let captain_b = csv_escape(get_str(&record, iam_idx_captainb));
        let captain_c = String::new(); // I-Am has no captainc
        let dr_str = disembark_region.map(|v| v.to_string()).unwrap_or_default();
        let dp_str = disembark_port.map(|v| v.to_string()).unwrap_or_default();
        let deaths_str = parse_code(get_str(&record, iam_idx_vymrtimp))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let embarked_i = if embarked > 0.0 {
            format!("{}", embarked as i64)
        } else {
            String::new()
        };
        let disembarked_i = if disembarked > 0.0 {
            format!("{}", disembarked as i64)
        } else {
            String::new()
        };

        let ep1 = parse_code(get_str(&record, iam_idx_plac1tra))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ep2 = parse_code(get_str(&record, iam_idx_plac2tra))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ep3 = String::new(); // I-Am has no plac3tra
        let ne1 = parse_code(get_str(&record, iam_idx_ncar13))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ne2 = parse_code(get_str(&record, iam_idx_ncar15))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ne3 = String::new(); // I-Am has no ncar17
        let dp1 = parse_code(get_str(&record, iam_idx_sla1port))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let dp2 = parse_code(get_str(&record, iam_idx_adpsale1))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let dp3 = parse_code(get_str(&record, iam_idx_adpsale2))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd1 = parse_code(get_str(&record, iam_idx_slas32))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd2 = parse_code(get_str(&record, iam_idx_slas36))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let nd3 = parse_code(get_str(&record, iam_idx_slas39))
            .map(|v| v.to_string())
            .unwrap_or_default();
        let vid2 = parse_code(get_str(&record, iam_idx_voyageid2))
            .map(|v| v.to_string())
            .unwrap_or_default();

        iam_voyages_csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            voyage_id,
            ship_name,
            captain_a,
            captain_b,
            captain_c,
            embark_region,
            dr_str,
            dp_str,
            year,
            embarked_i,
            disembarked_i,
            deaths_str,
            ep1,
            ep2,
            ep3,
            ne1,
            ne2,
            ne3,
            dp1,
            dp2,
            dp3,
            nd1,
            nd2,
            nd3,
            vid2,
        ));
    }

    // Write I-Am region flows
    let mut iam_region_csv =
        String::from("embark_region,disembark_region,year,embarked,disembarked,voyages\n");
    let mut iam_region_keys: Vec<_> = iam_region_flows.keys().cloned().collect();
    iam_region_keys.sort();
    for key in iam_region_keys {
        let agg = &iam_region_flows[&key];
        iam_region_csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            key.0, key.1, key.2, agg.embarked as i64, agg.disembarked as i64, agg.voyages
        ));
    }
    fs::write(out_path.join("iam_region_flows.csv"), &iam_region_csv)
        .expect("Cannot write iam_region_flows.csv");

    // Write I-Am port flows
    let mut iam_port_csv =
        String::from("embark_region,disembark_port,year,embarked,disembarked,voyages\n");
    let mut iam_port_keys: Vec<_> = iam_port_flows.keys().cloned().collect();
    iam_port_keys.sort();
    for key in iam_port_keys {
        let agg = &iam_port_flows[&key];
        iam_port_csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            key.0, key.1, key.2, agg.embarked as i64, agg.disembarked as i64, agg.voyages
        ));
    }
    fs::write(out_path.join("iam_port_flows.csv"), &iam_port_csv)
        .expect("Cannot write iam_port_flows.csv");

    // Write I-Am voyages
    fs::write(out_path.join("iam_voyages.csv"), &iam_voyages_csv)
        .expect("Cannot write iam_voyages.csv");

    eprintln!(
        "build.rs: TASTD — {} region flows, {} port flows, {} origin-port flows | I-Am — {} region flows, {} port flows",
        region_flows.len(),
        port_flows.len(),
        origin_flows.len(),
        iam_region_flows.len(),
        iam_port_flows.len()
    );
}
