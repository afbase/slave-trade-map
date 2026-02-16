"""Data preprocessing for UMAP analysis of the slave trade database."""

import logging

import numpy as np
import pandas as pd
from sklearn.preprocessing import StandardScaler

from .config import (
    MISSINGNESS_KEY_COLS,
    TRADE_ERA_BINS,
    TRADE_ERA_LABELS,
)

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# 1. Row filtering
# ---------------------------------------------------------------------------

def filter_rows(df: pd.DataFrame) -> pd.DataFrame:
    """Filter to rows that have a year, embarkation region, disembarkation region,
    and at least one slave count."""
    n_orig = len(df)

    has_year = df["YEARAM"].notna() | df["YEARDEP"].notna()
    has_emb = df["MAJBYIMP"].notna() | df["MAJBYIMP1"].notna()
    has_dis = df["MJSELIMP"].notna() | df["MJSELIMP1"].notna()
    has_slaves = df["SLAXIMP"].notna() | df["SLAMIMP"].notna()

    mask = has_year & has_emb & has_dis & has_slaves
    out = df.loc[mask].copy()
    logger.info("filter_rows: kept %d / %d rows (%.1f%%)",
                len(out), n_orig, 100 * len(out) / n_orig if n_orig else 0)
    return out


# ---------------------------------------------------------------------------
# 2. Missingness indicators
# ---------------------------------------------------------------------------

def add_missingness_indicators(df: pd.DataFrame, cols: list[str]) -> pd.DataFrame:
    """Add binary ``<col>_MISSING`` columns before imputation."""
    df = df.copy()
    for col in cols:
        if col in df.columns:
            df[f"{col}_MISSING"] = df[col].isna().astype(int)
    return df


# ---------------------------------------------------------------------------
# 3. Imputation
# ---------------------------------------------------------------------------

def impute_missing(df: pd.DataFrame) -> pd.DataFrame:
    """Group-median imputation: group by (NATINIMP, TRADE_ERA), fill NaN with
    group median for all numeric columns, then fall back to global median."""
    df = df.copy()

    # Build TRADE_ERA if not already present
    if "TRADE_ERA" not in df.columns and "YEARAM" in df.columns:
        df["TRADE_ERA"] = pd.cut(df["YEARAM"], bins=TRADE_ERA_BINS,
                                 labels=TRADE_ERA_LABELS)

    numeric_cols = df.select_dtypes(include="number").columns.tolist()

    # Group-median imputation
    if "NATINIMP" in df.columns and "TRADE_ERA" in df.columns:
        group_medians = df.groupby(["NATINIMP", "TRADE_ERA"], observed=True)[numeric_cols].transform("median")
        df[numeric_cols] = df[numeric_cols].fillna(group_medians)

    # Fallback: global median for anything still missing
    global_medians = df[numeric_cols].median()
    df[numeric_cols] = df[numeric_cols].fillna(global_medians)

    return df


# ---------------------------------------------------------------------------
# 4. Feature engineering
# ---------------------------------------------------------------------------

def engineer_features(df: pd.DataFrame) -> pd.DataFrame:
    """Create derived features from raw columns."""
    df = df.copy()

    # Log-scaled counts
    if "SLAXIMP" in df.columns:
        df["LOG_SLAXIMP"] = np.log1p(df["SLAXIMP"])
    if "SLAMIMP" in df.columns:
        df["LOG_SLAMIMP"] = np.log1p(df["SLAMIMP"])
    if "TONNAGE" in df.columns:
        df["LOG_TONNAGE"] = np.log1p(df["TONNAGE"])

    # Mortality rate
    if "VYMRTIMP" in df.columns:
        df["MORTALITY_RATE"] = df["VYMRTIMP"] / 100.0
    elif "SLADVOY" in df.columns and "SLAXIMP" in df.columns:
        df["MORTALITY_RATE"] = df["SLADVOY"] / df["SLAXIMP"].replace(0, np.nan)

    # Survival rate
    if "MORTALITY_RATE" in df.columns:
        df["SURVIVAL_RATE"] = 1.0 - df["MORTALITY_RATE"]

    # Per-ton ratios (guard against zero tonnage)
    if "TONNAGE" in df.columns:
        safe_tonnage = df["TONNAGE"].replace(0, np.nan)
        if "SLAXIMP" in df.columns:
            df["SLAVES_PER_TON"] = df["SLAXIMP"] / safe_tonnage
        if "CREW" in df.columns:
            df["CREW_PER_TON"] = df["CREW"] / safe_tonnage

    # Trade era categorical
    if "YEARAM" in df.columns:
        df["TRADE_ERA"] = pd.cut(df["YEARAM"], bins=TRADE_ERA_BINS,
                                 labels=TRADE_ERA_LABELS)

    # Post-abolition flag
    if "YEARAM" in df.columns:
        df["IS_POST_ABOLITION"] = (df["YEARAM"] > 1807).astype(int)

    return df


# ---------------------------------------------------------------------------
# 5. Normalization
# ---------------------------------------------------------------------------

def normalize_features(
    df: pd.DataFrame, continuous_cols: list[str]
) -> tuple[pd.DataFrame, StandardScaler]:
    """Apply StandardScaler to *continuous_cols* (in-place update).

    Returns the modified DataFrame and the fitted scaler.
    """
    df = df.copy()
    present = [c for c in continuous_cols if c in df.columns]
    scaler = StandardScaler()
    df[present] = scaler.fit_transform(df[present])
    return df, scaler


# ---------------------------------------------------------------------------
# 6. Full pipeline
# ---------------------------------------------------------------------------

def preprocess_pipeline(
    df: pd.DataFrame,
) -> tuple[pd.DataFrame, pd.Series]:
    """Run the full preprocessing pipeline.

    Returns
    -------
    clean_df : pd.DataFrame
        Filtered, feature-engineered, and imputed DataFrame.
    original_index : pd.Series
        Mapping from new positional index to original DataFrame index so
        downstream code can map back to raw rows.
    """
    df = filter_rows(df)
    df = add_missingness_indicators(df, MISSINGNESS_KEY_COLS)
    df = engineer_features(df)
    df = impute_missing(df)

    original_index = pd.Series(df.index, name="original_index")
    df = df.reset_index(drop=True)

    return df, original_index
