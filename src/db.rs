//! In-browser SQLite database for slave trade flow queries.
//!
//! Loads pre-aggregated CSVs (built by `build.rs`) into an in-memory SQLite
//! database at startup. Provides query methods for destinations, flows, and
//! individual voyage records across both the Transatlantic and Inter-American
//! datasets.

use crate::models::{DatasetMode, DestinationInfo, FlowData, VoyageRecord, YearlySourceFlow};
use rusqlite::{params, Connection};
use std::cell::RefCell;
use std::rc::Rc;

const REGION_FLOWS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/region_flows.csv"));
const PORT_FLOWS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/port_flows.csv"));
const LABELS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/labels.csv"));
const VOYAGES_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/voyages.csv"));
const GEO_LABELS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/geo_labels.csv"));
const IAM_REGION_FLOWS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/iam_region_flows.csv"));
const IAM_PORT_FLOWS_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/iam_port_flows.csv"));
const IAM_VOYAGES_CSV: &str = include_str!(concat!(env!("OUT_DIR"), "/iam_voyages.csv"));

/// In-memory SQLite database wrapping pre-aggregated slave trade data.
#[derive(Clone)]
pub struct Database {
    conn: Rc<RefCell<Connection>>,
}

impl Database {
    /// Create a new in-memory database and load all CSV data.
    pub fn new() -> anyhow::Result<Self> {
        let conn = Connection::open_in_memory()?;

        conn.execute_batch(
            "CREATE TABLE region_flows (
                embark_region INTEGER NOT NULL,
                disembark_region INTEGER NOT NULL,
                year INTEGER NOT NULL,
                embarked INTEGER NOT NULL,
                disembarked INTEGER NOT NULL,
                voyages INTEGER NOT NULL
            );
            CREATE INDEX idx_rf_disembark ON region_flows(disembark_region, year);
            CREATE INDEX idx_rf_year ON region_flows(year);

            CREATE TABLE port_flows (
                embark_region INTEGER NOT NULL,
                disembark_port INTEGER NOT NULL,
                year INTEGER NOT NULL,
                embarked INTEGER NOT NULL,
                disembarked INTEGER NOT NULL,
                voyages INTEGER NOT NULL
            );
            CREATE INDEX idx_pf_disembark ON port_flows(disembark_port, year);
            CREATE INDEX idx_pf_year ON port_flows(year);

            CREATE TABLE labels (
                variable TEXT NOT NULL,
                code INTEGER NOT NULL,
                label TEXT NOT NULL
            );
            CREATE INDEX idx_labels ON labels(variable, code);

            CREATE TABLE voyages (
                voyage_id INTEGER,
                ship_name TEXT,
                captain_a TEXT,
                captain_b TEXT,
                captain_c TEXT,
                embark_region INTEGER,
                disembark_region INTEGER,
                disembark_port INTEGER,
                year_arrived INTEGER,
                embarked INTEGER,
                disembarked INTEGER,
                deaths INTEGER,
                embark_port1 INTEGER,
                embark_port2 INTEGER,
                embark_port3 INTEGER,
                embarked_port1 INTEGER,
                embarked_port2 INTEGER,
                embarked_port3 INTEGER,
                disembark_place1 INTEGER,
                disembark_place2 INTEGER,
                disembark_place3 INTEGER,
                disembarked_place1 INTEGER,
                disembarked_place2 INTEGER,
                disembarked_place3 INTEGER
            );
            CREATE INDEX idx_v_dest_region ON voyages(disembark_region, year_arrived);
            CREATE INDEX idx_v_dest_port ON voyages(disembark_port, year_arrived);
            CREATE INDEX idx_v_embark_region ON voyages(embark_region);

            CREATE TABLE geo_labels (
                code INTEGER PRIMARY KEY,
                label TEXT NOT NULL
            );

            -- Inter-American tables
            CREATE TABLE iam_region_flows (
                embark_region INTEGER NOT NULL,
                disembark_region INTEGER NOT NULL,
                year INTEGER NOT NULL,
                embarked INTEGER NOT NULL,
                disembarked INTEGER NOT NULL,
                voyages INTEGER NOT NULL
            );
            CREATE INDEX idx_irf_disembark ON iam_region_flows(disembark_region, year);
            CREATE INDEX idx_irf_year ON iam_region_flows(year);

            CREATE TABLE iam_port_flows (
                embark_region INTEGER NOT NULL,
                disembark_port INTEGER NOT NULL,
                year INTEGER NOT NULL,
                embarked INTEGER NOT NULL,
                disembarked INTEGER NOT NULL,
                voyages INTEGER NOT NULL
            );
            CREATE INDEX idx_ipf_disembark ON iam_port_flows(disembark_port, year);
            CREATE INDEX idx_ipf_year ON iam_port_flows(year);

            CREATE TABLE iam_voyages (
                voyage_id INTEGER,
                ship_name TEXT,
                captain_a TEXT,
                captain_b TEXT,
                captain_c TEXT,
                embark_region INTEGER,
                disembark_region INTEGER,
                disembark_port INTEGER,
                year_arrived INTEGER,
                embarked INTEGER,
                disembarked INTEGER,
                deaths INTEGER,
                embark_port1 INTEGER,
                embark_port2 INTEGER,
                embark_port3 INTEGER,
                embarked_port1 INTEGER,
                embarked_port2 INTEGER,
                embarked_port3 INTEGER,
                disembark_place1 INTEGER,
                disembark_place2 INTEGER,
                disembark_place3 INTEGER,
                disembarked_place1 INTEGER,
                disembarked_place2 INTEGER,
                disembarked_place3 INTEGER,
                voyage_id2 INTEGER
            );
            CREATE INDEX idx_iv_dest_region ON iam_voyages(disembark_region, year_arrived);
            CREATE INDEX idx_iv_dest_port ON iam_voyages(disembark_port, year_arrived);
            CREATE INDEX idx_iv_embark_region ON iam_voyages(embark_region);",
        )?;

        let db = Self {
            conn: Rc::new(RefCell::new(conn)),
        };
        db.load_data()?;
        Ok(db)
    }

    fn load_data(&self) -> anyhow::Result<()> {
        let conn = self.conn.borrow();

        // Load region flows
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(REGION_FLOWS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT INTO region_flows VALUES (?1,?2,?3,?4,?5,?6)",
                params![
                    r.get(0).unwrap().parse::<i64>()?,
                    r.get(1).unwrap().parse::<i64>()?,
                    r.get(2).unwrap().parse::<i64>()?,
                    r.get(3).unwrap().parse::<i64>()?,
                    r.get(4).unwrap().parse::<i64>()?,
                    r.get(5).unwrap().parse::<i64>()?,
                ],
            )?;
        }

        // Load port flows
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(PORT_FLOWS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT INTO port_flows VALUES (?1,?2,?3,?4,?5,?6)",
                params![
                    r.get(0).unwrap().parse::<i64>()?,
                    r.get(1).unwrap().parse::<i64>()?,
                    r.get(2).unwrap().parse::<i64>()?,
                    r.get(3).unwrap().parse::<i64>()?,
                    r.get(4).unwrap().parse::<i64>()?,
                    r.get(5).unwrap().parse::<i64>()?,
                ],
            )?;
        }

        // Load labels
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(LABELS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT INTO labels VALUES (?1,?2,?3)",
                params![
                    r.get(0).unwrap().trim(),
                    r.get(1).unwrap().parse::<i64>()?,
                    r.get(2).unwrap().trim(),
                ],
            )?;
        }

        // Load geo_labels
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(GEO_LABELS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT OR IGNORE INTO geo_labels VALUES (?1,?2)",
                params![r.get(0).unwrap().parse::<i64>()?, r.get(1).unwrap().trim()],
            )?;
        }

        // Load voyages
        Self::load_voyages_table(&conn, VOYAGES_CSV, "voyages", false)?;

        // Load I-Am region flows
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(IAM_REGION_FLOWS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT INTO iam_region_flows VALUES (?1,?2,?3,?4,?5,?6)",
                params![
                    r.get(0).unwrap().parse::<i64>()?,
                    r.get(1).unwrap().parse::<i64>()?,
                    r.get(2).unwrap().parse::<i64>()?,
                    r.get(3).unwrap().parse::<i64>()?,
                    r.get(4).unwrap().parse::<i64>()?,
                    r.get(5).unwrap().parse::<i64>()?,
                ],
            )?;
        }

        // Load I-Am port flows
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(IAM_PORT_FLOWS_CSV.as_bytes());
        for result in rdr.records() {
            let r = result?;
            conn.execute(
                "INSERT INTO iam_port_flows VALUES (?1,?2,?3,?4,?5,?6)",
                params![
                    r.get(0).unwrap().parse::<i64>()?,
                    r.get(1).unwrap().parse::<i64>()?,
                    r.get(2).unwrap().parse::<i64>()?,
                    r.get(3).unwrap().parse::<i64>()?,
                    r.get(4).unwrap().parse::<i64>()?,
                    r.get(5).unwrap().parse::<i64>()?,
                ],
            )?;
        }

        // Load I-Am voyages
        Self::load_voyages_table(&conn, IAM_VOYAGES_CSV, "iam_voyages", true)?;

        Ok(())
    }

    fn load_voyages_table(conn: &Connection, csv_data: &str, table: &str, has_vid2: bool) -> anyhow::Result<()> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv_data.as_bytes());

        let sql = if has_vid2 {
            format!("INSERT INTO {} VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25)", table)
        } else {
            format!("INSERT INTO {} VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24)", table)
        };

        for result in rdr.records() {
            let r = result?;
            let parse_opt_i64 = |idx: usize| -> Option<i64> {
                let s = r.get(idx).unwrap_or("").trim();
                if s.is_empty() {
                    None
                } else {
                    s.parse::<f64>().ok().map(|f| f as i64)
                }
            };
            let get_str = |idx: usize| -> &str { r.get(idx).unwrap_or("").trim() };

            if has_vid2 {
                conn.execute(
                    &sql,
                    params![
                        parse_opt_i64(0),  // voyage_id
                        get_str(1),        // ship_name
                        get_str(2),        // captain_a
                        get_str(3),        // captain_b
                        get_str(4),        // captain_c
                        parse_opt_i64(5),  // embark_region
                        parse_opt_i64(6),  // disembark_region
                        parse_opt_i64(7),  // disembark_port
                        parse_opt_i64(8),  // year_arrived
                        parse_opt_i64(9),  // embarked
                        parse_opt_i64(10), // disembarked
                        parse_opt_i64(11), // deaths
                        parse_opt_i64(12), // embark_port1
                        parse_opt_i64(13), // embark_port2
                        parse_opt_i64(14), // embark_port3
                        parse_opt_i64(15), // embarked_port1
                        parse_opt_i64(16), // embarked_port2
                        parse_opt_i64(17), // embarked_port3
                        parse_opt_i64(18), // disembark_place1
                        parse_opt_i64(19), // disembark_place2
                        parse_opt_i64(20), // disembark_place3
                        parse_opt_i64(21), // disembarked_place1
                        parse_opt_i64(22), // disembarked_place2
                        parse_opt_i64(23), // disembarked_place3
                        parse_opt_i64(24), // voyage_id2
                    ],
                )?;
            } else {
                conn.execute(
                    &sql,
                    params![
                        parse_opt_i64(0),
                        get_str(1),
                        get_str(2),
                        get_str(3),
                        get_str(4),
                        parse_opt_i64(5),
                        parse_opt_i64(6),
                        parse_opt_i64(7),
                        parse_opt_i64(8),
                        parse_opt_i64(9),
                        parse_opt_i64(10),
                        parse_opt_i64(11),
                        parse_opt_i64(12),
                        parse_opt_i64(13),
                        parse_opt_i64(14),
                        parse_opt_i64(15),
                        parse_opt_i64(16),
                        parse_opt_i64(17),
                        parse_opt_i64(18),
                        parse_opt_i64(19),
                        parse_opt_i64(20),
                        parse_opt_i64(21),
                        parse_opt_i64(22),
                        parse_opt_i64(23),
                    ],
                )?;
            }
        }
        Ok(())
    }

    /// Return the (min, max) year range for the given dataset mode.
    pub fn get_year_range(&self, mode: DatasetMode) -> anyhow::Result<(i64, i64)> {
        let conn = self.conn.borrow();
        let sql = match mode {
            DatasetMode::Transatlantic => "SELECT MIN(year), MAX(year) FROM region_flows",
            DatasetMode::InterAmerican => "SELECT MIN(year), MAX(year) FROM iam_region_flows",
            DatasetMode::Combined => {
                "SELECT MIN(y), MAX(y) FROM (
                    SELECT year AS y FROM region_flows
                    UNION ALL
                    SELECT year AS y FROM iam_region_flows
                )"
            }
        };
        let mut stmt = conn.prepare(sql)?;
        let (min, max) = stmt.query_row([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })?;
        Ok((min, max))
    }

    /// List destination regions with aggregate embark/voyage counts for the given time range and dataset.
    pub fn get_destination_regions(
        &self,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<DestinationInfo>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                let mut stmt = conn.prepare(
                    "SELECT rf.disembark_region,
                            COALESCE(l.label, CAST(rf.disembark_region AS TEXT)),
                            SUM(rf.embarked),
                            SUM(rf.voyages)
                     FROM region_flows rf
                     LEFT JOIN labels l ON l.variable = 'MJSELIMP' AND l.code = rf.disembark_region
                     WHERE rf.year >= ?1 AND rf.year <= ?2
                     GROUP BY rf.disembark_region
                     ORDER BY SUM(rf.embarked) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let emb: f64 = row.get(2)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: emb,
                            voyage_count: row.get(3)?,
                            ta_embarked: emb,
                            iam_embarked: 0.0,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
            DatasetMode::InterAmerican => {
                let mut stmt = conn.prepare(
                    "SELECT rf.disembark_region,
                            COALESCE(l.label, CAST(rf.disembark_region AS TEXT)),
                            SUM(rf.embarked),
                            SUM(rf.voyages)
                     FROM iam_region_flows rf
                     LEFT JOIN labels l ON l.variable = 'MJSELIMP' AND l.code = rf.disembark_region
                     WHERE rf.year >= ?1 AND rf.year <= ?2
                     GROUP BY rf.disembark_region
                     ORDER BY SUM(rf.embarked) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let emb: f64 = row.get(2)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: emb,
                            voyage_count: row.get(3)?,
                            ta_embarked: 0.0,
                            iam_embarked: emb,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
            DatasetMode::Combined => {
                let mut stmt = conn.prepare(
                    "SELECT code, name, SUM(ta_emb), SUM(iam_emb), SUM(voys)
                     FROM (
                         SELECT rf.disembark_region AS code,
                                COALESCE(l.label, CAST(rf.disembark_region AS TEXT)) AS name,
                                SUM(rf.embarked) AS ta_emb,
                                0 AS iam_emb,
                                SUM(rf.voyages) AS voys
                         FROM region_flows rf
                         LEFT JOIN labels l ON l.variable = 'MJSELIMP' AND l.code = rf.disembark_region
                         WHERE rf.year >= ?1 AND rf.year <= ?2
                         GROUP BY rf.disembark_region
                         UNION ALL
                         SELECT rf.disembark_region AS code,
                                COALESCE(l.label, CAST(rf.disembark_region AS TEXT)) AS name,
                                0 AS ta_emb,
                                SUM(rf.embarked) AS iam_emb,
                                SUM(rf.voyages) AS voys
                         FROM iam_region_flows rf
                         LEFT JOIN labels l ON l.variable = 'MJSELIMP' AND l.code = rf.disembark_region
                         WHERE rf.year >= ?1 AND rf.year <= ?2
                         GROUP BY rf.disembark_region
                     )
                     GROUP BY code
                     ORDER BY SUM(ta_emb) + SUM(iam_emb) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let ta: f64 = row.get(2)?;
                        let iam: f64 = row.get(3)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: ta + iam,
                            voyage_count: row.get(4)?,
                            ta_embarked: ta,
                            iam_embarked: iam,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
        }
    }

    /// List destination ports with aggregate embark/voyage counts for the given time range and dataset.
    pub fn get_destination_ports(
        &self,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<DestinationInfo>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                let mut stmt = conn.prepare(
                    "SELECT pf.disembark_port,
                            COALESCE(l.label, CAST(pf.disembark_port AS TEXT)),
                            SUM(pf.embarked),
                            SUM(pf.voyages)
                     FROM port_flows pf
                     LEFT JOIN labels l ON l.variable = 'MJSLPTIMP' AND l.code = pf.disembark_port
                     WHERE pf.year >= ?1 AND pf.year <= ?2
                     GROUP BY pf.disembark_port
                     ORDER BY SUM(pf.embarked) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let emb: f64 = row.get(2)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: emb,
                            voyage_count: row.get(3)?,
                            ta_embarked: emb,
                            iam_embarked: 0.0,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
            DatasetMode::InterAmerican => {
                let mut stmt = conn.prepare(
                    "SELECT pf.disembark_port,
                            COALESCE(l.label, CAST(pf.disembark_port AS TEXT)),
                            SUM(pf.embarked),
                            SUM(pf.voyages)
                     FROM iam_port_flows pf
                     LEFT JOIN labels l ON l.variable = 'MJSLPTIMP' AND l.code = pf.disembark_port
                     WHERE pf.year >= ?1 AND pf.year <= ?2
                     GROUP BY pf.disembark_port
                     ORDER BY SUM(pf.embarked) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let emb: f64 = row.get(2)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: emb,
                            voyage_count: row.get(3)?,
                            ta_embarked: 0.0,
                            iam_embarked: emb,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
            DatasetMode::Combined => {
                let mut stmt = conn.prepare(
                    "SELECT code, name, SUM(ta_emb), SUM(iam_emb), SUM(voys)
                     FROM (
                         SELECT pf.disembark_port AS code,
                                COALESCE(l.label, CAST(pf.disembark_port AS TEXT)) AS name,
                                SUM(pf.embarked) AS ta_emb,
                                0 AS iam_emb,
                                SUM(pf.voyages) AS voys
                         FROM port_flows pf
                         LEFT JOIN labels l ON l.variable = 'MJSLPTIMP' AND l.code = pf.disembark_port
                         WHERE pf.year >= ?1 AND pf.year <= ?2
                         GROUP BY pf.disembark_port
                         UNION ALL
                         SELECT pf.disembark_port AS code,
                                COALESCE(l.label, CAST(pf.disembark_port AS TEXT)) AS name,
                                0 AS ta_emb,
                                SUM(pf.embarked) AS iam_emb,
                                SUM(pf.voyages) AS voys
                         FROM iam_port_flows pf
                         LEFT JOIN labels l ON l.variable = 'MJSLPTIMP' AND l.code = pf.disembark_port
                         WHERE pf.year >= ?1 AND pf.year <= ?2
                         GROUP BY pf.disembark_port
                     )
                     GROUP BY code
                     ORDER BY SUM(ta_emb) + SUM(iam_emb) DESC",
                )?;
                let rows = stmt
                    .query_map(params![year_min, year_max], |row| {
                        let ta: f64 = row.get(2)?;
                        let iam: f64 = row.get(3)?;
                        Ok(DestinationInfo {
                            code: row.get(0)?,
                            name: row.get::<_, String>(1)?.trim().to_string(),
                            total_embarked: ta + iam,
                            voyage_count: row.get(4)?,
                            ta_embarked: ta,
                            iam_embarked: iam,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            }
        }
    }

    /// Get origin-flow breakdown for a destination region (grouped by embark region).
    pub fn get_flows_to_region(
        &self,
        dest: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<FlowData>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                Self::query_flows_region(&conn, "region_flows", "transatlantic", dest, year_min, year_max)
            }
            DatasetMode::InterAmerican => {
                Self::query_flows_region(&conn, "iam_region_flows", "interamerican", dest, year_min, year_max)
            }
            DatasetMode::Combined => {
                let mut ta = Self::query_flows_region(&conn, "region_flows", "transatlantic", dest, year_min, year_max)?;
                let iam = Self::query_flows_region(&conn, "iam_region_flows", "interamerican", dest, year_min, year_max)?;
                ta.extend(iam);
                Ok(ta)
            }
        }
    }

    fn query_flows_region(
        conn: &Connection,
        table: &str,
        dataset: &str,
        dest: i64,
        year_min: i64,
        year_max: i64,
    ) -> anyhow::Result<Vec<FlowData>> {
        let sql = format!(
            "SELECT rf.embark_region,
                    COALESCE(sl.label, CAST(rf.embark_region AS TEXT)),
                    rf.disembark_region,
                    COALESCE(dl.label, CAST(rf.disembark_region AS TEXT)),
                    SUM(rf.embarked),
                    SUM(rf.disembarked),
                    SUM(rf.voyages),
                    MIN(rf.year),
                    MAX(rf.year)
             FROM {table} rf
             LEFT JOIN labels sl ON sl.variable = 'MAJBYIMP' AND sl.code = rf.embark_region
             LEFT JOIN labels dl ON dl.variable = 'MJSELIMP' AND dl.code = rf.disembark_region
             WHERE rf.disembark_region = ?1 AND rf.year >= ?2 AND rf.year <= ?3
             GROUP BY rf.embark_region
             ORDER BY SUM(rf.embarked) DESC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map(params![dest, year_min, year_max], |row| {
                Ok(FlowData {
                    source_code: row.get(0)?,
                    source_name: row.get::<_, String>(1)?.trim().to_string(),
                    target_code: row.get(2)?,
                    target_name: row.get::<_, String>(3)?.trim().to_string(),
                    total_embarked: row.get(4)?,
                    total_disembarked: row.get(5)?,
                    voyage_count: row.get(6)?,
                    first_year: row.get(7)?,
                    last_year: row.get(8)?,
                    dataset: dataset.to_string(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Get origin-flow breakdown for a destination port (grouped by embark region).
    pub fn get_flows_to_port(
        &self,
        dest: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<FlowData>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                Self::query_flows_port(&conn, "port_flows", "transatlantic", dest, year_min, year_max)
            }
            DatasetMode::InterAmerican => {
                Self::query_flows_port(&conn, "iam_port_flows", "interamerican", dest, year_min, year_max)
            }
            DatasetMode::Combined => {
                let mut ta = Self::query_flows_port(&conn, "port_flows", "transatlantic", dest, year_min, year_max)?;
                let iam = Self::query_flows_port(&conn, "iam_port_flows", "interamerican", dest, year_min, year_max)?;
                ta.extend(iam);
                Ok(ta)
            }
        }
    }

    fn query_flows_port(
        conn: &Connection,
        table: &str,
        dataset: &str,
        dest: i64,
        year_min: i64,
        year_max: i64,
    ) -> anyhow::Result<Vec<FlowData>> {
        let sql = format!(
            "SELECT pf.embark_region,
                    COALESCE(sl.label, CAST(pf.embark_region AS TEXT)),
                    pf.disembark_port,
                    COALESCE(dl.label, CAST(pf.disembark_port AS TEXT)),
                    SUM(pf.embarked),
                    SUM(pf.disembarked),
                    SUM(pf.voyages),
                    MIN(pf.year),
                    MAX(pf.year)
             FROM {table} pf
             LEFT JOIN labels sl ON sl.variable = 'MAJBYIMP' AND sl.code = pf.embark_region
             LEFT JOIN labels dl ON dl.variable = 'MJSLPTIMP' AND dl.code = pf.disembark_port
             WHERE pf.disembark_port = ?1 AND pf.year >= ?2 AND pf.year <= ?3
             GROUP BY pf.embark_region
             ORDER BY SUM(pf.embarked) DESC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map(params![dest, year_min, year_max], |row| {
                Ok(FlowData {
                    source_code: row.get(0)?,
                    source_name: row.get::<_, String>(1)?.trim().to_string(),
                    target_code: row.get(2)?,
                    target_name: row.get::<_, String>(3)?.trim().to_string(),
                    total_embarked: row.get(4)?,
                    total_disembarked: row.get(5)?,
                    voyage_count: row.get(6)?,
                    first_year: row.get(7)?,
                    last_year: row.get(8)?,
                    dataset: dataset.to_string(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Get per-year, per-source breakdown for a destination region (for timeline chart).
    pub fn get_yearly_flows_to_region(
        &self,
        dest: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<YearlySourceFlow>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                Self::query_yearly_region(&conn, "region_flows", "transatlantic", dest, year_min, year_max)
            }
            DatasetMode::InterAmerican => {
                Self::query_yearly_region(&conn, "iam_region_flows", "interamerican", dest, year_min, year_max)
            }
            DatasetMode::Combined => {
                let mut ta = Self::query_yearly_region(&conn, "region_flows", "transatlantic", dest, year_min, year_max)?;
                let iam = Self::query_yearly_region(&conn, "iam_region_flows", "interamerican", dest, year_min, year_max)?;
                ta.extend(iam);
                Ok(ta)
            }
        }
    }

    fn query_yearly_region(
        conn: &Connection,
        table: &str,
        dataset: &str,
        dest: i64,
        year_min: i64,
        year_max: i64,
    ) -> anyhow::Result<Vec<YearlySourceFlow>> {
        let sql = format!(
            "SELECT rf.year, rf.embark_region,
                    COALESCE(sl.label, CAST(rf.embark_region AS TEXT)),
                    SUM(rf.embarked)
             FROM {table} rf
             LEFT JOIN labels sl ON sl.variable = 'MAJBYIMP' AND sl.code = rf.embark_region
             WHERE rf.disembark_region = ?1 AND rf.year >= ?2 AND rf.year <= ?3
             GROUP BY rf.year, rf.embark_region
             ORDER BY rf.year, SUM(rf.embarked) DESC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map(params![dest, year_min, year_max], |row| {
                Ok(YearlySourceFlow {
                    year: row.get(0)?,
                    source_code: row.get(1)?,
                    source_name: row.get::<_, String>(2)?.trim().to_string(),
                    embarked: row.get(3)?,
                    dataset: dataset.to_string(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Get per-year, per-source breakdown for a destination port (for timeline chart).
    pub fn get_yearly_flows_to_port(
        &self,
        dest: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<YearlySourceFlow>> {
        let conn = self.conn.borrow();
        match mode {
            DatasetMode::Transatlantic => {
                Self::query_yearly_port(&conn, "port_flows", "transatlantic", dest, year_min, year_max)
            }
            DatasetMode::InterAmerican => {
                Self::query_yearly_port(&conn, "iam_port_flows", "interamerican", dest, year_min, year_max)
            }
            DatasetMode::Combined => {
                let mut ta = Self::query_yearly_port(&conn, "port_flows", "transatlantic", dest, year_min, year_max)?;
                let iam = Self::query_yearly_port(&conn, "iam_port_flows", "interamerican", dest, year_min, year_max)?;
                ta.extend(iam);
                Ok(ta)
            }
        }
    }

    fn query_yearly_port(
        conn: &Connection,
        table: &str,
        dataset: &str,
        dest: i64,
        year_min: i64,
        year_max: i64,
    ) -> anyhow::Result<Vec<YearlySourceFlow>> {
        let sql = format!(
            "SELECT pf.year, pf.embark_region,
                    COALESCE(sl.label, CAST(pf.embark_region AS TEXT)),
                    SUM(pf.embarked)
             FROM {table} pf
             LEFT JOIN labels sl ON sl.variable = 'MAJBYIMP' AND sl.code = pf.embark_region
             WHERE pf.disembark_port = ?1 AND pf.year >= ?2 AND pf.year <= ?3
             GROUP BY pf.year, pf.embark_region
             ORDER BY pf.year, SUM(pf.embarked) DESC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map(params![dest, year_min, year_max], |row| {
                Ok(YearlySourceFlow {
                    year: row.get(0)?,
                    source_code: row.get(1)?,
                    source_name: row.get::<_, String>(2)?.trim().to_string(),
                    embarked: row.get(3)?,
                    dataset: dataset.to_string(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    fn voyage_query_sql(table: &str) -> String {
        format!(
            "SELECT v.voyage_id, v.ship_name, v.captain_a, v.captain_b, v.captain_c,
                    v.embark_region,
                    COALESCE(er.label, CAST(v.embark_region AS TEXT)),
                    v.year_arrived, v.embarked, v.disembarked, v.deaths,
                    ep1.label, ep2.label, ep3.label,
                    v.embarked_port1, v.embarked_port2, v.embarked_port3,
                    dp1.label, dp2.label, dp3.label,
                    v.disembarked_place1, v.disembarked_place2, v.disembarked_place3{vid2_col}
             FROM {table} v
             LEFT JOIN geo_labels er ON er.code = v.embark_region
             LEFT JOIN geo_labels ep1 ON ep1.code = v.embark_port1
             LEFT JOIN geo_labels ep2 ON ep2.code = v.embark_port2
             LEFT JOIN geo_labels ep3 ON ep3.code = v.embark_port3
             LEFT JOIN geo_labels dp1 ON dp1.code = v.disembark_place1
             LEFT JOIN geo_labels dp2 ON dp2.code = v.disembark_place2
             LEFT JOIN geo_labels dp3 ON dp3.code = v.disembark_place3",
            table = table,
            vid2_col = if table == "iam_voyages" { ", v.voyage_id2" } else { "" },
        )
    }

    fn row_to_voyage(row: &rusqlite::Row, dataset: &str, has_vid2: bool) -> rusqlite::Result<VoyageRecord> {
        Ok(VoyageRecord {
            voyage_id: row.get(0)?,
            ship_name: row.get::<_, String>(1)?.trim().to_string(),
            captain_a: row.get::<_, String>(2)?.trim().to_string(),
            captain_b: row.get::<_, String>(3)?.trim().to_string(),
            captain_c: row.get::<_, String>(4)?.trim().to_string(),
            embark_region_code: row.get(5)?,
            embark_region_name: row.get::<_, String>(6)?.trim().to_string(),
            year_arrived: row.get(7)?,
            embarked: row.get(8)?,
            disembarked: row.get(9)?,
            deaths: row.get(10)?,
            embark_port1_name: row.get::<_, Option<String>>(11)?.unwrap_or_default().trim().to_string(),
            embark_port2_name: row.get::<_, Option<String>>(12)?.unwrap_or_default().trim().to_string(),
            embark_port3_name: row.get::<_, Option<String>>(13)?.unwrap_or_default().trim().to_string(),
            embarked_port1: row.get(14)?,
            embarked_port2: row.get(15)?,
            embarked_port3: row.get(16)?,
            disembark_place1_name: row.get::<_, Option<String>>(17)?.unwrap_or_default().trim().to_string(),
            disembark_place2_name: row.get::<_, Option<String>>(18)?.unwrap_or_default().trim().to_string(),
            disembark_place3_name: row.get::<_, Option<String>>(19)?.unwrap_or_default().trim().to_string(),
            disembarked_place1: row.get(20)?,
            disembarked_place2: row.get(21)?,
            disembarked_place3: row.get(22)?,
            dataset: dataset.to_string(),
            linked_voyage_id: if has_vid2 { row.get(23)? } else { None },
        })
    }

    /// Get individual voyage records for a destination region.
    pub fn get_voyages_for_region(
        &self,
        dest_region: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<VoyageRecord>> {
        let conn = self.conn.borrow();
        let mut results = Vec::new();

        if mode == DatasetMode::Transatlantic || mode == DatasetMode::Combined {
            let sql = format!(
                "{} WHERE v.disembark_region = ?1 AND v.year_arrived >= ?2 AND v.year_arrived <= ?3 ORDER BY v.embark_region, v.year_arrived",
                Self::voyage_query_sql("voyages")
            );
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt
                .query_map(params![dest_region, year_min, year_max], |row| {
                    Self::row_to_voyage(row, "transatlantic", false)
                })?
                .collect::<Result<Vec<_>, _>>()?;
            results.extend(rows);
        }

        if mode == DatasetMode::InterAmerican || mode == DatasetMode::Combined {
            let sql = format!(
                "{} WHERE v.disembark_region = ?1 AND v.year_arrived >= ?2 AND v.year_arrived <= ?3 ORDER BY v.embark_region, v.year_arrived",
                Self::voyage_query_sql("iam_voyages")
            );
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt
                .query_map(params![dest_region, year_min, year_max], |row| {
                    Self::row_to_voyage(row, "interamerican", true)
                })?
                .collect::<Result<Vec<_>, _>>()?;
            results.extend(rows);
        }

        Ok(results)
    }

    /// Get individual voyage records for a destination port.
    pub fn get_voyages_for_port(
        &self,
        dest_port: i64,
        year_min: i64,
        year_max: i64,
        mode: DatasetMode,
    ) -> anyhow::Result<Vec<VoyageRecord>> {
        let conn = self.conn.borrow();
        let mut results = Vec::new();

        if mode == DatasetMode::Transatlantic || mode == DatasetMode::Combined {
            let sql = format!(
                "{} WHERE v.disembark_port = ?1 AND v.year_arrived >= ?2 AND v.year_arrived <= ?3 ORDER BY v.embark_region, v.year_arrived",
                Self::voyage_query_sql("voyages")
            );
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt
                .query_map(params![dest_port, year_min, year_max], |row| {
                    Self::row_to_voyage(row, "transatlantic", false)
                })?
                .collect::<Result<Vec<_>, _>>()?;
            results.extend(rows);
        }

        if mode == DatasetMode::InterAmerican || mode == DatasetMode::Combined {
            let sql = format!(
                "{} WHERE v.disembark_port = ?1 AND v.year_arrived >= ?2 AND v.year_arrived <= ?3 ORDER BY v.embark_region, v.year_arrived",
                Self::voyage_query_sql("iam_voyages")
            );
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt
                .query_map(params![dest_port, year_min, year_max], |row| {
                    Self::row_to_voyage(row, "interamerican", true)
                })?
                .collect::<Result<Vec<_>, _>>()?;
            results.extend(rows);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Database {
        Database::new().expect("Database should initialize from embedded CSVs")
    }

    #[test]
    fn year_range_transatlantic() {
        let db = test_db();
        let (min, max) = db.get_year_range(DatasetMode::Transatlantic).unwrap();
        assert!(min >= 1500 && min <= 1520, "TA min year {min} out of expected range");
        assert!(max >= 1860 && max <= 1870, "TA max year {max} out of expected range");
        assert!(min < max);
    }

    #[test]
    fn year_range_interamerican() {
        let db = test_db();
        let (min, max) = db.get_year_range(DatasetMode::InterAmerican).unwrap();
        assert!(min >= 1540 && min <= 1560, "IAm min year {min} out of expected range");
        assert!(max >= 1835 && max <= 1845, "IAm max year {max} out of expected range");
        assert!(min < max);
    }

    #[test]
    fn year_range_combined_spans_both() {
        let db = test_db();
        let (ta_min, _) = db.get_year_range(DatasetMode::Transatlantic).unwrap();
        let (_, iam_max) = db.get_year_range(DatasetMode::InterAmerican).unwrap();
        let (c_min, c_max) = db.get_year_range(DatasetMode::Combined).unwrap();
        assert!(c_min <= ta_min, "Combined min should be <= TA min");
        assert!(c_max >= iam_max, "Combined max should be >= IAm max");
    }

    #[test]
    fn destinations_not_empty() {
        let db = test_db();
        let dests = db
            .get_destination_regions(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        assert!(!dests.is_empty(), "Should have destination regions");
        assert!(dests[0].total_embarked > 0.0, "Top destination should have embarked > 0");
    }

    #[test]
    fn destination_ports_not_empty() {
        let db = test_db();
        let ports = db
            .get_destination_ports(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        assert!(!ports.is_empty(), "Should have destination ports");
    }

    #[test]
    fn flows_to_region() {
        let db = test_db();
        let dests = db
            .get_destination_regions(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        let top_dest = dests[0].code;
        let flows = db
            .get_flows_to_region(top_dest, 1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        assert!(!flows.is_empty(), "Top destination should have origin flows");
        for f in &flows {
            assert_eq!(f.dataset, "transatlantic");
            assert_eq!(f.target_code, top_dest);
        }
    }

    #[test]
    fn flows_to_port() {
        let db = test_db();
        let ports = db
            .get_destination_ports(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        let top_port = ports[0].code;
        let flows = db
            .get_flows_to_port(top_port, 1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        assert!(!flows.is_empty(), "Top port should have origin flows");
    }

    #[test]
    fn voyages_for_region() {
        let db = test_db();
        let dests = db
            .get_destination_regions(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        let top_dest = dests[0].code;
        let voyages = db
            .get_voyages_for_region(top_dest, 1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        assert!(!voyages.is_empty(), "Should have voyage records");
        for v in &voyages {
            assert_eq!(v.dataset, "transatlantic");
        }
    }

    #[test]
    fn combined_mode_merges_datasets() {
        let db = test_db();
        let dests = db
            .get_destination_regions(1500, 1900, DatasetMode::Combined)
            .unwrap();
        assert!(!dests.is_empty(), "Combined mode should have destinations");
        // At least one dest should have both TA and IAm data (or just one — but total > 0)
        let has_ta = dests.iter().any(|d| d.ta_embarked > 0.0);
        let has_iam = dests.iter().any(|d| d.iam_embarked > 0.0);
        assert!(has_ta, "Combined destinations should include TA data");
        assert!(has_iam, "Combined destinations should include IAm data");
    }

    #[test]
    fn narrow_time_window_filters() {
        let db = test_db();
        let full = db
            .get_destination_regions(1500, 1900, DatasetMode::Transatlantic)
            .unwrap();
        let narrow = db
            .get_destination_regions(1700, 1750, DatasetMode::Transatlantic)
            .unwrap();
        let full_total: f64 = full.iter().map(|d| d.total_embarked).sum();
        let narrow_total: f64 = narrow.iter().map(|d| d.total_embarked).sum();
        assert!(
            narrow_total < full_total,
            "Narrow window {narrow_total} should have fewer embarked than full range {full_total}"
        );
    }

    /// The normalization formula: port-like codes (not multiples of 100) round
    /// down to their parent region. Verify the arithmetic against known cases.
    #[test]
    fn region_code_normalization_arithmetic() {
        let normalize = |c: i64| -> i64 {
            if c % 100 != 0 { (c / 100) * 100 } else { c }
        };

        // Already a region code — should pass through unchanged
        assert_eq!(normalize(50300), 50300, "50300 (Pernambuco) stays 50300");
        assert_eq!(normalize(60200), 60200, "60200 (Senegambia) stays 60200");
        assert_eq!(normalize(31100), 31100, "31100 (Caribbean) stays 31100");
        assert_eq!(normalize(10000), 10000, "10000 stays 10000");

        // Port-like codes — should round down to parent region
        assert_eq!(normalize(50311), 50300, "50311 → 50300 (the known bug case)");
        assert_eq!(normalize(50301), 50300, "50301 → 50300");
        assert_eq!(normalize(50399), 50300, "50399 → 50300");
        assert_eq!(normalize(31101), 31100, "31101 → 31100");
        assert_eq!(normalize(60299), 60200, "60299 → 60200");

        // Edge: code ending in 00 but not a "nice" region (e.g. 50000)
        assert_eq!(normalize(50000), 50000, "50000 stays 50000");
        assert_eq!(normalize(99900), 99900, "99900 stays 99900");
    }

    /// After build.rs normalization, no disembark_region code in the TASTD
    /// region_flows table should be a port-like code (not a multiple of 100).
    #[test]
    fn tastd_region_codes_are_normalized() {
        let db = test_db();
        let conn = db.conn.borrow();
        let mut stmt = conn
            .prepare("SELECT DISTINCT disembark_region FROM region_flows")
            .unwrap();
        let rows: Vec<i64> = stmt
            .query_map([], |row| row.get::<_, i64>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(!rows.is_empty(), "Should have TASTD region flow data");
        for code in &rows {
            assert_eq!(
                code % 100, 0,
                "TASTD disembark_region {} is not a region-level code (should be multiple of 100)",
                code
            );
        }
    }

    /// After build.rs normalization, no disembark_region code in the I-Am
    /// region_flows table should be a port-like code (not a multiple of 100).
    #[test]
    fn iam_region_codes_are_normalized() {
        let db = test_db();
        let conn = db.conn.borrow();
        let mut stmt = conn
            .prepare("SELECT DISTINCT disembark_region FROM iam_region_flows")
            .unwrap();
        let rows: Vec<i64> = stmt
            .query_map([], |row| row.get::<_, i64>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(!rows.is_empty(), "Should have I-Am region flow data");
        for code in &rows {
            assert_eq!(
                code % 100, 0,
                "I-Am disembark_region {} is not a region-level code (should be multiple of 100)",
                code
            );
        }
    }

    /// Specifically verify that code 50311 (the known bug) does not appear
    /// anywhere in region_flows, and that 50300 (Pernambuco) does.
    #[test]
    fn no_50311_in_region_flows() {
        let db = test_db();
        let conn = db.conn.borrow();

        let count_50311: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM region_flows WHERE disembark_region = 50311",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_50311, 0, "Code 50311 should not exist in region_flows");

        let count_50300: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM region_flows WHERE disembark_region = 50300",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count_50300 > 0, "Code 50300 (Pernambuco) should exist in region_flows");
    }

    /// Embark regions should also all be region-level codes (multiples of 100).
    #[test]
    fn embark_region_codes_are_region_level() {
        let db = test_db();
        let conn = db.conn.borrow();
        let mut stmt = conn
            .prepare("SELECT DISTINCT embark_region FROM region_flows")
            .unwrap();
        let rows: Vec<i64> = stmt
            .query_map([], |row| row.get::<_, i64>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(!rows.is_empty());
        for code in &rows {
            assert_eq!(
                code % 100, 0,
                "embark_region {} is not a region-level code",
                code
            );
        }
    }

    #[test]
    fn iam_voyages_have_dataset_tag() {
        let db = test_db();
        let dests = db
            .get_destination_regions(1500, 1900, DatasetMode::InterAmerican)
            .unwrap();
        if let Some(top) = dests.first() {
            let voyages = db
                .get_voyages_for_region(top.code, 1500, 1900, DatasetMode::InterAmerican)
                .unwrap();
            for v in &voyages {
                assert_eq!(v.dataset, "interamerican");
            }
        }
    }
}
