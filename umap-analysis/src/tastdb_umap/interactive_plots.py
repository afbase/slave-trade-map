"""Interactive Plotly visualizations for UMAP embeddings."""

import numpy as np
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go

from .config import (
    INTERACTIVE_DIR,
    NAT_CODE_TO_NAME,
    NATIONALITY_COLORS,
)


def _map_nationality(df: pd.DataFrame) -> pd.DataFrame:
    """Add a human-readable nationality column from NATINIMP codes."""
    df = df.copy()
    df["nationality"] = (
        df["NATINIMP"]
        .map(NAT_CODE_TO_NAME)
        .fillna("Other")
    )
    return df


def create_3d_exploration(
    emb_3d: np.ndarray,
    df: pd.DataFrame,
    output_dir=None,
) -> str:
    """Create an interactive 3D UMAP scatter plot.

    Parameters
    ----------
    emb_3d : ndarray of shape (N, 3)
        3-component UMAP embedding.
    df : DataFrame
        Source dataframe aligned row-wise with *emb_3d*.
    output_dir : Path, optional
        Directory for the HTML file (defaults to config.INTERACTIVE_DIR).

    Returns
    -------
    str
        Path to the saved HTML file.
    """
    output_dir = output_dir or INTERACTIVE_DIR
    output_dir.mkdir(parents=True, exist_ok=True)

    plot_df = _map_nationality(df)
    plot_df["UMAP1"] = emb_3d[:, 0]
    plot_df["UMAP2"] = emb_3d[:, 1]
    plot_df["UMAP3"] = emb_3d[:, 2]

    # Build hover text
    plot_df["emb_region"] = plot_df["MAJBYIMP1"].astype(str)
    plot_df["disemb_region"] = plot_df["MJSELIMP1"].astype(str)
    plot_df["slaves_embarked"] = plot_df["SLAXIMP"].fillna(0).astype(int).astype(str)

    hover_template = (
        "<b>Voyage %{customdata[0]}</b><br>"
        "Year: %{customdata[1]}<br>"
        "Nationality: %{customdata[2]}<br>"
        "Embarkation region: %{customdata[3]}<br>"
        "Disembarkation region: %{customdata[4]}<br>"
        "Slaves embarked: %{customdata[5]}"
        "<extra></extra>"
    )

    customdata = plot_df[
        ["VOYAGEID", "YEARAM", "nationality", "emb_region", "disemb_region", "slaves_embarked"]
    ].values

    # Determine color order so legend is consistent
    nationalities = sorted(plot_df["nationality"].unique(), key=lambda n: n)
    color_map = {n: NATIONALITY_COLORS.get(n, "#999999") for n in nationalities}

    fig = go.Figure()

    for nat in nationalities:
        mask = plot_df["nationality"] == nat
        fig.add_trace(
            go.Scatter3d(
                x=plot_df.loc[mask, "UMAP1"],
                y=plot_df.loc[mask, "UMAP2"],
                z=plot_df.loc[mask, "UMAP3"],
                mode="markers",
                name=nat,
                marker=dict(size=2, color=color_map[nat], opacity=0.5),
                customdata=customdata[mask.values],
                hovertemplate=hover_template,
            )
        )

    fig.update_layout(
        title="3D UMAP: Trans-Atlantic Slave Trade Voyages",
        scene=dict(
            xaxis_title="UMAP 1",
            yaxis_title="UMAP 2",
            zaxis_title="UMAP 3",
        ),
        legend_title="Nationality",
        width=1100,
        height=800,
    )

    out_path = output_dir / "3d_umap_exploration.html"
    fig.write_html(str(out_path))
    return str(out_path)


def create_temporal_slider(
    emb: np.ndarray,
    df: pd.DataFrame,
    output_dir=None,
) -> str:
    """Create a 2D UMAP scatter with a decade-based animation slider.

    Parameters
    ----------
    emb : ndarray of shape (N, 2)
        2-component UMAP embedding.
    df : DataFrame
        Source dataframe aligned row-wise with *emb*.
    output_dir : Path, optional
        Directory for the HTML file (defaults to config.INTERACTIVE_DIR).

    Returns
    -------
    str
        Path to the saved HTML file.
    """
    output_dir = output_dir or INTERACTIVE_DIR
    output_dir.mkdir(parents=True, exist_ok=True)

    plot_df = _map_nationality(df)
    plot_df["UMAP1"] = emb[:, 0]
    plot_df["UMAP2"] = emb[:, 1]
    plot_df["DECADE"] = (plot_df["YEARAM"] // 10) * 10

    # Drop rows with missing decade
    plot_df = plot_df.dropna(subset=["DECADE"])
    plot_df["DECADE"] = plot_df["DECADE"].astype(int)

    # Build color map for plotly express
    color_map = {n: NATIONALITY_COLORS.get(n, "#999999") for n in sorted(plot_df["nationality"].unique())}

    fig = px.scatter(
        plot_df,
        x="UMAP1",
        y="UMAP2",
        color="nationality",
        animation_frame="DECADE",
        color_discrete_map=color_map,
        hover_data=["VOYAGEID", "YEARAM", "nationality"],
        title="UMAP Temporal Evolution by Decade",
        opacity=0.5,
    )

    fig.update_traces(marker=dict(size=3))
    fig.update_layout(
        width=1100,
        height=800,
        xaxis_title="UMAP 1",
        yaxis_title="UMAP 2",
        legend_title="Nationality",
    )

    out_path = output_dir / "temporal_umap_slider.html"
    fig.write_html(str(out_path))
    return str(out_path)
