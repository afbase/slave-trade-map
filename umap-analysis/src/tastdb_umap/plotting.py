"""Publication-ready matplotlib/seaborn plots for UMAP analysis.

All figures are saved at 300 DPI to config.IMAGES_DIR by default.
"""

import warnings

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from matplotlib.lines import Line2D

from .config import (
    ABOLITION_YEAR_GENERAL,
    CHARTERED_COMPANIES,
    IMAGES_DIR,
    NAT_CODE_TO_NAME,
    NATIONALITY_COLORS,
    PLOT_DPI,
    PLOT_FIGSIZE,
    SCATTER_ALPHA,
    SCATTER_SIZE,
    TRADE_ERA_BINS,
    TRADE_ERA_LABELS,
    WARS,
)

# Clean style for all plots
plt.style.use("seaborn-v0_8-whitegrid")


# ── Helper ────────────────────────────────────────────────────────────────────


def _save_fig(fig, filename, output_dir=None):
    """Save figure to output_dir (or IMAGES_DIR) at PLOT_DPI, then close."""
    out = output_dir or IMAGES_DIR
    out.mkdir(parents=True, exist_ok=True)
    fig.tight_layout()
    fig.savefig(out / filename, dpi=PLOT_DPI, bbox_inches="tight")
    plt.close(fig)


def _resolve_label(code, column, value_labels):
    """Look up a human-readable label for *code* in *value_labels* dict."""
    if value_labels is None:
        return str(code)
    col_labels = value_labels.get(column, {})
    return col_labels.get(code, col_labels.get(str(int(code)) if isinstance(code, (float, np.floating)) else str(code), str(code)))


# ── 1. Trade route clusters ──────────────────────────────────────────────────


def plot_trade_route_clusters(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Color by MJSELIMP1 (broad disembarkation region)."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "MJSELIMP1"
    codes = df[col].dropna().unique()
    codes = np.sort(codes)
    cmap = plt.cm.get_cmap("tab20", len(codes))

    for i, code in enumerate(codes):
        mask = df[col] == code
        label = _resolve_label(code, col, value_labels)
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=cmap(i), label=label,
        )

    ax.set_title("UMAP: Trade Route Clusters by Disembarkation Region")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(
        bbox_to_anchor=(1.02, 1), loc="upper left", markerscale=4, fontsize=7,
        frameon=True,
    )
    _save_fig(fig, "01_umap_trade_route_clusters.png", output_dir)


# ── 2. Temporal evolution ─────────────────────────────────────────────────────


def plot_temporal_evolution(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Color by YEARAM using viridis colormap."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    mask = df["YEARAM"].notna()
    sc = ax.scatter(
        emb[mask, 0], emb[mask, 1],
        c=df.loc[mask, "YEARAM"], cmap="viridis",
        s=SCATTER_SIZE, alpha=SCATTER_ALPHA,
    )
    cbar = fig.colorbar(sc, ax=ax, pad=0.02)
    cbar.set_label("Year")
    ax.set_title("UMAP: Temporal Evolution of the Slave Trade")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    _save_fig(fig, "02_umap_temporal_evolution.png", output_dir)


# ── 3. National patterns ─────────────────────────────────────────────────────


def plot_national_patterns(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Color by NATINIMP mapped to nationality names and colors."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "NATINIMP"

    def _name(code):
        if value_labels:
            vl = value_labels.get(col, {})
            if code in vl:
                return vl[code]
        return NAT_CODE_TO_NAME.get(code, "Other")

    groups = {}
    for code in df[col].dropna().unique():
        name = _name(code)
        # Consolidate codes > 8 into "Other"
        if code > 8 and name not in NATIONALITY_COLORS:
            name = "Other"
        groups.setdefault(name, []).append(code)

    for name, codes in groups.items():
        mask = df[col].isin(codes)
        color = NATIONALITY_COLORS.get(name, "#999999")
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=color, label=name,
        )

    ax.set_title("UMAP: National Patterns in the Slave Trade")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(markerscale=4, fontsize=8, frameon=True)
    _save_fig(fig, "03_umap_national_patterns.png", output_dir)


# ── 4. Mortality landscape ───────────────────────────────────────────────────


def plot_mortality_landscape(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Color by VYMRTIMP using diverging RdYlGn_r."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "VYMRTIMP"
    mask = df[col].notna()
    sc = ax.scatter(
        emb[mask, 0], emb[mask, 1],
        c=df.loc[mask, col], cmap="RdYlGn_r",
        s=SCATTER_SIZE, alpha=SCATTER_ALPHA,
    )
    cbar = fig.colorbar(sc, ax=ax, pad=0.02)
    cbar.set_label("Mortality Rate (%)")
    ax.set_title("UMAP: Mortality Landscape")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    _save_fig(fig, "04_umap_mortality_landscape.png", output_dir)


# ── 5. African embarkation regions ───────────────────────────────────────────


def plot_african_embarkation(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Color by MAJBYIMP (specific African purchase region)."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "MAJBYIMP"
    codes = df[col].dropna().unique()
    codes = np.sort(codes)
    cmap = plt.cm.get_cmap("tab20", max(len(codes), 1))

    for i, code in enumerate(codes):
        mask = df[col] == code
        label = _resolve_label(code, col, value_labels)
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=cmap(i % 20), label=label,
        )

    ax.set_title("UMAP: African Embarkation Regions")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(
        bbox_to_anchor=(1.02, 1), loc="upper left", markerscale=4, fontsize=7,
        frameon=True,
    )
    _save_fig(fig, "05_umap_african_embarkation_regions.png", output_dir)


# ── 6. Chartered companies ───────────────────────────────────────────────────


def plot_chartered_companies(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Highlight chartered company voyages; independents in gray."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "COMPANY"

    # Plot independent (non-company) voyages first as background
    independent = df[col].isna() | (df[col] == "Independent")
    ax.scatter(
        emb[independent, 0], emb[independent, 1],
        s=SCATTER_SIZE, alpha=0.1, color="#d0d0d0", label="Independent",
    )

    for name, info in CHARTERED_COMPANIES.items():
        mask = df[col] == name
        if mask.sum() == 0:
            continue
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE + 2, alpha=0.6, color=info["color"], label=name,
        )

    ax.set_title("UMAP: Chartered Companies")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(
        bbox_to_anchor=(1.02, 1), loc="upper left", markerscale=4, fontsize=7,
        frameon=True,
    )
    _save_fig(fig, "06_umap_chartered_companies.png", output_dir)


# ── 7. War periods ───────────────────────────────────────────────────────────


def plot_war_periods(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Small-multiples grid: one subplot per war + peacetime."""
    col = "WAR_PERIOD"
    wars = list(WARS.keys())
    n_panels = len(wars) + 1  # +1 for Peacetime
    ncols = 4
    nrows = int(np.ceil(n_panels / ncols))
    fig, axes = plt.subplots(nrows, ncols, figsize=(16, 12))
    axes = axes.flatten()

    for i, war_name in enumerate(wars):
        ax = axes[i]
        ax.scatter(
            emb[:, 0], emb[:, 1],
            s=1, alpha=0.05, color="#d0d0d0",
        )
        mask = df[col] == war_name
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=0.6, color="#e41a1c",
        )
        ax.set_title(war_name, fontsize=7)
        ax.set_xticks([])
        ax.set_yticks([])

    # Peacetime panel
    ax_peace = axes[len(wars)]
    ax_peace.scatter(
        emb[:, 0], emb[:, 1],
        s=1, alpha=0.05, color="#d0d0d0",
    )
    peacetime = df[col] == "Peacetime"
    ax_peace.scatter(
        emb[peacetime, 0], emb[peacetime, 1],
        s=SCATTER_SIZE, alpha=0.3, color="#377eb8",
    )
    ax_peace.set_title("Peacetime", fontsize=7)
    ax_peace.set_xticks([])
    ax_peace.set_yticks([])

    # Hide unused axes
    for j in range(len(wars) + 1, len(axes)):
        axes[j].set_visible(False)

    fig.suptitle("UMAP: War Period Overlays", fontsize=14)
    _save_fig(fig, "07_umap_war_periods.png", output_dir)


# ── 8. Abolition comparison ──────────────────────────────────────────────────


def plot_abolition_comparison(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Side-by-side pre vs post 1807 abolition."""
    fig, (ax_pre, ax_post) = plt.subplots(1, 2, figsize=(20, 10))
    col = "NATINIMP"

    def _name(code):
        if value_labels:
            vl = value_labels.get(col, {})
            if code in vl:
                return vl[code]
        return NAT_CODE_TO_NAME.get(code, "Other")

    pre_mask = df["YEARAM"] < ABOLITION_YEAR_GENERAL
    post_mask = df["YEARAM"] >= ABOLITION_YEAR_GENERAL

    for ax, era_mask, title in [
        (ax_pre, pre_mask, f"Pre-Abolition (< {ABOLITION_YEAR_GENERAL})"),
        (ax_post, post_mask, f"Post-Abolition (>= {ABOLITION_YEAR_GENERAL})"),
    ]:
        # Background: all points in very light gray
        ax.scatter(
            emb[:, 0], emb[:, 1],
            s=1, alpha=0.03, color="#d0d0d0",
        )
        for code in sorted(df.loc[era_mask, col].dropna().unique()):
            name = _name(code)
            if code > 8 and name not in NATIONALITY_COLORS:
                name = "Other"
            color = NATIONALITY_COLORS.get(name, "#999999")
            m = era_mask & (df[col] == code)
            ax.scatter(
                emb[m, 0], emb[m, 1],
                s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=color, label=name,
            )
        ax.set_title(title)
        ax.set_xlabel("UMAP 1")
        ax.set_ylabel("UMAP 2")
        # Deduplicate legend labels
        handles, labels = ax.get_legend_handles_labels()
        seen = {}
        unique_h, unique_l = [], []
        for h, l in zip(handles, labels):
            if l not in seen:
                seen[l] = True
                unique_h.append(h)
                unique_l.append(l)
        ax.legend(unique_h, unique_l, markerscale=4, fontsize=7, frameon=True)

    fig.suptitle("Pre vs Post Abolition (1807)", fontsize=14)
    _save_fig(fig, "08_umap_abolition_comparison.png", output_dir)


# ── 9. HDBSCAN clusters ──────────────────────────────────────────────────────


def plot_hdbscan_clusters(emb, labels, output_dir=None, value_labels=None, **kwargs):
    """Color by HDBSCAN cluster label; noise in gray. Annotate centroids."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    unique_labels = np.unique(labels)
    cluster_labels = unique_labels[unique_labels >= 0]
    cmap = plt.cm.get_cmap("tab20", max(len(cluster_labels), 1))

    # Noise points
    noise = labels == -1
    if noise.sum() > 0:
        ax.scatter(
            emb[noise, 0], emb[noise, 1],
            s=1, alpha=0.1, color="#d0d0d0", label="Noise",
        )

    for i, cl in enumerate(cluster_labels):
        mask = labels == cl
        color = cmap(i % 20)
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=color, label=f"Cluster {cl}",
        )
        # Annotate centroid
        cx, cy = emb[mask, 0].mean(), emb[mask, 1].mean()
        ax.annotate(
            str(cl), (cx, cy),
            fontsize=8, fontweight="bold", ha="center", va="center",
            bbox=dict(boxstyle="round,pad=0.2", fc="white", ec="gray", alpha=0.8),
        )

    ax.set_title("UMAP: HDBSCAN Clusters")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(
        bbox_to_anchor=(1.02, 1), loc="upper left", markerscale=4, fontsize=7,
        frameon=True, ncol=2 if len(cluster_labels) > 15 else 1,
    )
    _save_fig(fig, "09_umap_hdbscan_clusters.png", output_dir)


# ── 10. Outliers ──────────────────────────────────────────────────────────────


def plot_outliers(emb, labels, df, output_dir=None, top_n=10, value_labels=None, **kwargs):
    """Highlight noise/outliers in red; annotate the top_n furthest."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)

    normal = labels >= 0
    outlier = labels == -1

    ax.scatter(
        emb[normal, 0], emb[normal, 1],
        s=1, alpha=0.1, color="#d0d0d0",
    )
    ax.scatter(
        emb[outlier, 0], emb[outlier, 1],
        s=SCATTER_SIZE + 1, alpha=0.5, color="#e41a1c", label="Outliers",
    )

    # Find top_n furthest outliers from centroid of non-outlier points
    if outlier.sum() > 0:
        center = emb[normal].mean(axis=0) if normal.sum() > 0 else emb.mean(axis=0)
        dists = np.linalg.norm(emb[outlier] - center, axis=1)
        outlier_idx = np.where(outlier)[0]
        top_idx = outlier_idx[np.argsort(dists)[-top_n:]]

        vid_col = "VOYAGEID" if "VOYAGEID" in df.columns else df.columns[0]
        for idx in top_idx:
            ax.annotate(
                str(df.iloc[idx][vid_col]),
                (emb[idx, 0], emb[idx, 1]),
                fontsize=6, color="#e41a1c", alpha=0.8,
            )

    ax.set_title("UMAP: Outlier Voyages")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(markerscale=4, fontsize=8, frameon=True)
    _save_fig(fig, "10_umap_outliers.png", output_dir)


# ── 11. Resistance events ────────────────────────────────────────────────────


def plot_resistance_events(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Highlight voyages with resistance > 0."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    col = "RESISTANCE"

    # Background
    ax.scatter(
        emb[:, 0], emb[:, 1],
        s=1, alpha=0.1, color="#d0d0d0",
    )

    if col in df.columns:
        resist = df[col].fillna(0) > 0
        ax.scatter(
            emb[resist, 0], emb[resist, 1],
            s=SCATTER_SIZE + 3, alpha=0.6, color="#e41a1c", label="Resistance",
        )
    else:
        warnings.warn(f"Column '{col}' not found in DataFrame; skipping highlights.")

    ax.set_title("UMAP: Resistance Events")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(markerscale=4, fontsize=8, frameon=True)
    _save_fig(fig, "11_umap_resistance_events.png", output_dir)


# ── 12. Trans-Atlantic vs Intra-American ─────────────────────────────────────


def plot_transatlantic_vs_intraamerican(emb, dataset_labels, output_dir=None, value_labels=None, **kwargs):
    """Two-color split by dataset_labels array of 'Trans-Atlantic' / 'Intra-American'."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)
    labels_arr = np.asarray(dataset_labels)
    colors = {"Trans-Atlantic": "#377eb8", "Intra-American": "#e41a1c"}

    for label, color in colors.items():
        mask = labels_arr == label
        if mask.sum() == 0:
            continue
        ax.scatter(
            emb[mask, 0], emb[mask, 1],
            s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=color, label=label,
        )

    ax.set_title("UMAP: Trans-Atlantic vs Intra-American")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    ax.legend(markerscale=4, fontsize=8, frameon=True)
    _save_fig(fig, "12_umap_transatlantic_vs_intraamerican.png", output_dir)


# ── 13. Temporal small multiples ──────────────────────────────────────────────


def plot_temporal_small_multiples(emb, df, output_dir=None, value_labels=None, **kwargs):
    """4x4 grid of 25-year eras, points colored by NATINIMP."""
    n_eras = len(TRADE_ERA_LABELS)
    ncols = 4
    nrows = int(np.ceil(n_eras / ncols))
    fig, axes = plt.subplots(nrows, ncols, figsize=(20, 20))
    axes = axes.flatten()

    col = "NATINIMP"

    def _name(code):
        if value_labels:
            vl = value_labels.get(col, {})
            if code in vl:
                return vl[code]
        return NAT_CODE_TO_NAME.get(code, "Other")

    for i, (era_label, lo, hi) in enumerate(
        zip(TRADE_ERA_LABELS, TRADE_ERA_BINS[:-1], TRADE_ERA_BINS[1:])
    ):
        ax = axes[i]
        # Background
        ax.scatter(emb[:, 0], emb[:, 1], s=0.5, alpha=0.03, color="#d0d0d0")

        era_mask = (df["YEARAM"] >= lo) & (df["YEARAM"] < hi)
        for code in sorted(df.loc[era_mask, col].dropna().unique()):
            name = _name(code)
            if code > 8 and name not in NATIONALITY_COLORS:
                name = "Other"
            color = NATIONALITY_COLORS.get(name, "#999999")
            m = era_mask & (df[col] == code)
            ax.scatter(
                emb[m, 0], emb[m, 1],
                s=SCATTER_SIZE, alpha=SCATTER_ALPHA, color=color,
            )
        ax.set_title(era_label, fontsize=8)
        ax.set_xticks([])
        ax.set_yticks([])

    # Hide unused axes
    for j in range(n_eras, len(axes)):
        axes[j].set_visible(False)

    # Shared legend
    handles = [
        Line2D([0], [0], marker="o", color="w", markerfacecolor=c, markersize=6, label=n)
        for n, c in NATIONALITY_COLORS.items()
    ]
    fig.legend(
        handles=handles, loc="lower center", ncol=4, fontsize=8,
        frameon=True, bbox_to_anchor=(0.5, 0.01),
    )

    fig.suptitle("UMAP: Temporal Small Multiples (25-year eras, colored by nationality)", fontsize=14)
    _save_fig(fig, "13_umap_temporal_small_multiples.png", output_dir)


# ── 14. Company timeline / Gantt chart ────────────────────────────────────────


def plot_company_timeline_gantt(df, output_dir=None, value_labels=None, **kwargs):
    """Top: Gantt-style horizontal bars per company. Bottom: stacked area of volume."""
    fig, (ax_gantt, ax_area) = plt.subplots(2, 1, figsize=(16, 10), height_ratios=[1, 1.2])

    companies = list(CHARTERED_COMPANIES.keys())

    # ── Gantt bars ────────────────────────────────────────────────────────────
    for i, name in enumerate(companies):
        info = CHARTERED_COMPANIES[name]
        ax_gantt.barh(
            i, info["end"] - info["start"], left=info["start"],
            color=info["color"], edgecolor="white", height=0.6,
        )
    ax_gantt.set_yticks(range(len(companies)))
    ax_gantt.set_yticklabels(companies, fontsize=7)
    ax_gantt.set_xlabel("Year")
    ax_gantt.set_title("Chartered Company Active Periods")
    ax_gantt.invert_yaxis()

    # ── Stacked area: voyage volume by company over time ──────────────────────
    col = "COMPANY"
    if col in df.columns and "YEARAM" in df.columns:
        year_range = np.arange(
            int(df["YEARAM"].min()) // 5 * 5,
            int(df["YEARAM"].max()) + 5,
            5,
        )
        stack_data = {}
        for name in companies:
            mask = df[col] == name
            if mask.sum() == 0:
                stack_data[name] = np.zeros(len(year_range) - 1)
                continue
            counts, _ = np.histogram(df.loc[mask, "YEARAM"].dropna(), bins=year_range)
            stack_data[name] = counts

        centers = (year_range[:-1] + year_range[1:]) / 2
        arrays = [stack_data[n] for n in companies]
        colors = [CHARTERED_COMPANIES[n]["color"] for n in companies]
        ax_area.stackplot(centers, *arrays, labels=companies, colors=colors, alpha=0.8)
        ax_area.set_xlabel("Year")
        ax_area.set_ylabel("Voyage Count (5-yr bins)")
        ax_area.set_title("Voyage Volume by Chartered Company Over Time")
        ax_area.legend(fontsize=6, loc="upper right", frameon=True)
    else:
        ax_area.text(
            0.5, 0.5, "COMPANY or YEARAM column not found",
            transform=ax_area.transAxes, ha="center",
        )

    _save_fig(fig, "14_company_timeline_gantt.png", output_dir)


# ── 15. Centroid trajectories ─────────────────────────────────────────────────


def plot_centroid_trajectories(emb, df, output_dir=None, value_labels=None, **kwargs):
    """Arrow paths connecting nationality centroids through 25-year eras."""
    fig, ax = plt.subplots(figsize=PLOT_FIGSIZE)

    # Background
    ax.scatter(emb[:, 0], emb[:, 1], s=0.5, alpha=0.05, color="#d0d0d0")

    col = "NATINIMP"
    # Use top nationalities (codes 1-8)
    top_nats = sorted(NAT_CODE_TO_NAME.keys())

    for code in top_nats:
        name = NAT_CODE_TO_NAME[code]
        color = NATIONALITY_COLORS.get(name, "#999999")

        centroids = []
        for lo, hi in zip(TRADE_ERA_BINS[:-1], TRADE_ERA_BINS[1:]):
            mask = (df[col] == code) & (df["YEARAM"] >= lo) & (df["YEARAM"] < hi)
            if mask.sum() < 5:  # skip eras with too few points
                continue
            cx = emb[mask, 0].mean()
            cy = emb[mask, 1].mean()
            centroids.append((cx, cy))

        if len(centroids) < 2:
            continue

        centroids = np.array(centroids)
        # Draw arrows between consecutive centroids
        for j in range(len(centroids) - 1):
            dx = centroids[j + 1, 0] - centroids[j, 0]
            dy = centroids[j + 1, 1] - centroids[j, 1]
            ax.annotate(
                "",
                xy=(centroids[j + 1, 0], centroids[j + 1, 1]),
                xytext=(centroids[j, 0], centroids[j, 1]),
                arrowprops=dict(
                    arrowstyle="->", color=color, lw=1.5,
                    connectionstyle="arc3,rad=0.1",
                ),
            )

        # Mark start and end
        ax.scatter(centroids[0, 0], centroids[0, 1], s=40, color=color, marker="o", zorder=5)
        ax.scatter(centroids[-1, 0], centroids[-1, 1], s=40, color=color, marker="s", zorder=5)

    # Legend
    handles = [
        Line2D([0], [0], color=NATIONALITY_COLORS.get(NAT_CODE_TO_NAME[c], "#999999"),
               lw=2, label=NAT_CODE_TO_NAME[c])
        for c in top_nats
    ]
    ax.legend(handles=handles, fontsize=8, frameon=True)
    ax.set_title("UMAP: Centroid Trajectories by Nationality (25-year eras)")
    ax.set_xlabel("UMAP 1")
    ax.set_ylabel("UMAP 2")
    _save_fig(fig, "15_umap_centroid_trajectories.png", output_dir)
