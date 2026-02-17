#!/usr/bin/env python3
"""Compute statistics for the Nueva Espana (New Spain) regional analysis.

Filters voyages to ports 41201-41205 (MJSLPTIMP) from the preprocessed
TASTDB data and prints all statistics needed for the conclusions section.
"""

import sys
from pathlib import Path

import numpy as np
import pandas as pd

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "src"))

from tastdb_umap import config
from tastdb_umap.config import (
    NUEVA_ESPANA_PORTS,
    NUEVA_ESPANA_PORT_NAMES,
    NUEVA_GRANADA_PORTS,
    VENEZUELA_PORTS,
    HONDURAS_PORTS,
    SCC_UNSPECIFIED_PORTS,
    NAT_CODE_TO_NAME,
)
from tastdb_umap.data_loader import load_tastdb, load_value_labels
from tastdb_umap.preprocessing import preprocess_pipeline
from tastdb_umap.historical_overlay import tag_chartered_companies
from tastdb_umap.umap_runner import load_cached_embedding


def mask_any_port(df, port_codes, port_cols=None):
    """Return boolean mask: True if any port column matches one of the codes."""
    if port_cols is None:
        port_cols = ["MJSLPTIMP", "MAJSELPT", "SLA1PORT", "ARRPORT"]
    available = [c for c in port_cols if c in df.columns]
    mask = pd.Series(False, index=df.index)
    for col in available:
        mask = mask | df[col].isin(port_codes)
    return mask


def print_section(title):
    print(f"\n{'='*70}")
    print(f"  {title}")
    print(f"{'='*70}")


def main():
    # ── Load ──────────────────────────────────────────────────────────────
    print_section("Loading data")
    tastdb = load_tastdb()
    value_labels = load_value_labels()

    print_section("Preprocessing")
    df, original_idx = preprocess_pipeline(tastdb)
    df = tag_chartered_companies(df)
    print(f"  Preprocessed: {len(df):,} rows")

    # ── SCC region (MJSELIMP = 41200) ─────────────────────────────────────
    print_section("Spanish Circum-Caribbean decomposition")
    scc_mask = df["MJSELIMP"] == 41200
    scc = df[scc_mask]
    print(f"  Total SCC (MJSELIMP=41200): {len(scc):,} voyages")

    ne_mask = mask_any_port(df, NUEVA_ESPANA_PORTS)
    ng_mask = mask_any_port(df, NUEVA_GRANADA_PORTS)
    vz_mask = mask_any_port(df, VENEZUELA_PORTS)
    hn_mask = mask_any_port(df, HONDURAS_PORTS)
    un_mask = mask_any_port(df, SCC_UNSPECIFIED_PORTS)

    # Restrict to SCC region
    ne_scc = (scc_mask & ne_mask).sum()
    ng_scc = (scc_mask & ng_mask).sum()
    vz_scc = (scc_mask & vz_mask).sum()
    hn_scc = (scc_mask & hn_mask).sum()
    un_scc = (scc_mask & un_mask).sum()
    total_decomp = ne_scc + ng_scc + vz_scc + hn_scc + un_scc

    print(f"  Nueva Espana:    {ne_scc:>5}")
    print(f"  Nueva Granada:   {ng_scc:>5}")
    print(f"  Venezuela:       {vz_scc:>5}")
    print(f"  Honduras:        {hn_scc:>5}")
    print(f"  Unspecified SCC: {un_scc:>5}")
    print(f"  ---")
    print(f"  Sum:             {total_decomp:>5}")
    print(f"  Total SCC:       {len(scc):>5}")
    print(f"  Accounted: {100*total_decomp/len(scc):.1f}%")

    # ── Remainder: SCC voyages that don't match any port list
    remainder = len(scc) - total_decomp
    if remainder > 0:
        remainder_mask = scc_mask & ~(ne_mask | ng_mask | vz_mask | hn_mask | un_mask)
        rem_df = df[remainder_mask]
        print(f"\n  Remainder ({remainder} voyages) port code distribution:")
        for col in ["MJSLPTIMP", "MAJSELPT", "SLA1PORT", "ARRPORT"]:
            if col in rem_df.columns:
                vc = rem_df[col].dropna().value_counts().head(10)
                if not vc.empty:
                    print(f"    {col}: {dict(vc)}")

    # ── Nueva Espana analysis ─────────────────────────────────────────────
    ne = df[scc_mask & ne_mask].copy()
    print_section(f"Nueva Espana Overview ({len(ne)} voyages)")

    # Date range
    year_col = "YEARAM"
    year_min = int(ne[year_col].min()) if ne[year_col].notna().any() else "?"
    year_max = int(ne[year_col].max()) if ne[year_col].notna().any() else "?"
    print(f"  Date range: {year_min} - {year_max}")

    # Total enslaved
    total_enslaved = ne["SLAXIMP"].sum()
    print(f"  Total enslaved (SLAXIMP): {total_enslaved:,.0f}")

    # Mortality
    mort_mean = ne["VYMRTIMP"].mean()
    mort_median = ne["VYMRTIMP"].median()
    print(f"  Mortality mean: {mort_mean:.1f}%")
    print(f"  Mortality median: {mort_median:.1f}%")

    # Avg slaves per voyage
    avg_slaves = ne["SLAXIMP"].mean()
    print(f"  Avg slaves/voyage: {avg_slaves:.0f}")

    # ── Port breakdown ────────────────────────────────────────────────────
    print_section("Port Breakdown")
    for code, name in NUEVA_ESPANA_PORT_NAMES.items():
        port_mask = scc_mask & mask_any_port(df, [code])
        sub = df[port_mask]
        if len(sub) == 0:
            continue
        enslaved = sub["SLAXIMP"].sum()
        mort = sub["VYMRTIMP"].mean()
        yr_min = int(sub[year_col].min()) if sub[year_col].notna().any() else "?"
        yr_max = int(sub[year_col].max()) if sub[year_col].notna().any() else "?"
        print(f"  {name} ({code}): {len(sub)} voyages, "
              f"{enslaved:,.0f} enslaved, {mort:.1f}% mort, {yr_min}-{yr_max}")

    # ── Temporal breakdown ────────────────────────────────────────────────
    print_section("Temporal Breakdown (50-year eras)")
    eras = [(1525, 1575), (1575, 1625), (1625, 1675)]
    for lo, hi in eras:
        era_mask = (ne[year_col] >= lo) & (ne[year_col] < hi)
        era = ne[era_mask]
        if len(era) == 0:
            print(f"  {lo}-{hi}: 0 voyages")
            continue
        enslaved = era["SLAXIMP"].sum()
        mort = era["VYMRTIMP"].mean()
        print(f"  {lo}-{hi}: {len(era)} voyages, "
              f"{enslaved:,.0f} enslaved, {mort:.1f}% mort")

    # Also show full 25-year era breakdown
    print("\n  Full 25-year breakdown:")
    for lo in range(1525, 1700, 25):
        hi = lo + 25
        era_mask = (ne[year_col] >= lo) & (ne[year_col] < hi)
        era = ne[era_mask]
        if len(era) == 0:
            continue
        mort = era["VYMRTIMP"].mean()
        print(f"    {lo}-{hi}: {len(era)} voyages, {mort:.1f}% mort")

    # ── Nationality breakdown ─────────────────────────────────────────────
    print_section("Nationality Breakdown")
    nat_col = "NATINIMP"
    nat_vc = ne[nat_col].value_counts()
    for code, count in nat_vc.items():
        name = NAT_CODE_TO_NAME.get(int(code), f"Code {int(code)}")
        nat_sub = ne[ne[nat_col] == code]
        mort = nat_sub["VYMRTIMP"].mean()
        enslaved = nat_sub["SLAXIMP"].sum()
        pct = 100 * count / len(ne)
        print(f"  {name}: {count} voyages ({pct:.1f}%), "
              f"{enslaved:,.0f} enslaved, {mort:.1f}% mort")

    # ── African source regions ────────────────────────────────────────────
    print_section("African Source Regions")
    for col, label in [("MAJBYIMP1", "Broad"), ("MAJBYIMP", "Specific")]:
        if col not in ne.columns:
            continue
        print(f"\n  {label} ({col}):")
        vc = ne[col].value_counts()
        for code, count in vc.head(15).items():
            pct = 100 * count / len(ne)
            lbl = code
            if value_labels and col in value_labels:
                lbl = value_labels[col].get(code, value_labels[col].get(str(int(code)) if isinstance(code, float) else str(code), code))
            print(f"    {lbl}: {count} ({pct:.1f}%)")

    # ── Company involvement ───────────────────────────────────────────────
    print_section("Company Involvement")
    comp_vc = ne["COMPANY"].value_counts()
    for name, count in comp_vc.items():
        pct = 100 * count / len(ne)
        print(f"  {name}: {count} ({pct:.1f}%)")

    # ── Ship metrics ──────────────────────────────────────────────────────
    print_section("Ship Metrics")
    metrics = {
        "TONNAGE": "Avg tonnage",
        "CREW": "Avg crew",
        "GUNS": "Avg guns",
        "VOY1IMP": "Avg voyage duration (days)",
        "VOY2IMP": "Avg middle passage (days)",
        "SLAXIMP": "Avg slaves embarked",
        "SLAMIMP": "Avg slaves disembarked",
    }
    for col, label in metrics.items():
        if col in ne.columns:
            val = ne[col].mean()
            non_null = ne[col].notna().sum()
            print(f"  {label}: {val:.1f} (n={non_null})")

    # Slaves per ton
    if "SLAXIMP" in ne.columns and "TONNAGE" in ne.columns:
        valid = ne[ne["TONNAGE"].notna() & ne["SLAXIMP"].notna() & (ne["TONNAGE"] > 0)]
        if len(valid) > 0:
            spt = (valid["SLAXIMP"] / valid["TONNAGE"]).mean()
            print(f"  Avg slaves/ton: {spt:.2f} (n={len(valid)})")

    # ── UMAP position ─────────────────────────────────────────────────────
    print_section("UMAP Position")
    from tastdb_umap.feature_selection import assemble_matrix, get_feature_set_a
    X_a, names_a, idx_a = assemble_matrix(df, get_feature_set_a)
    df_a = df.iloc[idx_a].reset_index(drop=True)

    emb = load_cached_embedding("fset_a_default")
    if emb is not None:
        ne_mask_a = mask_any_port(df_a, NUEVA_ESPANA_PORTS) & (df_a["MJSELIMP"] == 41200)
        ne_points = emb[ne_mask_a.values]
        if len(ne_points) > 0:
            cx, cy = ne_points[:, 0].mean(), ne_points[:, 1].mean()
            print(f"  NE centroid: ({cx:.2f}, {cy:.2f})")
            print(f"  NE points in embedding: {len(ne_points)}")

            # SCC centroid for comparison
            scc_mask_a = df_a["MJSELIMP"] == 41200
            scc_points = emb[scc_mask_a.values]
            scx, scy = scc_points[:, 0].mean(), scc_points[:, 1].mean()
            dist = np.sqrt((cx - scx)**2 + (cy - scy)**2)
            print(f"  SCC centroid: ({scx:.2f}, {scy:.2f})")
            print(f"  Distance NE to SCC centroid: {dist:.2f}")

            # Overall centroid
            ocx, ocy = emb[:, 0].mean(), emb[:, 1].mean()
            dist_overall = np.sqrt((cx - ocx)**2 + (cy - ocy)**2)
            print(f"  Overall centroid: ({ocx:.2f}, {ocy:.2f})")
            print(f"  Distance NE to overall centroid: {dist_overall:.2f}")

            # UMAP spread
            spread = np.std(ne_points, axis=0)
            print(f"  NE spread (std): ({spread[0]:.2f}, {spread[1]:.2f})")
    else:
        print("  No cached embedding found; skipping UMAP analysis")

    print_section("DONE")


if __name__ == "__main__":
    main()
