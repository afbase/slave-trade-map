"""Feature set definitions and categorical encoding for UMAP analysis."""

import numpy as np
import pandas as pd

from .config import (
    BROAD_REGION_COLS,
    DEMOGRAPHIC_RATIOS,
    DEMOGRAPHIC_RATIOS_ALL,
    MORTALITY_COLS,
    SPECIFIC_REGION_COLS,
    VOYAGE_COLS,
)


def encode_categorical(df: pd.DataFrame, cols: list[str]) -> pd.DataFrame:
    """One-hot encode categorical columns, returning the encoded DataFrame.

    Each column is encoded with a prefix matching its name (e.g. NATINIMP_3).
    Original categorical columns are dropped from the result.
    """
    present = [c for c in cols if c in df.columns]
    if not present:
        return df
    return pd.get_dummies(df, columns=present, prefix={c: c for c in present})


def _available(df: pd.DataFrame, cols: list[str]) -> list[str]:
    """Return only those column names that actually exist in *df*."""
    return [c for c in cols if c in df.columns]


def _build_feature_df(
    df: pd.DataFrame,
    continuous: list[str],
    categorical: list[str],
    include_missing_indicators: bool = False,
) -> tuple[np.ndarray, list[str], pd.Index]:
    """Shared helper: select columns, one-hot encode, fill NaN, return matrix.

    Returns
    -------
    matrix : np.ndarray
    feature_names : list[str]
    valid_index : pd.Index   (row indices of *df* that were used)
    """
    cont = _available(df, continuous)
    cat = _available(df, categorical)

    sub = df[cont + cat].copy()

    if include_missing_indicators:
        missing_cols = [c for c in df.columns if c.endswith("_MISSING")]
        for mc in missing_cols:
            if mc not in sub.columns:
                sub[mc] = df[mc]

    sub = encode_categorical(sub, cat)
    sub = sub.fillna(0)

    matrix = sub.values.astype(np.float64)
    return matrix, list(sub.columns), sub.index


# ── Feature Set A: Full Voyage Profile (~80-90 features) ─────────────────


def get_feature_set_a(df: pd.DataFrame) -> tuple[np.ndarray, list[str], pd.Index]:
    """Full voyage profile: geography, scale, demographics, mortality, voyage."""
    categorical = ["MAJBYIMP1", "MJSELIMP1", "NATINIMP", "FATE", "RESISTANCE"]
    continuous = (
        ["YEARAM", "LOG_SLAXIMP", "LOG_SLAMIMP", "LOG_TONNAGE"]
        + DEMOGRAPHIC_RATIOS
        + VOYAGE_COLS
        + ["VYMRTIMP", "MORTALITY_RATE"]
        + ["SLAVES_PER_TON", "CREW_PER_TON"]
    )
    return _build_feature_df(df, continuous, categorical, include_missing_indicators=True)


# ── Feature Set B: Trade Route Geography (~170 cols) ─────────────────────


def get_feature_set_b(df: pd.DataFrame) -> tuple[np.ndarray, list[str], pd.Index]:
    """Trade route geography: specific regions (many categories) + scale."""
    categorical = SPECIFIC_REGION_COLS + ["NATINIMP"]
    continuous = ["YEARAM", "LOG_SLAXIMP"]
    return _build_feature_df(df, continuous, categorical)


# ── Feature Set C: Mortality & Demographics ──────────────────────────────


def get_feature_set_c(df: pd.DataFrame) -> tuple[np.ndarray, list[str], pd.Index]:
    """Mortality and demographic ratios with voyage context."""
    continuous = (
        list(DEMOGRAPHIC_RATIOS_ALL)
        + list(MORTALITY_COLS)
        + [
            "LOG_SLAXIMP",
            "SLAVES_PER_TON",
            "VOY2IMP",
            "YEARAM",
            "MORTALITY_RATE",
            "SURVIVAL_RATE",
        ]
    )
    return _build_feature_df(df, continuous, categorical=[])


# ── Feature Set D: Temporal + National ───────────────────────────────────


def get_feature_set_d(df: pd.DataFrame) -> tuple[np.ndarray, list[str], pd.Index]:
    """Temporal evolution by nationality and broad region."""
    categorical = ["NATINIMP"] + list(BROAD_REGION_COLS)
    continuous = ["YEARAM", "LOG_SLAXIMP"]
    return _build_feature_df(df, continuous, categorical)


# ── Assembly helper ──────────────────────────────────────────────────────


def assemble_matrix(
    df: pd.DataFrame,
    feature_set_func,
) -> tuple[np.ndarray, list[str], pd.Index]:
    """Call *feature_set_func* on *df*, fill remaining NaN with 0, and return.

    Returns
    -------
    matrix : np.ndarray   (n_samples, n_features)
    feature_names : list[str]
    valid_index : pd.Index
    """
    matrix, names, idx = feature_set_func(df)
    # Belt-and-suspenders: ensure no NaN leak through
    np.nan_to_num(matrix, copy=False, nan=0.0)
    return matrix, names, idx
