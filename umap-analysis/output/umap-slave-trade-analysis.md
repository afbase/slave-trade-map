+++
title = "UMAP Dimensionality Reduction Reveals Hidden Structure in Trans-Atlantic Slave Trade Data"
date = 2026-02-16
draft = true
template = "blog/page.html"

[taxonomies]
tags = ["data-visualization", "umap", "slave-trade", "digital-humanities", "machine-learning"]
authors = ["Clinton Bowen"]
+++

## Introduction

UMAP (Uniform Manifold Approximation and Projection) is a nonlinear dimensionality-reduction technique that preserves both local and global structure in high-dimensional data. Originally developed by McInnes, Healy, and Melville (2018), it has become a standard tool in machine learning for exploratory analysis of complex datasets. Here we apply UMAP to the Trans-Atlantic Slave Trade Database (TASTDB) -- one of the most comprehensive records of the forced migration of enslaved Africans -- to ask whether the mathematical structure of the voyage records reveals patterns that are difficult to see in tabular or summary form.

The goal is not to reduce human suffering to a scatterplot. Rather, it is to use computational methods to surface structural relationships -- temporal shifts, national trading patterns, mortality clusters, and the impact of abolition legislation -- that may otherwise remain hidden across 31,728 voyage records spanning 1514--1866.

## Data and Methods

### Source datasets

We draw on two datasets published by [slavevoyages.org](https://www.slavevoyages.org):

1. **Trans-Atlantic Slave Trade Database (TASTDB)** -- 31,728 documented voyages from the trans-Atlantic trade.
2. **Intra-American Slave Trade Database (I-Am 1.0)** -- 11,521 voyages within the Americas.

### Preprocessing

Continuous columns (voyage duration, crew size, slaves embarked/disembarked, mortality, demographic ratios) were imputed with column medians and standardized. Categorical columns (embarkation region, disembarkation region, nationality) were one-hot encoded. Key columns received binary missingness indicators so that the pattern of missing data itself becomes a feature.

### Feature sets

We constructed 178 features across four configurations:

- **Feature Set A** -- Core voyage features: year, duration, tonnage, crew, guns, slave counts, mortality, demographic ratios, broad geographic regions, nationality.
- **Feature Set B** -- Adds specific (port-level) geographic codes.
- **Feature Set C** -- Adds all available demographic ratio variants.
- **Feature Set D** -- Kitchen-sink: all of the above plus additional missingness indicators.

Feature Set A is used for the primary embedding throughout this post unless noted otherwise.

### UMAP parameters

Default configuration: `n_neighbors=15`, `min_dist=0.1`, Euclidean metric, random seed 42. We also tested local (`n_neighbors=5, min_dist=0.05`), global (`n_neighbors=50, min_dist=0.25`), and cosine-metric variants.

### Clustering

HDBSCAN was used for density-based cluster detection on the 2D UMAP embedding, identifying 193 clusters and 6,022 outlier voyages (19.0% of the dataset).

---

## Natural Geometry of the Trade

![Trade route clusters](./images/01_umap_trade_route_clusters.png)

The 2D UMAP embedding immediately separates voyages into visually distinct clusters. These clusters correspond largely to specific trade-route corridors -- combinations of embarkation region, disembarkation region, and time period. The three largest clusters are: Cluster 30 (1,037 voyages), Cluster 31 (796 voyages), Cluster 157 (503 voyages).

![HDBSCAN clusters](./images/09_umap_hdbscan_clusters.png)

HDBSCAN density-based clustering identifies natural groupings within the embedding. Overlaying national flag on the same embedding reveals that nationality and geography are tightly coupled: Portuguese/Brazilian voyages dominate the South Atlantic corridors, British voyages cluster in the Caribbean routes, and so on.

## Temporal Evolution

![Temporal coloring](./images/02_umap_temporal_evolution.png)

Coloring the embedding by year of arrival reveals a striking gradient: early voyages (1500s--1600s) occupy one region of the manifold while later voyages (1800s) occupy another, with smooth transitions in between. The trade did not simply grow in volume -- its *structure* changed over time.

![Temporal small multiples](./images/13_umap_temporal_small_multiples.png)

A 25-year era small-multiples view (see also the interactive slider in the appendix) makes this temporal migration through UMAP space even more vivid. National dominance shifts from Portugal to Britain to an illegal trade driven by multiple flags.

## Chartered Companies

![Company overlay](./images/06_umap_chartered_companies.png)

The major chartered companies of the early modern period -- the Royal African Company, Dutch West India Company, and others -- leave distinct signatures in the embedding. Company-era voyages cluster tightly, reflecting the standardized routes and ship configurations that monopoly trading imposed.

![Company timeline](./images/14_company_timeline_gantt.png)

A Gantt chart and stacked area visualization of chartered company active periods and voyage volumes reveals the temporal overlap and succession of monopoly trading enterprises.

## Wars and Disruptions

![War periods](./images/07_umap_war_periods.png)

Major conflicts -- the Seven Years' War, American Revolution, Napoleonic Wars, and Haitian Revolution -- produce visible perturbations in the embedding. War-period voyages are displaced from peacetime clusters, reflecting disrupted routes, changed flag states, and increased risk.

## The Abolition Boundary

![Abolition boundary](./images/08_umap_abolition_comparison.png)

Perhaps the most striking feature is the structural break around 1807, the year Britain abolished its slave trade. The embedding shows 24,187 pre-abolition and 7,541 post-abolition voyages occupying largely distinct regions of the manifold. Post-abolition voyages are not simply a continuation of earlier patterns under different flags -- they have a fundamentally different structure, reflecting illegal trade, different ship types, altered routes, and changed demographic compositions.

## Mortality and Demographics

![Mortality patterns](./images/04_umap_mortality_landscape.png)

Overlaying mortality rate on the embedding reveals spatial patterns: high-mortality voyages cluster in specific regions associated with longer middle-passage durations and particular embarkation zones. The demographic ratios (proportion of men, women, children) also show smooth gradients across the manifold.

## Outliers and Anomalies

![Outliers](./images/10_umap_outliers.png)

HDBSCAN identified 6,022 outlier voyages (19.0%) that do not fit cleanly into any cluster. These include unusually large or small shipments, voyages with atypical routes, and records with extensive missing data. Outlier analysis can surface individual voyages worthy of closer historical examination.

## Trans-Atlantic vs. Intra-American

![Dataset comparison](./images/12_umap_transatlantic_vs_intraamerican.png)

When the Intra-American (I-Am 1.0) voyages are projected into the same UMAP space as the trans-Atlantic voyages, they occupy a distinct region -- confirming that the intra-American trade was structurally different in terms of voyage duration, ship size, demographics, and geography.

## National Signatures

![Nationality clusters](./images/03_umap_national_patterns.png)

Each national flag produces a characteristic footprint in the embedding. The overlap between nations is informative: it shows where different powers competed on the same routes, while separation reveals nationally distinctive trading patterns.

## Centroid Trajectories

![Centroid trajectories](./images/15_centroid_trajectories.png)

Tracking the centroid of each nation's voyages by decade traces a path through UMAP space that narrates the history of each nation's involvement in the trade. Portugal's trajectory spans the entire manifold; Britain's rises sharply and then vanishes after 1807; the U.S. trajectory shows a late-period illegal trade that drifts toward the periphery.

## African Embarkation Regions

![Embarkation regions](./images/05_umap_african_embarkation_regions.png)

Coloring by broad African embarkation region shows clear spatial separation: Senegambia, the Gold Coast, the Bight of Biafra, West-Central Africa, and Southeast Africa each map to distinct portions of the embedding, reflecting the different trade networks and European partnerships associated with each region.

## Resistance

![Resistance events](./images/11_umap_resistance_events.png)

Voyages with documented resistance events (shipboard revolts, shore-based resistance) cluster in specific regions of the embedding, suggesting that resistance was associated with particular route, demographic, and temporal configurations rather than being randomly distributed.

---

## Methodology Notes

- **UMAP** reduces high-dimensional feature vectors to 2D (or 3D) coordinates while preserving both local neighborhood structure and broader topological features. The key parameters are `n_neighbors` (size of local neighborhood, default 15) and `min_dist` (minimum distance between points in the embedding, default 0.1).
- **Feature selection** matters: different feature sets emphasize different aspects of the data. Feature Set A (core voyage features) produces the most historically interpretable embedding; Feature Set D (all features) captures more variance but is harder to interpret.
- **HDBSCAN** is preferred over k-means because it does not require specifying the number of clusters in advance and can identify outliers.
- **Projection bias**: UMAP is a nonlinear projection and distances in the embedding are not directly proportional to distances in the original feature space. Clusters that appear close may not be close in the original data, and vice versa.

## Conclusion

Dimensionality reduction reveals that the Trans-Atlantic Slave Trade Database is not an undifferentiated mass of records. It has *geometric structure*: clusters that correspond to trade-route corridors, temporal gradients that narrate the evolution of the trade, national signatures that reflect geopolitical competition, and a stark boundary at abolition that confirms the structural transformation of the post-1807 illegal trade. These patterns are consistent with established historiography but are made visible *simultaneously* in a single embedding, offering a new mode of exploratory analysis for digital humanities researchers.

## Interactive Exploration

The following interactive HTML visualizations allow hands-on exploration:

- [3D UMAP Exploration](./interactive/3d_umap_exploration.html) -- Rotate and zoom the 3D embedding; hover for voyage details.
- [Temporal Slider](./interactive/temporal_umap_slider.html) -- Step through the trade decade by decade.

## References

1. Slave Voyages Project. *Trans-Atlantic Slave Trade Database*. [slavevoyages.org](https://www.slavevoyages.org).
2. McInnes, L., Healy, J., & Melville, J. (2018). UMAP: Uniform Manifold Approximation and Projection for Dimension Reduction. *arXiv:1802.03426*.
3. Campello, R. J. G. B., Moulavi, D., & Sander, J. (2013). Density-Based Clustering Based on Hierarchical Density Estimates. *PAKDD 2013*.
