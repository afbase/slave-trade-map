"""Historical overlay utilities: chartered-company tagging, war periods,
abolition eras, centroid computations, convex hulls, and timeline aggregation."""

import numpy as np
import pandas as pd
from scipy.spatial import ConvexHull

from . import config


# ---------------------------------------------------------------------------
# Tagging helpers
# ---------------------------------------------------------------------------

def tag_chartered_companies(df: pd.DataFrame) -> pd.DataFrame:
    """Add a COMPANY column to *df*.

    For each voyage, check if ``NATINIMP`` matches any company's *nat_codes*
    and ``YEARAM`` falls within ``[start, end]``.  When multiple companies
    match, their names are joined with ``" / "``.  Non-matching rows receive
    ``"Independent"``.
    """
    df = df.copy()
    labels = [None] * len(df)

    natinimp = df["NATINIMP"].values
    yearam = df["YEARAM"].values

    for idx in range(len(df)):
        nat = natinimp[idx]
        year = yearam[idx]
        if pd.isna(nat) or pd.isna(year):
            continue
        matches = []
        for name, info in config.CHARTERED_COMPANIES.items():
            if int(nat) in info["nat_codes"] and info["start"] <= year <= info["end"]:
                matches.append(name)
        if matches:
            labels[idx] = " / ".join(matches)

    df["COMPANY"] = pd.array(labels, dtype="object")
    df["COMPANY"] = df["COMPANY"].fillna("Independent")
    return df


def tag_war_periods(df: pd.DataFrame) -> pd.DataFrame:
    """Add a WAR_PERIOD column to *df*.

    Check if ``YEARAM`` falls within any war's ``(start, end)`` range from
    :data:`config.WARS`.  If multiple wars overlap (rare), pick the first.
    Non-matching rows get ``"Peacetime"``.
    """
    df = df.copy()
    labels = [None] * len(df)

    yearam = df["YEARAM"].values
    war_items = list(config.WARS.items())

    for idx in range(len(df)):
        year = yearam[idx]
        if pd.isna(year):
            continue
        for name, (start, end) in war_items:
            if start <= year <= end:
                labels[idx] = name
                break

    df["WAR_PERIOD"] = pd.array(labels, dtype="object")
    df["WAR_PERIOD"] = df["WAR_PERIOD"].fillna("Peacetime")
    return df


def tag_abolition_era(df: pd.DataFrame) -> pd.DataFrame:
    """Add an ABOLITION_ERA column to *df*.

    Uses nation-specific abolition dates mapped through ``NATINIMP``:

    * 3 (GB) -> 1807
    * 5 (USA) -> 1808
    * 2 (Portugal/Brazil) -> 1831
    * 1 (Spain) -> 1817
    * 4 (Netherlands) -> 1814
    * 6 (France) -> 1815
    * 7 (Denmark) -> 1803
    * Default -> 1807

    Sets ``"Pre-Abolition"`` if ``YEARAM`` < nation's year, else
    ``"Post-Abolition"``.
    """
    nat_to_year = {
        7: 1807,   # Great Britain
        9: 1808,   # U.S.A.
        6: 1831,   # Portugal/Brazil
        3: 1817,   # Spain
        8: 1814,   # Netherlands
        10: 1815,  # France
        15: 1803,  # Denmark
    }
    default_year = 1807

    df = df.copy()
    labels = []

    for _, row in df.iterrows():
        nat = row.get("NATINIMP")
        year = row.get("YEARAM")
        if pd.isna(nat) or pd.isna(year):
            labels.append(np.nan)
            continue
        abolition_year = nat_to_year.get(int(nat), default_year)
        labels.append("Pre-Abolition" if year < abolition_year else "Post-Abolition")

    df["ABOLITION_ERA"] = labels
    return df


# ---------------------------------------------------------------------------
# Centroid and geometry helpers
# ---------------------------------------------------------------------------

def compute_centroids_by_group(
    embedding: np.ndarray,
    group_labels: np.ndarray | pd.Series,
) -> dict:
    """Return ``{group: (mean_x, mean_y)}`` for each unique group label."""
    group_labels = np.asarray(group_labels)
    centroids: dict = {}
    for label in np.unique(group_labels):
        mask = group_labels == label
        centroids[label] = (
            float(embedding[mask, 0].mean()),
            float(embedding[mask, 1].mean()),
        )
    return centroids


def compute_centroid_trajectories(
    embedding: np.ndarray,
    df: pd.DataFrame,
    group_col: str,
    time_col: str = "YEARAM",
    n_bins: int = 15,
) -> pd.DataFrame:
    """Compute group centroids over binned time periods.

    Parameters
    ----------
    embedding : (N, 2) array
    df : DataFrame with *group_col* and *time_col*
    group_col : column name for group labels
    time_col : column name for the time variable
    n_bins : number of equal-width time bins

    Returns
    -------
    DataFrame with columns: group, time_bin, centroid_x, centroid_y, count
    """
    time_vals = df[time_col].values
    bin_edges = np.linspace(np.nanmin(time_vals), np.nanmax(time_vals), n_bins + 1)
    bin_indices = np.digitize(time_vals, bin_edges[1:-1])  # 0 .. n_bins-1

    groups = df[group_col].values
    records = []

    for grp in np.unique(groups[~pd.isna(groups)]):
        for b in range(n_bins):
            mask = (groups == grp) & (bin_indices == b)
            count = int(mask.sum())
            if count == 0:
                continue
            cx = float(embedding[mask, 0].mean())
            cy = float(embedding[mask, 1].mean())
            bin_center = (bin_edges[b] + bin_edges[b + 1]) / 2.0
            records.append({
                "group": grp,
                "time_bin": bin_center,
                "centroid_x": cx,
                "centroid_y": cy,
                "count": count,
            })

    return pd.DataFrame(records, columns=["group", "time_bin", "centroid_x", "centroid_y", "count"])


def compute_convex_hulls(
    embedding: np.ndarray,
    group_labels: np.ndarray | pd.Series,
) -> dict:
    """Compute convex hulls for each group with >= 3 points.

    Returns
    -------
    dict : ``{group: hull_vertices_array}`` where *hull_vertices_array* is an
        (M, 2) array of the hull boundary points (closed polygon).
    """
    group_labels = np.asarray(group_labels)
    hulls: dict = {}
    for label in np.unique(group_labels):
        mask = group_labels == label
        pts = embedding[mask]
        if len(pts) < 3:
            continue
        hull = ConvexHull(pts)
        vertices = hull.vertices
        # Close the polygon by appending the first vertex
        closed = np.concatenate([vertices, [vertices[0]]])
        hulls[label] = pts[closed]
    return hulls


# ---------------------------------------------------------------------------
# Timeline aggregation
# ---------------------------------------------------------------------------

def get_company_timeline_data(df: pd.DataFrame) -> pd.DataFrame:
    """Aggregate voyage counts and total slaves per company per year.

    Expects *df* to already have a ``COMPANY`` column (see
    :func:`tag_chartered_companies`).

    Returns
    -------
    DataFrame with columns: COMPANY, YEARAM, voyage_count, total_slaves
    """
    grouped = (
        df.groupby(["COMPANY", "YEARAM"], sort=True)
        .agg(
            voyage_count=("VOYAGEID" if "VOYAGEID" in df.columns else "COMPANY", "size"),
            total_slaves=("SLAXIMP", "sum"),
        )
        .reset_index()
    )
    # Ensure column order
    return grouped[["COMPANY", "YEARAM", "voyage_count", "total_slaves"]]
