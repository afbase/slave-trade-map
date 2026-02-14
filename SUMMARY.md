# SPSS-to-CSV Conversion: Trans-Atlantic Slave Trade Database

## Source File

- **File:** `tastdb-exp-2020.sav` (30 MB)
- **Origin:** SlaveVoyages Trans-Atlantic Slave Trade Database (https://www.slavevoyages.org)
- **Contents:** 36,108 voyage records across 274 variables

## Problem

The Rust-based `pspp` CLI tool produced malformed CSV output (space-separated data rows instead of comma-delimited), making it unusable. An alternative conversion method was needed.

## Solution

Used Python with `pyreadstat` (a wrapper around the C ReadStat library) inside a virtual environment to read the `.sav` file and export properly formatted CSVs.

**Script:** `convert_sav.py`

**Dependencies:** `pyreadstat`, `pandas` (installed in `.venv/`)

## Output Files

| File | Size | Rows | Columns | Description |
|---|---|---|---|---|
| `tastdb-exp-2020-raw.csv` | 23 MB | 36,108 | 274 | All variables with raw numeric codes preserved |
| `tastdb-exp-2020-labeled.csv` | 34 MB | 36,108 | 274 | 54 coded variables replaced with human-readable labels; unmapped codes retain original values |
| `tastdb-exp-2020-variables.csv` | 17 KB | 274 | 5 | Variable metadata: position, name, label, type, format |
| `tastdb-exp-2020-value-labels.csv` | 581 KB | 19,101 | 3 | Full code-to-label lookup table across all 54 labeled variables |

## Labeled Variables (54 total)

Variables like `ARRPORT`, `NATIONAL`, `MAJBUYPT`, `FATE`, and 50 others had their numeric codes replaced with descriptive text in the labeled CSV. Examples:

| Variable | Raw Code | Label |
|---|---|---|
| ARRPORT | 60820 | Bahia, port unspecified |
| NATIONAL | 7 | Portugal |
| MAJBUYPT | 60500 | Mozambique |
| FATE | 1 | Voyage completed as intended |

## Validation

- **Row count:** 36,109 lines (36,108 data + 1 header) -- confirmed
- **Column count:** 274 -- confirmed
- **Labeled data:** Spot-checked `ARRPORT`, `NATIONAL`, `MAJBUYPT`, `FATE` -- all show readable place/country/status names

## Cleanup

The malformed `tastdb-exp-2020.csv` produced by the Rust `pspp` tool was removed.

## Reproducibility

To re-run the conversion:

```bash
# Activate the venv (already created)
source /private/tmp/spss-convert/.venv/bin/activate

# Or run directly:
/private/tmp/spss-convert/.venv/bin/python3 /private/tmp/spss-convert/convert_sav.py
```

To recreate the venv from scratch:

```bash
python3 -m venv /private/tmp/spss-convert/.venv
/private/tmp/spss-convert/.venv/bin/pip install pyreadstat pandas
/private/tmp/spss-convert/.venv/bin/python3 /private/tmp/spss-convert/convert_sav.py
```
