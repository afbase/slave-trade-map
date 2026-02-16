"""HDBSCAN clustering and outlier analysis on UMAP embeddings."""

import hdbscan
import numpy as np
import pandas as pd
from sklearn.metrics import pairwise_distances_argmin_min


def cluster_hdbscan(embedding, min_cluster_size=50, min_samples=10):
    """Cluster a 2D embedding with HDBSCAN.

    Parameters
    ----------
    embedding : np.ndarray of shape (n_samples, 2)
    min_cluster_size : int
    min_samples : int

    Returns
    -------
    np.ndarray of int, shape (n_samples,)
        Cluster labels; -1 denotes noise/outliers.
    """
    clusterer = hdbscan.HDBSCAN(
        min_cluster_size=min_cluster_size,
        min_samples=min_samples,
    )
    labels = clusterer.fit_predict(embedding)
    n_clusters = len(set(labels)) - (1 if -1 in labels else 0)
    n_outliers = int((labels == -1).sum())
    print(
        f"[HDBSCAN] {n_clusters} clusters found, "
        f"{n_outliers} outliers ({n_outliers / len(labels) * 100:.1f}%)"
    )
    return labels


def get_cluster_stats(embedding, labels):
    """Compute per-cluster centroid and size.

    Parameters
    ----------
    embedding : np.ndarray of shape (n_samples, 2)
    labels : np.ndarray of int

    Returns
    -------
    dict[int, dict]
        {cluster_id: {"centroid": (x, y), "size": int}}
        Noise cluster (-1) is included if present.
    """
    stats = {}
    for cid in np.unique(labels):
        mask = labels == cid
        pts = embedding[mask]
        stats[int(cid)] = {
            "centroid": (float(pts[:, 0].mean()), float(pts[:, 1].mean())),
            "size": int(mask.sum()),
        }
    return stats


def detect_outliers(labels):
    """Return a boolean mask where the point is an outlier (label == -1).

    Parameters
    ----------
    labels : np.ndarray of int

    Returns
    -------
    np.ndarray of bool
    """
    return labels == -1


def get_top_outliers(embedding, labels, df, n=10):
    """Find the *n* outlier points furthest from any cluster centroid.

    Parameters
    ----------
    embedding : np.ndarray of shape (n_samples, 2)
    labels : np.ndarray of int
    df : pd.DataFrame
        Original dataframe (aligned by index with embedding rows).
    n : int

    Returns
    -------
    pd.DataFrame
        Columns: index, UMAP1, UMAP2, VOYAGEID (if available), distance_to_nearest_centroid
    """
    outlier_mask = detect_outliers(labels)
    if not outlier_mask.any():
        return pd.DataFrame(
            columns=["index", "UMAP1", "UMAP2", "VOYAGEID", "distance_to_nearest_centroid"]
        )

    # Compute cluster centroids (excluding noise)
    cluster_ids = [cid for cid in np.unique(labels) if cid != -1]
    if not cluster_ids:
        # No real clusters -- distance is undefined; return outliers sorted by
        # distance from global centroid instead.
        centroid = embedding.mean(axis=0, keepdims=True)
        centroids = centroid
    else:
        centroids = np.array(
            [embedding[labels == cid].mean(axis=0) for cid in cluster_ids]
        )

    outlier_indices = np.where(outlier_mask)[0]
    outlier_pts = embedding[outlier_indices]

    # Distance from each outlier to nearest centroid
    _, dists = pairwise_distances_argmin_min(outlier_pts, centroids)

    # Sort descending by distance, take top n
    top_order = np.argsort(-dists)[:n]
    top_idx = outlier_indices[top_order]
    top_dists = dists[top_order]

    result = pd.DataFrame({
        "index": top_idx,
        "UMAP1": embedding[top_idx, 0],
        "UMAP2": embedding[top_idx, 1],
        "distance_to_nearest_centroid": top_dists,
    })

    if "VOYAGEID" in df.columns:
        result["VOYAGEID"] = df.iloc[top_idx]["VOYAGEID"].values

    return result
