"""CSV loading, value-label decoding, and column normalization."""

import pandas as pd
from .config import TASTDB_RAW, TASTDB_VALUE_LABELS, TASTDB_VARIABLES, IAM_CSV


def load_tastdb() -> pd.DataFrame:
    """Load the trans-Atlantic slave trade database (36,108 rows, 274 cols)."""
    df = pd.read_csv(TASTDB_RAW, low_memory=False)
    df.columns = df.columns.str.upper()
    print(f"Loaded tastdb: {df.shape[0]:,} rows, {df.shape[1]} cols")
    return df


def load_iam() -> pd.DataFrame:
    """Load the intra-American slave trade database (11,521 rows, 169 cols)."""
    df = pd.read_csv(IAM_CSV, low_memory=False, encoding="latin-1")
    df.columns = df.columns.str.upper()
    print(f"Loaded IAm: {df.shape[0]:,} rows, {df.shape[1]} cols")
    return df


def load_value_labels() -> dict[str, dict[float, str]]:
    """Load value labels: variable -> {code: label} mapping."""
    vl = pd.read_csv(TASTDB_VALUE_LABELS)
    result = {}
    for var, group in vl.groupby("variable"):
        result[var.upper()] = dict(zip(group["code"], group["label"]))
    print(f"Loaded value labels: {len(result)} variables, {len(vl):,} mappings")
    return result


def load_variables() -> pd.DataFrame:
    """Load variable metadata (274 rows)."""
    df = pd.read_csv(TASTDB_VARIABLES)
    df["name"] = df["name"].str.upper()
    print(f"Loaded variable metadata: {df.shape[0]} variables")
    return df


def decode_labels(df: pd.DataFrame, value_labels: dict, columns: list[str]) -> pd.DataFrame:
    """Replace numeric codes with human-readable labels for specified columns."""
    df = df.copy()
    for col in columns:
        col_upper = col.upper()
        if col_upper in value_labels and col_upper in df.columns:
            mapping = value_labels[col_upper]
            df[f"{col_upper}_LABEL"] = df[col_upper].map(mapping)
    return df


def get_nationality_labels(value_labels: dict) -> dict[float, str]:
    """Extract nationality code -> label mapping."""
    return value_labels.get("NATINIMP", {})


def get_region_labels(value_labels: dict) -> dict:
    """Extract region code -> label mappings for key geographic variables."""
    region_vars = [
        "MAJBYIMP", "MAJBYIMP1", "MJSELIMP", "MJSELIMP1",
        "MAJBUYPT", "MAJSELPT",
    ]
    return {var: value_labels.get(var, {}) for var in region_vars}
