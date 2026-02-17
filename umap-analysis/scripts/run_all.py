#!/usr/bin/env python3
"""Main entry point: run the full UMAP analysis pipeline."""

import logging
import sys
import time
from pathlib import Path

# Ensure the package is importable
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "src"))

from tastdb_umap import config
from tastdb_umap.data_loader import (
    load_tastdb,
    load_iam,
    load_value_labels,
    get_nationality_labels,
)
from tastdb_umap.preprocessing import preprocess_pipeline, normalize_features, filter_rows, engineer_features, add_missingness_indicators, impute_missing
from tastdb_umap.feature_selection import assemble_matrix, get_feature_set_a, get_feature_set_b, get_feature_set_c, get_feature_set_d
from tastdb_umap.umap_runner import fit_umap, fit_umap_3d, cache_embedding, load_cached_embedding
from tastdb_umap.clustering import cluster_hdbscan, get_cluster_stats, detect_outliers, get_top_outliers
from tastdb_umap.historical_overlay import (
    tag_chartered_companies,
    tag_war_periods,
    tag_abolition_era,
    compute_centroid_trajectories,
    get_company_timeline_data,
)
from tastdb_umap.plotting import (
    plot_trade_route_clusters,
    plot_temporal_evolution,
    plot_national_patterns,
    plot_mortality_landscape,
    plot_african_embarkation,
    plot_chartered_companies,
    plot_war_periods,
    plot_abolition_comparison,
    plot_hdbscan_clusters,
    plot_outliers,
    plot_resistance_events,
    plot_transatlantic_vs_intraamerican,
    plot_temporal_small_multiples,
    plot_company_timeline_gantt,
    plot_centroid_trajectories,
    plot_nueva_espana_highlight,
)
from tastdb_umap.interactive_plots import create_3d_exploration, create_temporal_slider
from tastdb_umap.blog_generator import generate_blog_post, copy_to_slavecodes

import numpy as np
import pandas as pd

logging.basicConfig(level=logging.INFO, format="%(levelname)s %(name)s: %(message)s")
logger = logging.getLogger("run_all")


def ensure_dirs():
    """Create output directories if they don't exist."""
    for d in [config.IMAGES_DIR, config.INTERACTIVE_DIR, config.OUTPUT_DIR]:
        d.mkdir(parents=True, exist_ok=True)
        print(f"  Output dir: {d}")


def step(msg):
    """Print a step header."""
    print(f"\n{'='*70}\n  {msg}\n{'='*70}")


def main():
    t0 = time.time()

    step("1. Creating output directories")
    ensure_dirs()

    # ── Load data ─────────────────────────────────────────────────────────
    step("2. Loading data")
    tastdb = load_tastdb()
    iam = load_iam()
    value_labels = load_value_labels()
    nat_labels = get_nationality_labels(value_labels)

    # ── Preprocess TASTDB ─────────────────────────────────────────────────
    step("3. Preprocessing trans-Atlantic data")
    df, original_idx = preprocess_pipeline(tastdb)
    print(f"  After preprocessing: {len(df):,} rows, {df.shape[1]} cols")

    # ── Tag historical overlays ───────────────────────────────────────────
    step("4. Tagging historical overlays")
    df = tag_chartered_companies(df)
    print(f"  Company tags: {df['COMPANY'].nunique()} unique values")
    df = tag_war_periods(df)
    print(f"  War period tags: {df['WAR_PERIOD'].nunique()} unique values")
    df = tag_abolition_era(df)
    print(f"  Abolition era tags: {df['ABOLITION_ERA'].value_counts().to_dict()}")

    # ── Feature Set A + UMAP ──────────────────────────────────────────────
    step("5. Feature Set A: Full Voyage Profile")
    X_a, names_a, idx_a = assemble_matrix(df, get_feature_set_a)
    print(f"  Feature matrix: {X_a.shape}")

    emb_a = load_cached_embedding("fset_a_default")
    if emb_a is None:
        emb_a = fit_umap(X_a, config_name="default")
        cache_embedding(emb_a, "fset_a_default")
    # Align df to valid rows used in feature set A
    df_a = df.iloc[idx_a].reset_index(drop=True)

    # ── Feature Set B (cosine) ────────────────────────────────────────────
    step("6. Feature Set B: Trade Route Geography (cosine)")
    X_b, names_b, idx_b = assemble_matrix(df, get_feature_set_b)
    print(f"  Feature matrix: {X_b.shape}")
    emb_b = load_cached_embedding("fset_b_cosine")
    if emb_b is None:
        emb_b = fit_umap(X_b, config_name="cosine")
        cache_embedding(emb_b, "fset_b_cosine")

    # ── Feature Set C ─────────────────────────────────────────────────────
    step("7. Feature Set C: Mortality & Demographics")
    X_c, names_c, idx_c = assemble_matrix(df, get_feature_set_c)
    print(f"  Feature matrix: {X_c.shape}")
    emb_c = load_cached_embedding("fset_c_default")
    if emb_c is None:
        emb_c = fit_umap(X_c, config_name="default")
        cache_embedding(emb_c, "fset_c_default")

    # ── Feature Set D ─────────────────────────────────────────────────────
    step("8. Feature Set D: Temporal + National")
    X_d, names_d, idx_d = assemble_matrix(df, get_feature_set_d)
    print(f"  Feature matrix: {X_d.shape}")
    emb_d = load_cached_embedding("fset_d_default")
    if emb_d is None:
        emb_d = fit_umap(X_d, config_name="default")
        cache_embedding(emb_d, "fset_d_default")

    # ── HDBSCAN clustering on Feature Set A ───────────────────────────────
    step("9. HDBSCAN clustering")
    labels = cluster_hdbscan(emb_a)
    cluster_stats = get_cluster_stats(emb_a, labels)
    outlier_mask = detect_outliers(labels)
    top_outliers = get_top_outliers(emb_a, labels, df_a)
    n_clusters = len([k for k in cluster_stats if k >= 0])
    n_outliers = int(outlier_mask.sum())
    print(f"  Clusters: {n_clusters}, Outliers: {n_outliers}")

    # ── 3D UMAP for interactive ───────────────────────────────────────────
    step("10. 3D UMAP for interactive exploration")
    emb_3d = load_cached_embedding("fset_a_3d")
    if emb_3d is None:
        emb_3d = fit_umap_3d(X_a, config_name="default")
        cache_embedding(emb_3d, "fset_a_3d")

    # ── Preprocess IAm + combined UMAP ────────────────────────────────────
    step("11. Processing Intra-American data for combined embedding")
    # Minimal preprocessing of IAm
    iam_df = iam.copy()
    # Ensure key columns exist (IAm uses lowercase names that were uppercased)
    if "YEARAM" not in iam_df.columns and "YEARDEP" in iam_df.columns:
        iam_df["YEARAM"] = iam_df["YEARDEP"]

    # Coerce key numeric columns (IAm CSV has some string values like ' ')
    for col in ["YEARAM", "SLAXIMP", "SLAMIMP", "VYMRTIMP"]:
        if col in iam_df.columns:
            iam_df[col] = pd.to_numeric(iam_df[col], errors="coerce")

    # Engineer basic features for IAm
    if "SLAXIMP" in iam_df.columns:
        iam_df["LOG_SLAXIMP"] = np.log1p(iam_df["SLAXIMP"])
    if "SLAMIMP" in iam_df.columns:
        iam_df["LOG_SLAMIMP"] = np.log1p(iam_df["SLAMIMP"])

    # For combined embedding, use overlapping features only
    # Find shared columns between Feature Set A cols and IAm
    shared_continuous = [c for c in ["YEARAM", "LOG_SLAXIMP", "LOG_SLAMIMP", "VYMRTIMP"] if c in df_a.columns and c in iam_df.columns]
    print(f"  Shared continuous features: {shared_continuous}")

    if len(shared_continuous) >= 2:
        combined_tastdb = df_a[shared_continuous].fillna(0).values.astype(np.float64)
        combined_iam = iam_df[shared_continuous].fillna(0).values.astype(np.float64)
        combined_X = np.vstack([combined_tastdb, combined_iam])
        dataset_labels = np.array(
            ["Trans-Atlantic"] * len(combined_tastdb) + ["Intra-American"] * len(combined_iam)
        )
        print(f"  Combined matrix: {combined_X.shape}")

        emb_combined = load_cached_embedding("combined_default")
        if emb_combined is None:
            emb_combined = fit_umap(combined_X, config_name="default")
            cache_embedding(emb_combined, "combined_default")
    else:
        emb_combined = None
        dataset_labels = None
        print("  WARNING: Not enough shared features for combined embedding")

    # ── Generate all 15 static plots ──────────────────────────────────────
    step("12. Generating static plots")

    plot_funcs = [
        ("01 Trade route clusters", lambda: plot_trade_route_clusters(emb_a, df_a, value_labels=value_labels)),
        ("02 Temporal evolution", lambda: plot_temporal_evolution(emb_a, df_a, value_labels=value_labels)),
        ("03 National patterns", lambda: plot_national_patterns(emb_a, df_a, value_labels=value_labels)),
        ("04 Mortality landscape", lambda: plot_mortality_landscape(emb_a, df_a, value_labels=value_labels)),
        ("05 African embarkation", lambda: plot_african_embarkation(emb_a, df_a, value_labels=value_labels)),
        ("06 Chartered companies", lambda: plot_chartered_companies(emb_a, df_a, value_labels=value_labels)),
        ("07 War periods", lambda: plot_war_periods(emb_a, df_a, value_labels=value_labels)),
        ("08 Abolition comparison", lambda: plot_abolition_comparison(emb_a, df_a, value_labels=value_labels)),
        ("09 HDBSCAN clusters", lambda: plot_hdbscan_clusters(emb_a, labels, value_labels=value_labels)),
        ("10 Outliers", lambda: plot_outliers(emb_a, labels, df_a, value_labels=value_labels)),
        ("11 Resistance events", lambda: plot_resistance_events(emb_a, df_a, value_labels=value_labels)),
        ("12 Trans-Atlantic vs Intra-American", lambda: (
            plot_transatlantic_vs_intraamerican(emb_combined, dataset_labels, value_labels=value_labels)
            if emb_combined is not None else print("  SKIP: No combined embedding")
        )),
        ("13 Temporal small multiples", lambda: plot_temporal_small_multiples(emb_a, df_a, value_labels=value_labels)),
        ("14 Company timeline Gantt", lambda: plot_company_timeline_gantt(df_a, value_labels=value_labels)),
        ("15 Centroid trajectories", lambda: plot_centroid_trajectories(emb_a, df_a, value_labels=value_labels)),
        ("16 Nueva Espana highlight", lambda: plot_nueva_espana_highlight(emb_a, df_a, value_labels=value_labels)),
    ]

    for name, func in plot_funcs:
        try:
            print(f"  Plotting: {name}...")
            func()
            print(f"  Done: {name}")
        except Exception as e:
            print(f"  ERROR on {name}: {e}")
            import traceback
            traceback.print_exc()

    # ── Interactive plots ─────────────────────────────────────────────────
    step("13. Generating interactive HTML plots")

    try:
        path_3d = create_3d_exploration(emb_3d, df_a)
        print(f"  3D exploration: {path_3d}")
    except Exception as e:
        print(f"  ERROR on 3D exploration: {e}")

    try:
        path_slider = create_temporal_slider(emb_a, df_a)
        print(f"  Temporal slider: {path_slider}")
    except Exception as e:
        print(f"  ERROR on temporal slider: {e}")

    # ── Blog post ─────────────────────────────────────────────────────────
    step("14. Generating blog post")

    pre_abol = (df_a["YEARAM"] < config.ABOLITION_YEAR_GENERAL).sum() if "YEARAM" in df_a.columns else 0
    post_abol = (df_a["YEARAM"] >= config.ABOLITION_YEAR_GENERAL).sum() if "YEARAM" in df_a.columns else 0

    # Build top cluster info string
    top_clusters = sorted(
        [(k, v) for k, v in cluster_stats.items() if k >= 0],
        key=lambda x: x[1]["size"],
        reverse=True,
    )[:3]
    top_info = ", ".join(
        f"Cluster {k} ({v['size']:,} voyages)" for k, v in top_clusters
    ) if top_clusters else "N/A"

    stats = {
        "total_voyages": len(df_a),
        "iam_voyages": len(iam_df),
        "clusters_found": n_clusters,
        "outlier_count": n_outliers,
        "outlier_pct": 100 * n_outliers / len(df_a) if len(df_a) > 0 else 0,
        "date_range_start": int(df_a["YEARAM"].min()) if "YEARAM" in df_a.columns else "?",
        "date_range_end": int(df_a["YEARAM"].max()) if "YEARAM" in df_a.columns else "?",
        "features_a_count": len(names_a),
        "top_cluster_info": f"The three largest clusters are: {top_info}.",
        "pre_abolition_count": int(pre_abol),
        "post_abolition_count": int(post_abol),
    }

    try:
        blog_path = generate_blog_post(stats)
        print(f"  Blog post: {blog_path}")
    except Exception as e:
        print(f"  ERROR on blog post: {e}")

    # ── Copy to slavecodes if available ───────────────────────────────────
    step("15. Copying to slavecodes_org (if available)")
    if copy_to_slavecodes():
        print("  Copied blog + images to slavecodes_org")
    else:
        print("  slavecodes_org directory not found; skipping copy")

    # ── Summary ───────────────────────────────────────────────────────────
    elapsed = time.time() - t0
    step("DONE")
    print(f"  Total time: {elapsed:.1f}s ({elapsed/60:.1f} min)")
    print(f"  Images: {len(list(config.IMAGES_DIR.glob('*.png')))} PNGs in {config.IMAGES_DIR}")
    print(f"  Interactive: {len(list(config.INTERACTIVE_DIR.glob('*.html')))} HTMLs in {config.INTERACTIVE_DIR}")
    print(f"  Blog: {len(list(config.OUTPUT_DIR.glob('*.md')))} markdown files in {config.OUTPUT_DIR}")
    print(f"  Cached embeddings: {len(list(config.IMAGES_DIR.glob('*.npy')))} .npy files")


if __name__ == "__main__":
    main()
