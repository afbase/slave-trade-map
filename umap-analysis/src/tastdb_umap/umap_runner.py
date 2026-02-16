"""UMAP dimensionality reduction with multiple configurations and caching."""

import numpy as np
import umap

from . import config


def fit_umap(X, config_name="default", n_components=2):
    """Fit UMAP on feature matrix X using a named configuration.

    Parameters
    ----------
    X : array-like of shape (n_samples, n_features)
    config_name : str
        Key into config.UMAP_CONFIGS.
    n_components : int
        Dimensionality of the embedding (2 or 3).

    Returns
    -------
    np.ndarray of shape (n_samples, n_components)
    """
    params = config.UMAP_CONFIGS[config_name]
    print(
        f"[UMAP] Fitting config='{config_name}' | "
        f"n_neighbors={params['n_neighbors']}, min_dist={params['min_dist']}, "
        f"metric={params['metric']}, n_components={n_components}, "
        f"n_samples={X.shape[0]}, n_features={X.shape[1]}"
    )

    reducer = umap.UMAP(
        n_neighbors=params["n_neighbors"],
        min_dist=params["min_dist"],
        metric=params["metric"],
        n_components=n_components,
        random_state=config.RANDOM_SEED,
    )
    embedding = reducer.fit_transform(X)
    print(f"[UMAP] Done config='{config_name}' -> embedding shape {embedding.shape}")
    return embedding


def fit_umap_3d(X, config_name="default"):
    """Convenience wrapper for 3-component UMAP."""
    return fit_umap(X, config_name=config_name, n_components=3)


def run_all_configs(X, feature_set_name="A"):
    """Run every UMAP configuration on X.

    Parameters
    ----------
    X : array-like
    feature_set_name : str
        Label used only in log messages.

    Returns
    -------
    dict[str, np.ndarray]
        Mapping config_name -> 2D embedding.
    """
    results = {}
    total = len(config.UMAP_CONFIGS)
    for idx, name in enumerate(config.UMAP_CONFIGS, 1):
        print(f"[UMAP] Running config {idx}/{total}: '{name}' (feature_set={feature_set_name})")
        results[name] = fit_umap(X, config_name=name)
    print(f"[UMAP] All {total} configs complete for feature_set={feature_set_name}")
    return results


def cache_embedding(embedding, name, output_dir=None):
    """Save an embedding array as a .npy file.

    Parameters
    ----------
    embedding : np.ndarray
    name : str
        Base name for the file (without extension).
    output_dir : Path or None
        Directory to write to; defaults to config.IMAGES_DIR.
    """
    out = (output_dir or config.IMAGES_DIR)
    out.mkdir(parents=True, exist_ok=True)
    path = out / f"{name}.npy"
    np.save(path, embedding)
    print(f"[cache] Saved embedding '{name}' -> {path}  shape={embedding.shape}")


def load_cached_embedding(name, output_dir=None):
    """Load a previously cached embedding, or return None.

    Parameters
    ----------
    name : str
        Base name used when caching.
    output_dir : Path or None

    Returns
    -------
    np.ndarray or None
    """
    path = (output_dir or config.IMAGES_DIR) / f"{name}.npy"
    if path.exists():
        embedding = np.load(path)
        print(f"[cache] Loaded embedding '{name}' from {path}  shape={embedding.shape}")
        return embedding
    return None
