#!/usr/bin/env python3
"""Convert tastdb-exp-2020.sav (SPSS) to CSV files."""

import csv
import sys
from pathlib import Path

import pyreadstat
import pandas as pd

BASE = Path("/private/tmp/spss-convert")
INPUT = BASE / "tastdb-exp-2020.sav"

# --- Read the .sav file (raw numeric codes) ---
print(f"Reading {INPUT} ...")
df, meta = pyreadstat.read_sav(str(INPUT), apply_value_formats=False)
print(f"  Shape: {df.shape[0]} rows x {df.shape[1]} columns")

# --- 1. Raw CSV ---
raw_path = BASE / "tastdb-exp-2020-raw.csv"
df.to_csv(raw_path, index=False)
print(f"Wrote {raw_path.name}")

# --- 2. Labeled CSV ---
labeled_df = df.copy()
label_maps = meta.variable_value_labels  # dict of {col: {code: label}}
labeled_cols = sorted(label_maps.keys())
print(f"  Applying labels to {len(labeled_cols)} coded variables ...")

for col in labeled_cols:
    mapping = label_maps[col]
    raw = labeled_df[col]
    mapped = raw.map(mapping)
    # Keep original value where no label exists
    labeled_df[col] = mapped.fillna(raw)

labeled_path = BASE / "tastdb-exp-2020-labeled.csv"
labeled_df.to_csv(labeled_path, index=False)
print(f"Wrote {labeled_path.name}")

# --- 3. Variable metadata CSV ---
col_labels = meta.column_names_to_labels  # {name: label}
var_rows = []
for i, name in enumerate(meta.column_names):
    var_rows.append({
        "position": i + 1,
        "name": name,
        "label": col_labels.get(name, ""),
        "type": str(meta.readstat_variable_types.get(name, "")),
        "format": meta.original_variable_types.get(name, ""),
    })

var_path = BASE / "tastdb-exp-2020-variables.csv"
pd.DataFrame(var_rows).to_csv(var_path, index=False)
print(f"Wrote {var_path.name} ({len(var_rows)} variables)")

# --- 4. Value labels lookup CSV ---
vl_rows = []
for col in sorted(label_maps.keys()):
    for code, label in sorted(label_maps[col].items(), key=lambda x: x[0]):
        vl_rows.append({
            "variable": col,
            "code": code,
            "label": label,
        })

vl_path = BASE / "tastdb-exp-2020-value-labels.csv"
pd.DataFrame(vl_rows).to_csv(vl_path, index=False)
print(f"Wrote {vl_path.name} ({len(vl_rows)} code->label mappings)")

print("\nDone.")
