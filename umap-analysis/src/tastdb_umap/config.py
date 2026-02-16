"""Configuration constants, paths, and historical event definitions."""

from pathlib import Path

# ── Paths ──────────────────────────────────────────────────────────────────
BASE_DIR = Path("/private/tmp/charts/slave-trade-map")
UMAP_DIR = BASE_DIR / "umap-analysis"

TASTDB_RAW = BASE_DIR / "tastdb-exp-2020-raw.csv"
TASTDB_VALUE_LABELS = BASE_DIR / "tastdb-exp-2020-value-labels.csv"
TASTDB_VARIABLES = BASE_DIR / "tastdb-exp-2020-variables.csv"
IAM_CSV = BASE_DIR / "I-Am1.0.csv"

IMAGES_DIR = UMAP_DIR / "images"
INTERACTIVE_DIR = UMAP_DIR / "interactive"
OUTPUT_DIR = UMAP_DIR / "output"

SLAVECODES_BLOG_DIR = BASE_DIR / "slavecodes_org" / "content" / "blog"

# ── Random seed ────────────────────────────────────────────────────────────
RANDOM_SEED = 42

# ── UMAP configurations ───────────────────────────────────────────────────
UMAP_CONFIGS = {
    "default": {"n_neighbors": 15, "min_dist": 0.1, "metric": "euclidean"},
    "local": {"n_neighbors": 5, "min_dist": 0.05, "metric": "euclidean"},
    "global": {"n_neighbors": 50, "min_dist": 0.25, "metric": "euclidean"},
    "cosine": {"n_neighbors": 15, "min_dist": 0.1, "metric": "cosine"},
}

# ── Trade era bins (25-year periods) ──────────────────────────────────────
TRADE_ERA_BINS = list(range(1500, 1900, 25))
TRADE_ERA_LABELS = [f"{y}-{y+24}" for y in TRADE_ERA_BINS[:-1]]

# ── Nationality codes (from NATINIMP value labels in tastdb-exp-2020) ─────
# Verified against tastdb-exp-2020-value-labels.csv
NAT_CODES = {
    "Spain": 1,
    "Uruguay": 2,
    "Spain / Uruguay": 3,
    "Portugal": 4,
    "Brazil": 5,
    "Portugal / Brazil": 6,
    "Great Britain": 7,
    "Netherlands": 8,
    "U.S.A.": 9,
    "France": 10,
    "Denmark": 11,
    "Hanse Towns, Brandenburg": 12,
    "Sweden": 13,
    "Norway": 14,
    "Denmark / Baltic": 15,
    "Argentina": 16,
    "Russia": 17,
    "Sardinia": 18,
    "Mexico": 19,
    "Genoa": 24,
    "Duchy of Courland": 25,
    "Prussia": 26,
    "Bremen": 27,
    "Austria": 28,
    "Germany": 29,
    "Other": 30,
    "Texas": 31,
}

NAT_CODE_TO_NAME = {v: k for k, v in NAT_CODES.items()}

# Broad group mapping: sub-codes -> display group
NAT_BROAD_GROUP = {
    1: "Spain / Uruguay", 2: "Spain / Uruguay", 3: "Spain / Uruguay",
    4: "Portugal / Brazil", 5: "Portugal / Brazil", 6: "Portugal / Brazil",
    7: "Great Britain",
    8: "Netherlands",
    9: "U.S.A.",
    10: "France",
    11: "Denmark / Baltic", 12: "Denmark / Baltic", 13: "Denmark / Baltic",
    14: "Denmark / Baltic", 15: "Denmark / Baltic", 25: "Denmark / Baltic",
    16: "Other", 17: "Other", 18: "Other", 19: "Other",
    24: "Other", 26: "Other", 27: "Other", 28: "Other",
    29: "Other", 30: "Other", 31: "Other",
}

# Top nationality colors (keyed by broad group display name)
NATIONALITY_COLORS = {
    "Spain / Uruguay": "#e41a1c",
    "Portugal / Brazil": "#377eb8",
    "Great Britain": "#4daf4a",
    "Netherlands": "#ff7f00",
    "U.S.A.": "#984ea3",
    "France": "#a65628",
    "Denmark / Baltic": "#f781bf",
    "Other": "#999999",
}

# ── Chartered companies ───────────────────────────────────────────────────
# Comprehensive list sourced from https://en.wikipedia.org/wiki/List_of_chartered_companies
# and related historical sources. Mapped to NATINIMP nationality codes.
# (name -> {nat_codes, start, end, color})
CHARTERED_COMPANIES = {
    # ── English / British ──────────────────────────────────────────────────
    "Guinea Co. of London": {
        "nat_codes": [7],  # Great Britain
        "start": 1618,
        "end": 1660,
        "color": "#2ca02c",
    },
    "Co. of Royal Adventurers": {
        "nat_codes": [7],
        "start": 1660,
        "end": 1672,
        "color": "#98df8a",
    },
    "Royal African Co. (RAC)": {
        "nat_codes": [7],
        "start": 1672,
        "end": 1752,
        "color": "#4daf4a",
    },
    "African Co. of Merchants": {
        "nat_codes": [7],
        "start": 1752,
        "end": 1821,
        "color": "#d62728",
    },
    "South Sea Co. (SSC)": {
        "nat_codes": [7],
        "start": 1711,
        "end": 1853,
        "color": "#b2df8a",
    },
    # ── Dutch ──────────────────────────────────────────────────────────────
    "Dutch West India Co. (WIC)": {
        "nat_codes": [8],  # Netherlands
        "start": 1621,
        "end": 1792,
        "color": "#ff7f00",
    },
    "Dutch East India Co. (VOC)": {
        "nat_codes": [8],
        "start": 1602,
        "end": 1799,
        "color": "#fdbf6f",
    },
    "Middelburgsche Commercie Co.": {
        "nat_codes": [8],
        "start": 1720,
        "end": 1798,
        "color": "#ffbb78",
    },
    # ── French ─────────────────────────────────────────────────────────────
    "French West India Co.": {
        "nat_codes": [10],  # France
        "start": 1664,
        "end": 1674,
        "color": "#a65628",
    },
    "Compagnie du Senegal (1st)": {
        "nat_codes": [10],
        "start": 1673,
        "end": 1681,
        "color": "#fb9a99",
    },
    "Compagnie de Guinee": {
        "nat_codes": [10],
        "start": 1684,
        "end": 1706,
        "color": "#e377c2",
    },
    "Compagnie royale du Senegal": {
        "nat_codes": [10],
        "start": 1696,
        "end": 1709,
        "color": "#f7b6d2",
    },
    "Compagnie du Senegal (3rd)": {
        "nat_codes": [10],
        "start": 1709,
        "end": 1718,
        "color": "#c49c94",
    },
    "French Indies Co. (Compagnie des Indes)": {
        "nat_codes": [10],
        "start": 1719,
        "end": 1769,
        "color": "#8c564b",
    },
    # ── Portuguese / Brazilian ─────────────────────────────────────────────
    "Comp. de Cacheu e Rios de Guine": {
        "nat_codes": [6],  # Portugal/Brazil
        "start": 1676,
        "end": 1682,
        "color": "#377eb8",
    },
    "Cacheu and Cape Verde Co.": {
        "nat_codes": [6],
        "start": 1690,
        "end": 1706,
        "color": "#1f77b4",
    },
    "Brazil Co. (Comp. Geral do Brasil)": {
        "nat_codes": [6],
        "start": 1649,
        "end": 1720,
        "color": "#aec7e8",
    },
    "Maranhao Co.": {
        "nat_codes": [6],
        "start": 1682,
        "end": 1685,
        "color": "#9edae5",
    },
    "Comp. Geral Grao-Para e Maranhao": {
        "nat_codes": [6],
        "start": 1755,
        "end": 1778,
        "color": "#a6cee3",
    },
    "Comp. Geral de Pernambuco e Paraiba": {
        "nat_codes": [6],
        "start": 1759,
        "end": 1777,
        "color": "#17becf",
    },
    # ── Spanish ────────────────────────────────────────────────────────────
    "Real Compania Gaditana de Negros": {
        "nat_codes": [3],  # Spain/Uruguay
        "start": 1765,
        "end": 1779,
        "color": "#e41a1c",
    },
    "Real Compania de Filipinas": {
        "nat_codes": [3],
        "start": 1785,
        "end": 1834,
        "color": "#fc4e2a",
    },
    "Real Compania de La Habana": {
        "nat_codes": [3],
        "start": 1740,
        "end": 1800,
        "color": "#fd8d3c",
    },
    # ── Danish / Scandinavian ──────────────────────────────────────────────
    "Danish Africa Co.": {
        "nat_codes": [15],  # Denmark/Baltic
        "start": 1659,
        "end": 1671,
        "color": "#bcbd22",
    },
    "Danish West India-Guinea Co.": {
        "nat_codes": [15],
        "start": 1671,
        "end": 1776,
        "color": "#f781bf",
    },
    "Swedish Africa Co.": {
        "nat_codes": [15],
        "start": 1649,
        "end": 1663,
        "color": "#cab2d6",
    },
    # ── Brandenburg / Prussian ─────────────────────────────────────────────
    "Brandenburg-African Co.": {
        "nat_codes": [30],  # Other
        "start": 1682,
        "end": 1711,
        "color": "#6a3d9a",
    },
    # ── Courland (Baltic) ──────────────────────────────────────────────────
    "Duchy of Courland (colonial)": {
        "nat_codes": [15],  # Denmark/Baltic
        "start": 1637,
        "end": 1690,
        "color": "#dbdb8d",
    },
}

# ── Wars ──────────────────────────────────────────────────────────────────
WARS = {
    "First Anglo-Dutch War": (1652, 1654),
    "Second Anglo-Dutch War": (1665, 1667),
    "Third Anglo-Dutch War": (1672, 1674),
    "Nine Years' War": (1688, 1697),
    "War of Spanish Succession": (1701, 1714),
    "Seven Years' War": (1756, 1763),
    "American Revolution": (1775, 1783),
    "Haitian Revolution": (1791, 1804),
    "French Revolutionary Wars": (1792, 1802),
    "Napoleonic Wars": (1803, 1815),
    "War of 1812": (1812, 1815),
}

# ── Abolition acts ────────────────────────────────────────────────────────
ABOLITION_ACTS = {
    "Denmark": 1803,
    "Great Britain": 1807,
    "U.S.A.": 1808,
    "Netherlands": 1814,
    "France": 1815,
    "Spain": 1817,
    "Brazil (first)": 1831,
    "Brazil (effective)": 1850,
}

# General abolition year for pre/post comparison
ABOLITION_YEAR_GENERAL = 1807

# ── Feature set column definitions ────────────────────────────────────────
# Continuous columns used across feature sets
DEMOGRAPHIC_RATIOS = [
    "MENRAT1", "MALRAT1", "BOYRAT1", "GIRLRAT1", "CHILRAT1", "WOMANRAT1",
]
DEMOGRAPHIC_RATIOS_ALL = [
    "MENRAT1", "MENRAT3", "MENRAT7",
    "MALRAT1", "MALRAT3", "MALRAT7",
    "BOYRAT1", "BOYRAT3", "BOYRAT7",
    "GIRLRAT1", "GIRLRAT3", "GIRLRAT7",
    "CHILRAT1", "CHILRAT3", "CHILRAT7",
    "WOMANRAT1",
]
MORTALITY_COLS = ["VYMRTIMP", "SLADVOY", "SLADAFRI", "SLADAMER"]
VOYAGE_COLS = ["VOY1IMP", "VOY2IMP", "CREW", "GUNS"]
SCALE_COLS = ["SLAXIMP", "SLAMIMP", "TONNAGE"]

# Categorical columns for one-hot encoding
BROAD_REGION_COLS = ["MAJBYIMP1", "MJSELIMP1"]
SPECIFIC_REGION_COLS = ["MAJBYIMP", "MJSELIMP"]

# Key columns that get missingness indicators
MISSINGNESS_KEY_COLS = [
    "VYMRTIMP", "TONNAGE", "CREW", "GUNS", "SLAXIMP", "SLAMIMP",
    "MENRAT1", "MALRAT1", "BOYRAT1", "GIRLRAT1",
]

# ── Plot styling ──────────────────────────────────────────────────────────
PLOT_DPI = 300
PLOT_FIGSIZE = (12, 10)
PLOT_FIGSIZE_WIDE = (16, 10)
SCATTER_ALPHA = 0.4
SCATTER_SIZE = 2
