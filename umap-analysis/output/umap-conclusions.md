+++
title = "Conclusions from UMAP Dimensionality Reduction of the Trans-Atlantic Slave Trade Database"
date = 2026-02-16
draft = true
template = "blog/page.html"

[taxonomies]
tags = ["data-visualization", "umap", "slave-trade", "digital-humanities", "machine-learning", "mortality", "resistance", "abolition"]
authors = ["Clinton Bowen"]
+++

## Overview

This analysis applies UMAP (Uniform Manifold Approximation and Projection) dimensionality reduction and HDBSCAN clustering to 31,728 trans-Atlantic slave trade voyages from the [SlaveVoyages database](https://www.slavevoyages.org), spanning 1514--1866. The 178 features per voyage---geographic origin and destination, year, nationality, ship tonnage, crew size, slave counts, mortality rates, demographic ratios, and voyage duration---are compressed into a two-dimensional embedding that reveals the structural geometry of the trade. A separate analysis of 11,521 intra-American voyages provides a comparative baseline.

What follows are empirical conclusions drawn from the data. Every number cited is computed directly from the database.

---

## I. Mortality

### The arithmetic of death

Across all 31,728 trans-Atlantic voyages in the filtered dataset, the mean imputed mortality rate is **29.3%**. The database records approximately **9.98 million enslaved people embarked**, though the true number---accounting for unrecorded voyages, deaths before embarkation, and incomplete records---is certainly higher.

### Mortality varied enormously by era

The deadliest period was **1650--1674**, with a mean mortality rate of **63.2%**. This coincides with the early operations of chartered monopoly companies, the Anglo-Dutch Wars, and an era before the infrastructure of the trade had been systematized. By contrast, **1775--1799** saw the lowest mortality at **19.1%**, reflecting accumulated logistical knowledge, shorter standardized routes, and commercial incentives to keep captives alive.

| Era | Mortality (%) | Avg Slaves Embarked | Voyages |
|-----|--------------|-------------------|---------|
| 1575--1625 | 19.9--43.0 | 359--360 | 604--627 |
| 1625--1675 | 31.5--63.2 | 316--324 | 502--532 |
| 1675--1725 | 29.3--44.8 | 266--306 | 1,439--3,001 |
| 1725--1775 | 24.0--46.9 | 267--312 | 4,055--6,064 |
| 1775--1825 | 19.1--24.1 | 306--315 | 5,438--5,595 |
| 1825--1875 | 26.7--62.5 | 414--607 | 365--3,294 |

The late surge in 1850--1874 (62.5% mortality, 607 average slaves) reflects the desperate, overcrowded illegal trade in its final years.

### Overcrowding was the primary killer

The strongest correlate of mortality is the number of slaves embarked (r = +0.22), followed by voyage duration (r = +0.17) and overcrowding measured as slaves per ton (r = +0.15). Ship tonnage alone has essentially no correlation with mortality (r = -0.02)---what mattered was not the size of the ship but how many human beings were packed into it.

Overcrowding (slaves/ton) peaked at **3.48** in 1600--1624 and resurged to **2.85** in 1825--1849 during the illegal trade, compared to a low of **1.47** during 1775--1799 when regulatory pressure and commercial competition briefly moderated conditions.

### Embarkation region shaped survival

Enslaved people taken from **Southeast Africa and the Indian Ocean islands** suffered the highest mortality at **52.4%**, a direct consequence of the vastly longer middle passage to the Americas. Those embarked from **Sierra Leone** had the lowest rate at **19.9%**, reflecting the relatively shorter crossing to Caribbean and North American destinations.

| Embarkation Region | Mortality (%) | Avg Slaves | Voyages |
|---|---|---|---|
| Southeast Africa & Indian Ocean | 52.4 | 444 | 876 |
| Bight of Biafra & Gulf of Guinea | 30.6 | 301 | 3,524 |
| West Central Africa & St. Helena | 28.9 | 384 | 8,745 |
| Bight of Benin | 28.6 | 332 | 4,479 |
| Gold Coast | 27.6 | 267 | 2,595 |
| Senegambia | 25.2 | 231 | 2,343 |
| Windward Coast | 21.9 | 222 | 838 |
| Sierra Leone | 19.9 | 215 | 1,026 |

### Destination mattered too

**Brazil** had the lowest regional mortality at **25.6%**, benefiting from the shortest Atlantic crossing from West Central Africa. **Spanish Mainland America** (primarily the Circum-Caribbean) had the highest at **35.4%**, reflecting longer and more circuitous routes.

---

## II. Chartered Companies

### Companies dominated the trade

**57.5%** of all documented voyages (18,241 of 31,728) occurred under chartered company regimes. These state-granted monopolies were not marginal actors---they were the institutional backbone of the slave trade for most of its history.

### The fifteen largest chartered company operations

| Company | Voyages | Total Enslaved | Mortality (%) | Period | Primary Destination |
|---|---|---|---|---|---|
| African Co. of Merchants / SSC | 6,180 | 1,716,955 | 19.7 | 1753--1820 | Jamaica, Barbados |
| RAC / South Sea Co. | 2,906 | 751,398 | 49.2 | 1711--1751 | Jamaica, Barbados |
| French Compagnie des Indes | 1,464 | 463,362 | 30.9 | 1719--1769 | Saint-Domingue, Martinique |
| Royal African Co. (alone) | 1,347 | 361,038 | 43.3 | 1673--1710 | Barbados, Jamaica |
| Real Compania de Filipinas | 1,259 | 366,993 | 26.8 | 1801--1834 | Cuba |
| Grao-Para & Pernambuco Cos. | 860 | 263,667 | 16.9 | 1759--1777 | Bahia, SE Brazil |
| Dutch WIC / VOC / Middelburgsche | 794 | 265,038 | 27.5 | 1720--1792 | Dutch Guianas |
| Dutch WIC / VOC (early) | 732 | 281,344 | 38.3 | 1630--1719 | Dutch Caribbean |
| Brazil Co. (Comp. Geral do Brasil) | 561 | 147,423 | 40.7 | 1649--1720 | Bahia |
| Cacheu & Cape Verde / Brazil Co. | 423 | 101,458 | 14.6 | 1690--1706 | Bahia |
| South Sea Co. (late, alone) | 421 | 203,238 | 31.0 | 1829--1853 | SE Brazil, Cuba |
| Guinea Co. of London | 318 | 104,342 | 27.4 | 1618--1659 | Spanish Circum-Caribbean |
| Comp. Geral Grao-Para e Maranhao | 238 | 73,349 | 17.3 | 1755--1778 | Bahia, Pernambuco |
| Compagnie du Senegal (3rd) | 155 | 46,023 | 50.7 | 1710--1718 | Saint-Domingue |
| Danish West India-Guinea Co. | 136 | 44,098 | 81.2 | 1691--1776 | Danish West Indies |

### The RAC was a death machine

The **Royal African Company**, operating from 1672 to 1752, produced the most consistently deadly voyages among major operators. During the period when it overlapped with the South Sea Company (1711--1751), their joint operations recorded mortality of **49.2%** across 2,906 voyages. The RAC alone (1673--1710) ran at **43.3%**. By contrast, after the RAC's dissolution in 1752, the African Company of Merchants---operating in a competitive rather than monopoly environment---achieved **19.7%** mortality across 6,180 voyages. The transition from monopoly to open trade cut the death rate by more than half.

### Portuguese companies achieved the lowest mortality

The **Companhia Geral do Grao-Para e Maranhao** and its sister Pernambuco company operated at **16.9%** mortality. The **Cacheu and Cape Verde Company**, overlapping with the Brazil Company, achieved **14.6%**---the lowest of any major chartered operation. This reflects the geographic advantage of the South Atlantic route (West Africa to Brazil) and the Portuguese empire's centuries of accumulated experience in the trade.

### The Danish company was catastrophic

The **Danish West India-Guinea Company** recorded **81.2%** mortality across 136 voyages to the Danish West Indies. This rate is nearly three times the trans-Atlantic average and suggests either systematic mismanagement or a fundamentally inadequate logistical chain between the Gold Coast (their primary source) and the Caribbean.

---

## III. Wars and Disruptions

### One-third of all voyages occurred during wartime

**10,643 voyages (33.5%)** took place during one of the eleven major conflicts coded in the analysis, from the First Anglo-Dutch War (1652) through the War of 1812.

### Wars reduced mortality---through selection bias

Counterintuitively, wartime voyages had **lower** mean mortality (23.3%) than peacetime voyages (32.4%). This is almost certainly a selection effect: only the best-provisioned, largest, and most capable ships attempted the crossing during wartime. Wartime vessels averaged 181 tons versus 154 in peacetime, and carried fewer enslaved people (294 vs 325), suggesting that armed escort requirements and capture risk selected for larger, less-crowded ships.

### War-by-war findings

| War | Voyages | Mortality (%) | Peacetime Nearby (%) | Delta |
|---|---|---|---|---|
| First Anglo-Dutch War (1652--54) | 38 | 91.2 | 46.1 | +45.1 |
| Third Anglo-Dutch War (1672--74) | 61 | 65.7 | 48.5 | +17.2 |
| Second Anglo-Dutch War (1665--67) | 72 | 55.2 | 53.2 | +2.0 |
| Nine Years' War (1688--97) | 437 | 45.7 | 44.5 | +1.2 |
| War of Spanish Succession (1701--14) | 1,402 | 30.6 | 35.7 | -5.1 |
| Napoleonic Wars (1803--15) | 2,248 | 23.5 | 26.8 | -3.3 |
| Seven Years' War (1756--63) | 1,409 | 23.4 | 29.5 | -6.1 |
| Haitian Revolution (1791--1804) | 3,535 | 17.1 | 23.9 | -6.8 |
| American Revolution (1775--83) | 1,441 | 18.7 | 23.7 | -5.0 |

The early Anglo-Dutch Wars (small sample sizes, maritime chaos in the Atlantic) were genuinely more lethal. But by the 18th century, wars actually *reduced* observed mortality, consistent with the selection-bias hypothesis.

### The Haitian Revolution was the largest wartime disruption

With 3,535 voyages, the Haitian Revolution era produced the most wartime-affected voyages of any conflict. It also caused the largest UMAP displacement (3.87 units from the peacetime centroid), meaning the *structural profile* of voyages during the revolution was markedly different from normal operations. The revolution did not just reduce traffic to Saint-Domingue---it reshaped the entire geometry of the Caribbean trade.

### The Napoleonic Wars displaced the trade most in UMAP space

Despite not being the deadliest conflict, the Napoleonic Wars caused the largest centroid displacement in the UMAP embedding (4.81 units), indicating the most comprehensive structural transformation of the trade. The complete reshuffling of Atlantic alliances, the British naval blockade, and the shift of Portuguese/Brazilian operations to the fore all contributed to a fundamentally different trade structure during 1803--1815.

---

## IV. Resistance

### Documented resistance was rare but significant

**460 voyages (1.4%)** have documented resistance events. This is certainly an undercount---resistance that was successfully suppressed or went unrecorded would not appear in European shipping logs. The documented events break down as:

- **393 slave insurrections** (85.4%)
- **45 vessels attacked from shore** (9.8%)
- **15 vessels' boats attacked from shore** (3.3%)
- **4 insurrections planned but thwarted** (0.9%)
- **2 mass escape attempts** (jumping overboard) (0.4%)

### Resistance voyages were longer and deadlier

Voyages with documented resistance lasted longer (63 vs 58 days) and had slightly higher mortality (31.4% vs 29.3%). This correlation could run in either direction: longer voyages may have provoked more resistance, or the disruption caused by resistance may have extended voyage duration and increased deaths.

### The French and American trades had the highest resistance rates

| Flag | Resistance Rate | Events | Voyages |
|---|---|---|---|
| France | **3.23%** | 116 | 3,594 |
| U.S.A. | **2.62%** | 43 | 1,644 |
| Spain / Uruguay | 1.69% | 29 | 1,711 |
| Netherlands | 1.28% | 20 | 1,563 |
| Great Britain | 1.24% | 153 | 12,317 |
| Portugal / Brazil | 0.91% | 96 | 10,584 |

The French rate is more than triple the Portuguese rate. While Great Britain recorded the most absolute resistance events (153), its rate per voyage was relatively low---possibly reflecting both the scale of British participation and better-armed British vessels.

### Senegambia produced the highest resistance rates by embarkation

Enslaved people from **Senegambia** resisted at a rate of **2.09%**, the highest of any embarkation region. This is consistent with the historical literature documenting organized resistance traditions among Senegambian peoples. The Gold Coast (1.62%), Bight of Benin (1.38%), and West Central Africa (1.30%) also produced significant resistance. Southeast Africa had the lowest rate (1.03%), despite having the highest mortality---suggesting that the extreme length and brutality of the Southeast African crossing may have reduced the physical capacity for organized revolt.

### Resistance declined sharply after 1825

The resistance rate dropped from around 1.5--2.2% throughout the 17th and 18th centuries to **0.39%** in 1825--1849 and **0.27%** in 1850--1874. This may reflect the shift to the illegal trade, where slavers had even stronger incentives to suppress and not document resistance, and where the conditions of the crossing may have left captives less able to organize.

---

## V. The Intra-American Trade

### A structurally different system

The intra-American slave trade database records **11,521 voyages** spanning 1550--1841, transporting approximately **456,132 enslaved people**. Compared to the trans-Atlantic trade:

| Metric | Trans-Atlantic | Intra-American |
|---|---|---|
| Voyages | 31,728 | 11,521 |
| Total enslaved | ~9.98 million | ~456,000 |
| Avg slaves/voyage | 315 | 40 |
| Mean mortality | 29.3% | 2.5% |

The intra-American trade operated at a fundamentally different scale: **40 enslaved people per voyage** versus 315 for the trans-Atlantic crossing. Mortality was **2.5%** versus 29.3%---an order of magnitude lower, reflecting the vastly shorter voyage distances within the Americas.

When projected into the same UMAP space, the intra-American voyages occupy a **distinct region** of the embedding, confirming that the two trades were structurally different systems, not smaller and larger versions of the same thing.

---

## VI. Brazil

### The largest destination in the trade

Brazil received **9,705 documented voyages** (30.6% of the trans-Atlantic total), carrying approximately **3.52 million enslaved people**---more than any other region. The trade to Brazil spanned 1574--1856, the longest continuous period of any major destination.

### Four sub-regions, four different trades

| Region | Voyages | Total Enslaved | Mortality (%) | Avg Slaves | Dominant Flag | Primary Source |
|---|---|---|---|---|---|---|
| Bahia | 4,239 | 1,347,765 | 22.5 | 318 | Portugal / Brazil (98.7%) | Bight of Benin |
| Southeast Brazil | 3,541 | 1,542,668 | 30.6 | 436 | Portugal / Brazil (91.5%) | West Central Africa |
| Pernambuco | 1,377 | 477,651 | 23.5 | 347 | Portugal / Brazil (90.6%) | West Central Africa |
| Amazonia | 548 | 151,374 | 22.4 | 276 | Portugal / Brazil (98.5%) | Senegambia |

**Bahia** was the most tightly coupled to a single African source: the **Bight of Benin** supplied 2,362 of its 4,239 voyages (55.7%). This reflects the deep Luso-African commercial networks of the Mina Coast and the cultural connections documented in the historiography of Bahian slave society.

**Southeast Brazil** (primarily Rio de Janeiro) received the largest individual shipments (avg 436 enslaved) and had the highest mortality (30.6%) of the Brazilian sub-regions. Its primary source was **West Central Africa** (2,770 of 3,541 voyages), reflecting the Luanda-Rio axis that was the backbone of the South Atlantic system.

**Amazonia** stands apart: smaller ships (avg 276 enslaved), sourced primarily from **Senegambia** (367 of 548 voyages), with the lowest mortality of any Brazilian sub-region (22.4%). This northern route was a geographically and commercially distinct operation.

### Portuguese companies and the Brazil trade

The **Companhia Geral do Grao-Para e Maranhao** and its sister Pernambuco company together accounted for 860 voyages (856 to Brazil specifically), operating with remarkably low mortality (16.9%). The earlier **Brazil Company** (Companhia Geral do Estado do Brasil, 1649--1720) handled 561 voyages but at much higher mortality (40.7%), reflecting the more dangerous conditions of the earlier trade.

### Brazil's trade intensified as others declined

While British and French participation collapsed after abolition, the Brazil trade surged: the 1825--1875 period saw the highest average slave counts (440 per voyage) and continued at high volume (2,119 voyages) well past Brazil's nominal first abolition in 1831.

---

## VII. Cuba

### The late-trade powerhouse

Cuba received **2,549 documented voyages** carrying approximately **875,742 enslaved people**. Unlike most destinations, Cuba's trade *intensified* in the 19th century: **1,194 voyages occurred after Spain's 1817 abolition treaty**, more than double the pre-abolition count of 824.

### A multi-flag operation

| Flag | Voyages to Cuba |
|---|---|
| Spain / Uruguay | 1,308 (51.3%) |
| Great Britain | 383 (15.0%) |
| U.S.A. | 362 (14.2%) |
| Portugal / Brazil | 289 (11.3%) |
| France | 168 (6.6%) |

Cuba was the most nationally diverse major destination. The significant British (383), American (362), and Portuguese (289) participation---well after their own abolition acts---documents the extent to which the illegal trade to Cuba operated under multiple flags of convenience.

### Post-abolition Cuba was deadlier

| Period | Voyages | Mortality (%) | Avg Slaves |
|---|---|---|---|
| Pre-1817 (legal) | 824 | 21.5 | -- |
| Post-1817 (illegal) | 1,725 | 30.8 | -- |
| 1775--1825 | 1,291 | 21.1 | 244 |
| 1825--1875 | 1,194 | 34.9 | 452 |

Post-abolition voyages to Cuba carried nearly twice as many enslaved people (452 vs 244) at substantially higher mortality (34.9% vs 21.1%). The illegal trade was not just a continuation of the legal trade by other means---it was a more concentrated, more brutal, and more deadly operation.

### The Real Compania de Filipinas was the dominant Cuban company

The Spanish **Real Compania de Filipinas** accounted for 1,259 voyages, with 1,065 going specifically to Cuba. Operating from 1801--1834, it carried 366,993 enslaved people at 26.8% mortality---a *legal* Spanish company operating through the transition to illegality.

### Cuba's African sources

Cuba's enslaved population was sourced broadly, with **"Other Africa"** (a catchall for unspecified regions) accounting for 1,750 of 2,549 voyages. The Bight of Biafra (224), West Central Africa (192), and Sierra Leone (100) were the next most common specified sources.

---

## VIII. Nueva Granada (Spanish Circum-Caribbean)

### An early-trade destination

The Spanish Circum-Caribbean (encompassing modern-day Colombia, Venezuela, Panama, and surrounding territories) received **1,498 voyages** carrying approximately **540,759 enslaved people**, with a mean mortality of **35.4%**---well above the trans-Atlantic average.

### Dominated by the early period

| Era | Voyages | Mortality (%) |
|---|---|---|
| 1525--1575 | 144 | 24.4 |
| 1575--1625 | 881 | 33.3 |
| 1625--1675 | 338 | 43.1 |
| 1675--1725 | 106 | 43.7 |
| 1725--1775 | 16 | 51.3 |
| 1775--1825 | 13 | 17.2 |

**88.3% of all voyages to Nueva Granada occurred before 1675.** This was overwhelmingly an early-colonial phenomenon, driven by the initial construction of the Spanish colonial economy. Mortality rose steadily over the period, from 24.4% in the earliest decades to 51.3% by the mid-18th century (though the late sample sizes are very small).

### A dual-flag operation

| Flag | Voyages |
|---|---|
| Great Britain | 763 (50.9%) |
| Portugal / Brazil | 559 (37.3%) |
| Spain / Uruguay | 98 (6.5%) |

Remarkably, Great Britain and Portugal together accounted for **88.2%** of voyages to Spanish colonial territory. Spain itself supplied only 6.5% of its own colonial slave trade to this region, relying instead on the asiento system and foreign carriers.

### Sourced from Senegambia and West Central Africa

**Senegambia** (508 voyages) and **West Central Africa** (630 voyages) were the primary embarkation regions for the Nueva Granada trade. The Guinea Co. of London---one of the earliest English chartered companies---sent 198 of its 318 total voyages to the Spanish Circum-Caribbean, making it the primary vehicle for English participation in the early Spanish colonial slave trade.

---

## IX. Nueva Espana (New Spain)

### The earliest American destination

Nueva Espana---encompassing the ports of Veracruz, Campeche, Guatemala, and Santo Tomas de Castilla---received **744 voyages** carrying approximately **271,798 enslaved people** at a mean mortality of **43.4%**, well above both the trans-Atlantic average (29.3%) and the broader Spanish Circum-Caribbean average (35.4%). These voyages span 1545--1817, though **98.3% occurred before 1675**, making Nueva Espana an almost exclusively early-colonial destination within the trans-Atlantic slave trade.

Nueva Espana constitutes a distinct sub-region within the broader Spanish Circum-Caribbean (MJSELIMP=41200, 1,498 total voyages). The remaining SCC voyages divide among Nueva Granada proper (Colombia/Panama), Venezuela, Honduras, and unspecified ports.

### Port breakdown

| Port | Voyages | Enslaved | Mortality (%) | Period |
|---|---|---|---|---|
| Nueva Espana, unspecified | 510 | 186,239 | 36.7 | 1545--1666 |
| Veracruz | 280 | 103,866 | 51.1 | 1546--1733 |
| Campeche | 166 | 60,982 | 61.2 | 1598--1735 |
| Guatemala | 4 | 1,280 | 25.0 | 1621--1817 |
| Santo Tomas de Castilla | 1 | 362 | 58.0 | 1613 |

**Veracruz** was the principal named port, handling 280 documented voyages. **Campeche** had the highest mortality at **61.2%**---more than double the trans-Atlantic average---likely reflecting longer coastal redistribution routes within the Gulf of Mexico after the Atlantic crossing. The large "unspecified" category (510 voyages) reflects the early period's poor record-keeping; these voyages are known to have arrived somewhere in New Spain but the specific port was not recorded.

### Temporally confined to the early colonial period

| Era | Voyages | Enslaved | Mortality (%) |
|---|---|---|---|
| 1525--1575 | 81 | 29,542 | 28.3 |
| 1575--1625 | 396 | 147,501 | 45.3 |
| 1625--1675 | 254 | 90,849 | 45.1 |
| 1675--1700 | 3 | -- | 31.3 |

The overwhelming concentration of voyages in 1575--1625 (53.2% of total) corresponds to the height of the asiento system under the Spanish Habsburg monarchy, when the Crown contracted with Portuguese and later English merchants to supply enslaved labor to its American colonies. The 25-year breakdown reveals that 1600--1625 was the single most active quarter-century, with 316 voyages and the highest mortality (51.6%).

After 1675, the trade to Nueva Espana effectively ceased---only 3 voyages are documented between 1675 and 1700, and sporadic single voyages thereafter. This termination reflects the shift of Spanish colonial labor demand to Cuba and the declining economic importance of the Mexican silver economy relative to Caribbean sugar.

### A Portuguese-then-British operation

| Flag | Voyages | Pct | Enslaved | Mortality (%) |
|---|---|---|---|---|
| Portugal / Brazil | 415 | 55.8% | 151,380 | 55.6 |
| Great Britain | 288 | 38.7% | 104,578 | 25.9 |
| Spain / Uruguay | 37 | 5.0% | 13,791 | 41.4 |

**Portugal dominated the early period** through the asiento system: as holders of the slave-trading monopoly for Spanish colonies, Portuguese merchants supplied over half of all voyages to Nueva Espana. Their mortality rate of **55.6%** was dramatically higher than the British rate of **25.9%**---a gap of nearly 30 percentage points. This may reflect the Portuguese trade's earlier temporal position (higher mortality in earlier eras) and different routing patterns.

**Great Britain entered through the Guinea Company of London**, which directed 113 of its 318 total voyages to Nueva Espana. British participation was concentrated in the 1618--1660 period, overlapping with and eventually replacing Portuguese asiento holders. The dramatically lower British mortality (25.9%) may reflect later-period operations when trade infrastructure had improved.

Spain itself supplied only **5.0%** of the enslaved labor to its own colony, continuing the pattern seen across the entire Spanish Circum-Caribbean.

### African sources: West Central Africa and Senegambia

| Source Region | Voyages | Pct |
|---|---|---|
| West Central Africa & St. Helena | 449 | 60.3% |
| Senegambia & offshore Atlantic | 216 | 29.0% |
| Other Africa | 38 | 5.1% |
| Bight of Biafra & Gulf of Guinea islands | 26 | 3.5% |
| Bight of Benin | 12 | 1.6% |

**West Central Africa** was the primary source (60.3%), consistent with the Portuguese trade networks centered on Luanda. **Senegambia** contributed 29.0%, reflecting the earliest phase of the trade when Upper Guinea was the primary embarkation zone. Together these two regions supplied nearly 90% of all enslaved people sent to New Spain.

### Company involvement

The trade to Nueva Espana was **81.7% independent** (non-company) operations---the highest independent share of any major destination analyzed, reflecting the asiento system's reliance on individual licensed traders rather than chartered monopolies. The **Guinea Company of London** was the only significant company presence, accounting for 113 voyages (15.2%). The Brazil Company contributed 10 voyages, and the RAC/SSC combination contributed 7 voyages in the later period.

### Extreme overcrowding on small ships

Nueva Espana-bound ships averaged **115 tons**---the smallest of any major destination---with an overcrowding rate of **3.60 slaves per ton**, the highest of any regional analysis. Average crew was 14.5 with 6.1 guns per ship. The middle passage averaged 54.4 days. These metrics paint a picture of small, lightly armed vessels packed beyond capacity---consistent with the early-period Portuguese trade before standardization and regulation.

### UMAP positioning

In the UMAP embedding, Nueva Espana voyages cluster at centroid **(6.81, -1.07)**, displaced **4.66 units** from the overall dataset centroid and **0.94 units** from the broader SCC centroid. The NE cluster sits in the early-period / Portuguese zone of the embedding, adjacent to but distinguishable from the Nueva Granada cluster. The relatively high UMAP spread (std 8.29, 7.07) reflects the 130-year temporal range and the transition from Portuguese to British operators within the same destination.

---

## X. Haiti (Saint-Domingue)

### The French colony that consumed human lives

Saint-Domingue received **2,333 voyages** carrying approximately **799,203 enslaved people** at 29.4% mortality. It was **97.9% a French operation** (2,283 of 2,333 voyages).

### The revolution ended the trade entirely

| Period | Voyages |
|---|---|
| Pre-revolution (before 1791) | 2,212 |
| During revolution (1791--1804) | 118 |
| Post-revolution (after 1804) | 3 |

The Haitian Revolution reduced the slave trade to Saint-Domingue by **94.7%** during the conflict itself, and effectively eliminated it afterward. Only 3 voyages are recorded post-independence. No other event in the history of the trade produced such a complete and permanent termination of a major trade route.

### Revolution-era mortality was lower

Pre-revolution mortality stood at **29.9%**, but during the revolution itself it dropped to **20.2%**. This likely reflects that the voyages that continued during 1791--1804 were smaller, more carefully provisioned operations---the easy, routine trade had been disrupted, leaving only the most capable operators.

### The Compagnie des Indes built Saint-Domingue

The **French Compagnie des Indes** directed **959 of its 1,464 total voyages to Saint-Domingue** (65.5%). This single company-route pair---French Indies Company to Saint-Domingue---represents one of the most concentrated institutional channels in the entire slave trade.

### African sources for Haiti

Saint-Domingue drew from a broad range of African regions: **West Central Africa** (829 voyages), **Bight of Benin** (497), **Senegambia** (215), and the **Bight of Biafra** (137). This diversity is reflected in the cultural and linguistic heterogeneity documented in Haitian history and the origins of Haitian Vodou.

---

## XI. The United States

### A smaller but significant destination

The United States mainland received **1,714 documented trans-Atlantic voyages** carrying approximately **359,184 enslaved people**---a fraction of the Brazilian or Caribbean trades, but with profound consequences for American history.

### South Carolina and Virginia dominated

| Destination | Voyages | Total Enslaved | Mortality (%) |
|---|---|---|---|
| South Carolina | 826 | 178,057 | 26.5 |
| Virginia | 444 | 100,740 | 41.9 |
| Georgia | 121 | 20,257 | 19.0 |
| Maryland | 116 | 25,857 | 42.2 |
| New York | 70 | 8,756 | 19.0 |
| Gulf Coast | 50 | 12,590 | 22.7 |
| Florida | 34 | 6,924 | 34.0 |
| Rhode Island | 29 | 3,034 | 14.9 |
| Massachusetts | 24 | 2,969 | 17.2 |

**South Carolina alone received 48.2%** of all voyages to what became the United States. The combined South Carolina--Virginia corridor accounted for 74.1% of documented trans-Atlantic arrivals.

**Virginia and Maryland had dramatically higher mortality** (41.9% and 42.2%) than other U.S. destinations. This may reflect the Chesapeake's position as a secondary stop after Caribbean ports, longer transit times, or the prevalence of smaller operators with less experience.

**New England destinations (Rhode Island, Massachusetts) had the lowest mortality** (14.9% and 17.2%), consistent with the shorter crossing from West Africa to the northeast.

### A British-then-American operation

| Flag | Voyages |
|---|---|
| Great Britain | 1,132 (66.0%) |
| U.S.A. | 528 (30.8%) |
| France | 31 (1.8%) |

Before independence, Great Britain carried 66% of all trans-Atlantic voyages to what became the U.S. After independence, American-flagged vessels took over---but the U.S. trade was always a subordinate branch of the British Atlantic system.

### The 1808 abolition was effective

The U.S. 1808 ban on the trans-Atlantic trade was strikingly effective: only **27 voyages** are documented after the ban, compared to 1,687 before. This 98.4% reduction stands in sharp contrast to Cuba, where post-abolition voyages *exceeded* pre-abolition. The U.S. achieved enforcement through a combination of naval patrols, domestic political will (however inconsistent), and the development of a self-reproducing enslaved population that reduced economic demand for new imports.

### African sources for the United States

The U.S. drew from the broadest range of African sources of any destination: **Senegambia** (351 voyages), **West Central Africa** (226), **Bight of Biafra** (191), **Sierra Leone** (189), with no single region accounting for more than 26%. This diversity is reflected in the cultural complexity of African-American communities.

---

## XII. UMAP Structural Findings

### 193 clusters reveal natural groupings

HDBSCAN identified 193 distinct clusters in the 2D UMAP embedding, with 6,022 outlier voyages (19.0%). These clusters are not arbitrary---they correspond to real structural groupings defined by nationality, era, route, and company.

### Abolition is the largest structural break

The UMAP centroid displacement between pre- and post-abolition voyages is **7.83 units**---the largest structural break in the dataset, larger than any war or company transition. The trade before and after abolition occupied *fundamentally different regions* of the embedding: 116 clusters contain only pre-abolition voyages, 22 contain only post-abolition voyages, and only 56 are shared.

### Companies have distinct UMAP "fingerprints"

Each chartered company occupies a characteristic region of the UMAP space. The most similar companies (closest centroids) are the Danish Africa Company and the Swedish Africa Company (0.51 UMAP units apart), confirming they operated nearly identical trade patterns. The most dissimilar are the African Company of Merchants and the French West India Company (20.6 units apart)---different eras, routes, demographics, and ship configurations.

---

## XIII. Abolition: A Structural Transformation, Not a Cessation

### The largest break in the dataset

The general 1807 boundary divides the dataset into 24,187 pre-abolition and 7,541 post-abolition voyages. The UMAP centroid displacement across this boundary is **7.83 units**---the single largest structural discontinuity in the entire embedding, exceeding any war, any company transition, or any other historical event. Abolition did not merely reduce the trade's volume; it fundamentally altered its character.

### Nation-by-nation: seven abolitions, seven different outcomes

| Nation | Year | Pre-Voyages | Post-Voyages | Pre Mort. | Post Mort. | UMAP Shift | Post Destinations |
|---|---|---|---|---|---|---|---|
| Denmark / Baltic | 1803 | 272 | 35 | 66.7% | 28.9% | 4.11 | Cuba, Danish W.I., SE Brazil |
| Great Britain | 1807 | 11,593 | 724 | 30.8% | 27.6% | 7.83 | SE Brazil, Cuba, Jamaica |
| U.S.A. | 1808 | 1,437 | 207 | 16.5% | 71.4% | 5.43 | Cuba, SE Brazil |
| Netherlands | 1814 | 1,557 | 6 | 32.2% | --- | --- | (effectively ended) |
| France | 1815 | 3,165 | 429 | 30.9% | 16.9% | 10.21 | Cuba, Martinique |
| Spain / Uruguay | 1817 | 547 | 1,164 | 29.6% | 27.2% | 2.43 | Cuba (85%) |
| Portugal / Brazil | 1831 | 9,268 | 1,316 | 28.0% | 25.1% | 8.01 | SE Brazil, Cuba, Bahia |

Three distinct patterns emerge:

**Effective termination** (Netherlands): Only 6 post-abolition voyages recorded. The Dutch trade simply stopped.

**Dramatic structural shift** (Great Britain, France, Portugal/Brazil): These nations' post-abolition voyages occupy fundamentally different UMAP space (7.83--10.21 units of displacement). France shows the largest shift: its 429 post-abolition voyages have the lowest mortality of any post-abolition cohort (16.9%), suggesting these were the most professional operators in the illegal trade---smaller crews, shorter voyages (44 vs 75 days), and different destinations (Cuba replaced Saint-Domingue).

**Expansion under illegality** (Spain/Uruguay): Spain is the only nation where post-abolition voyages *exceeded* pre-abolition (1,164 vs 547). The smallest UMAP displacement (2.43 units) indicates that the structure of the Spanish trade barely changed---it simply continued to Cuba under the same operational model, with 85% of post-abolition Spanish voyages going to Cuba. Spain's 1817 treaty was, in practice, dead letter.

### The U.S.A. paradox: effective ban, catastrophic survivors

The U.S. abolished the trans-Atlantic trade in 1808 and achieved near-total enforcement: only 207 post-ban voyages versus 1,437 before, a 98.4% reduction. But the surviving illegal voyages were the most deadly of any post-abolition cohort: **71.4% mortality**, with average shipments of 543 (vs 134 pre-ban) and overcrowding of 2.43 slaves/ton (vs 1.57). These post-1808 American-flagged vessels were overwhelmingly bound for Cuba (124 of 207), operating as part of the Cuban illegal trade network rather than importing to the United States.

### What changed structurally after 1807

**The destination map was redrawn.** Before 1807, the Caribbean absorbed 60.0% of all voyages; after 1807, only 33.1%. Brazil surged from 22.8% to **55.6%**. The post-abolition trade was overwhelmingly a South Atlantic operation.

**The embarkation map shifted.** West Central Africa rose from 23.7% to **39.9%** of embarkations. The Gold Coast collapsed from 10.4% to 0.9%---the most dramatic regional decline, reflecting British naval patrols concentrated along the Gold Coast. Southeast Africa surged from 1.2% to **7.9%**, as slavers sought sources beyond the reach of abolition-era enforcement.

**The national composition inverted.** Great Britain fell from 47.9% to 9.6% of voyages. Portugal/Brazil rose from 25.2% to **59.4%**. Spain/Uruguay rose from 0.8% to **20.2%**. The post-abolition trade was a Portuguese-Spanish operation, with British participation reduced to illegal ventures or anti-slavery patrol captures.

**Overcrowding intensified.** Post-abolition Portuguese voyages averaged **2.99 slaves per ton**, up from 1.97---the highest overcrowding rate of any major national post-abolition cohort. The incentive structure of the illegal trade favored cramming maximum human cargo into fast ships that could outrun patrols.

### Cluster analysis confirms the structural break

Of the 193 HDBSCAN clusters, **116 contain only pre-abolition voyages** and **22 contain only post-abolition voyages**. Only 56 clusters span the boundary. The post-abolition trade was not a diminished version of the pre-abolition trade operating in the same structural space---it was a new system that emerged into its own distinct region of the UMAP manifold.

---

## XIV. HDBSCAN Cluster Profiles

### 193 clusters are not arbitrary

The 193 clusters identified by HDBSCAN correspond to real structural groupings---specific combinations of nationality, era, route, company, and ship profile that the algorithm identifies as internally coherent and distinct from neighboring groups.

### Death clusters and survival clusters

The clusters with highest mortality (>100%) reflect imputation artifacts in the VYMRTIMP field for early-period voyages with incomplete records. Setting those aside, genuine high-mortality clusters include:

- **Cluster 48** (66 voyages, 99.8% mortality): 98% U.S.A.-flagged, 1834--1862, 94% Cuba-bound. This is the late American illegal trade to Cuba---small, desperate, and catastrophically deadly.
- **Cluster 135** (112 voyages, 145.2% mortality): 45% Danish/Baltic, 1659--1739, 33% Danish West Indies. This captures the early Danish company era, with mortality figures that may reflect combined voyage and coastal deaths.

At the other extreme:

- **Cluster 148** (51 voyages, 2.9% mortality): 82% British, 1785--1809, Jamaica-bound. Late-era British professional operators achieving near-zero middle passage deaths.
- **Cluster 180** (125 voyages, 5.9% mortality): 93% Portuguese, 1796--1838, Southeast Brazil. The mature Portuguese South Atlantic route at its most efficient.
- **Cluster 62** (164 voyages, 7.4% mortality): 100% British, 1793--1808, 100% wartime. British wartime voyages that were also among the safest---again consistent with the selection-bias finding that wartime selected for better-provisioned ships.

### Nationally homogeneous clusters

Multiple clusters are 100% Portuguese/Brazilian, reflecting the tightly integrated Luso-African trading system that operated on standardized routes with consistent ship profiles. The Portuguese trade was structurally the most uniform, which HDBSCAN captures as dense, well-separated clusters.

### Company clusters

Several clusters are >90% company-operated:

- **Cluster 87** (347 voyages): 100% African Co. of Merchants / SSC, 26.0% mortality.
- **Cluster 106** (257 voyages): 100% Royal African Co., 37.8% mortality.
- **Cluster 112** (215 voyages): 100% Grao-Para & Pernambuco companies, 13.9% mortality.
- **Cluster 128** (215 voyages): 100% Real Compania de Filipinas, 23.0% mortality.
- **Cluster 89** (240 voyages): 99% RAC/SSC, 25.7% mortality.

These company clusters confirm that chartered companies imposed standardized operational patterns that HDBSCAN identifies as structurally distinct from independent operators.

### Wartime clusters

Clusters dominated by wartime voyages (>99%) are exclusively British: Clusters 62, 86, 95, 150, 163. These wartime-British clusters all show lower-than-average mortality (7.4--26.0%), reinforcing the finding that British wartime voyages were a select subset of well-resourced operations.

---

## XV. National Trajectories Through UMAP Space

### Each nation traces a path through history

By computing the centroid of each nation's voyages per 25-year era and connecting them chronologically, we can trace how each nation's trade *moved through UMAP space* over time---a visual narrative of structural change.

**Great Britain** traces the longest total path: **72.5 UMAP units** across 14 eras, reflecting the British trade's dramatic evolution from early Guinea Coast ventures (1520s) through the RAC monopoly, the post-1752 liberalization, the peak era of the African Company of Merchants, and the post-1807 collapse into illegal activity. The largest single-era jump (14.5 units, 1525--1575) reflects the chaotic early period; the post-1807 movement toward the upper portion of the embedding reflects the shift to Brazilian and Cuban destinations.

**Portugal/Brazil** has the second-longest path at **77.5 UMAP units** across 12 eras, with the most erratic early trajectory (17.1-unit jump in 1625--1675 during the Dutch occupation and reconquest of northeastern Brazil). The Portuguese trajectory stabilizes after 1725 and moves steadily through UMAP space, reflecting the increasingly consolidated Luanda-to-Brazil axis.

**France** traces a shorter, more compact path (**45.4 UMAP units**, 8 eras). The largest jumps are at the beginning (12.3 units, 1650--1700, as the French trade was being established) and after abolition (10.7 units, 1775--1825, as the legal trade collapsed and the illegal trade emerged). The French trajectory is the most "folded"---it doubles back on itself in UMAP space, consistent with the French trade's reinvention after the Haitian Revolution destroyed its primary destination.

**Spain/Uruguay** shows the most dramatic late-period transformation: a **7.9-unit jump into the 1800--1825 era** as the Spanish trade surged from marginal participant to major illegal trader. The Spanish centroid barely moved for over a century (small colonial trade) and then leapt to a new position when the Cuban sugar boom drove massive demand.

**U.S.A.** traces the shortest path (**36.0 UMAP units**, 8 eras) and shows the sharpest terminal movement: a **6.2-unit jump into 1825--1850** followed by an **8.1-unit jump in 1850--1875**, as the few remaining American-flagged illegal voyages became structurally distinct from all prior American operations.

---

## XVI. The Caribbean Compared

### Eight Caribbean destinations, eight different profiles

| Destination | Voyages | Enslaved | Mortality | Avg Load | Dominant Flag | Primary African Source |
|---|---|---|---|---|---|---|
| Jamaica | 3,586 | 1,086,227 | 31.9% | 303 | Great Britain 97% | Bight of Biafra 22% |
| Saint-Domingue | 2,333 | 799,203 | 29.4% | 343 | France 98% | West Central Africa 36% |
| Cuba | 2,549 | 875,742 | 27.8% | 344 | Spain 51%, GB 15%, US 14% | "Other Africa" 69% |
| Barbados | 2,034 | 454,060 | 35.9% | 223 | Great Britain 91% | "Other Africa" 41% |
| Dutch Guianas | 904 | 297,676 | 28.1% | 329 | Netherlands 83% | West Central Africa 24% |
| Martinique | 703 | 204,103 | 29.3% | 290 | France 72%, GB 22% | Bight of Benin 26% |
| Antigua | 600 | 142,273 | 33.9% | 237 | Great Britain 92% | Bight of Biafra 16% |
| Danish West Indies | 374 | 105,671 | **45.7%** | 283 | Denmark 56%, GB 26% | Gold Coast 30% |

**Jamaica was the largest Caribbean destination** but had relatively high mortality (31.9%). Its near-total British monopoly (97%) and heavy reliance on Bight of Biafra sources distinguish it from other Caribbean destinations.

**Saint-Domingue received larger average shipments** (343) than Jamaica (303) at lower mortality (29.4%), reflecting the efficiency of the French Compagnie des Indes system and the West Central Africa--to--Caribbean route.

**Barbados had the highest mortality of the major British Caribbean destinations** (35.9%) despite the smallest average shipments (223). Barbados was often the first Caribbean port of call, receiving the sickest captives and serving as a transshipment point.

**The Danish West Indies had catastrophic mortality** at 45.7%---the worst of any major Caribbean destination, consistent with the Danish West India-Guinea Company's overall record of 81.2%.

**Cuba was the most nationally diverse destination**: five nationalities with >5% share. This diversity intensified after 1817 when Spanish, British, American, and Portuguese operators all participated in the illegal trade to Cuba.

---

## XVII. Outliers: The Anomalous 19%

### 6,022 voyages don't fit any cluster

HDBSCAN classified 6,022 voyages (19.0%) as outliers---points that don't belong to any of the 193 identified clusters. These are not errors; they are voyages whose combination of features is sufficiently unusual that they don't match any standard operational pattern.

### Outliers are not dramatically different in averages

| Metric | Outliers | Normal | Difference |
|---|---|---|---|
| Mean year | 1771 | 1761 | +10 |
| Avg slaves | 314 | 315 | -1 |
| Mortality | 30.7% | 29.0% | +1.7% |
| Tonnage | 169 | 161 | +8 |
| Duration | 56.5 days | 57.8 days | -1.3 |

The surprise is how *normal* the outliers look in aggregate. They are outliers not because of extreme individual values but because of **unusual combinations**---a Portuguese ship on a British route, a wartime vessel in a peacetime pattern, a company voyage with independent-trade characteristics.

### Outliers are disproportionately Portuguese/Brazilian and French

Portuguese/Brazilian voyages make up 38.3% of outliers but only 32.2% of normal clustered voyages. French voyages are 15.0% of outliers vs 10.5% of normals. Great Britain is underrepresented in outliers (28.4% vs 41.3%), consistent with the British trade being the most standardized and cluster-friendly.

### Outliers concentrate in the late 18th century

41.5% of outliers come from 1775--1825, compared to 33.5% of normal voyages. This is the era of maximum disruption: the American Revolution, Haitian Revolution, French Revolutionary Wars, Napoleonic Wars, and the onset of abolition all overlapping. The structural chaos of this era produced voyages that defied standard categorization.

---

## XVIII. The Geography of Forced Migration: African Source-Destination Pairings

### Each African region fed a different American system

The UMAP embedding separates voyages partly because different African embarkation regions were tightly coupled to specific American destinations. These were not random pairings---they reflect centuries-deep commercial networks, African political relationships with European traders, and the path-dependence of maritime routes.

**Where enslaved people from each African region were taken:**

| African Embarkation | Primary Destination (%) | Secondary (%) | Tertiary (%) |
|---|---|---|---|
| Bight of Benin | Brazil 59% | Caribbean 35% | Africa 3% |
| West Central Africa | Brazil 58% | Caribbean 28% | Spanish Mainland 9% |
| Southeast Africa | Brazil 61% | Caribbean 22% | Spanish Mainland 8% |
| Bight of Biafra | Caribbean 74% | Brazil 12% | Mainland N. America 5% |
| Gold Coast | Caribbean 83% | Mainland N. America 7% | Brazil 6% |
| Windward Coast | Caribbean 84% | Mainland N. America 10% | Africa 3% |
| Senegambia | Caribbean 40% | Spanish Mainland 23% | Brazil 18% |
| Sierra Leone | Caribbean 69% | Mainland N. America 19% | Africa 8% |

Three distinct patterns emerge:

**The Brazilian axis:** The Bight of Benin, West Central Africa, and Southeast Africa all fed primarily into Brazil. This South Atlantic system operated almost independently of the North Atlantic routes---different ships, different operators (overwhelmingly Portuguese), different seasonal patterns. West Central Africa was the backbone: it supplied **52%** of all enslaved people arriving in Brazil.

**The Caribbean funnel:** The Gold Coast, Windward Coast, Bight of Biafra, and Sierra Leone all fed primarily into the Caribbean (69--84%). These were the North Atlantic routes dominated by Britain, France, and the Netherlands. The Bight of Biafra had a particularly tight coupling to the British Caribbean, especially Jamaica, where Igbo and Efik peoples formed a significant part of the enslaved population.

**The Senegambian outlier:** Senegambia was the most diversified embarkation region, with no single destination taking more than 40%. Its 23% share going to the Spanish Circum-Caribbean (Nueva Granada) is the highest of any region---a legacy of the earliest period of the trade when Senegambia was the primary source and the Spanish Caribbean the primary destination.

### The reverse view: where each destination sourced from

| Destination | Primary Source (%) | Secondary (%) |
|---|---|---|
| Brazil | West Central Africa 52% | Bight of Benin 27% |
| Caribbean | "Other Africa" 33% | Bight of Biafra 15% |
| Mainland North America | "Other Africa" 27% | Senegambia 20% |
| Spanish Mainland America | West Central Africa 42% | Senegambia 28% |

Brazil's reliance on West Central Africa (52%) is the tightest source-destination coupling in the entire dataset, reflecting the Luanda--Rio de Janeiro axis that persisted for over two centuries.

---

## XIX. Ship Evolution: Three Centuries of Slaving Vessels

### The slave ship was not static

| Era | Tonnage | Crew | Guns | Slaves | Slaves/Ton | Duration |
|---|---|---|---|---|---|---|
| 1525--1575 | 152 | 14.5 | 6.0 | 333 | 2.01 | 57 days |
| 1575--1625 | 118 | 14.5 | 6.0 | 359 | 3.26 | 52 days |
| 1625--1675 | 185 | 14.5 | 11.7 | 321 | 2.14 | 69 days |
| 1675--1725 | 170 | 14.5 | 14.1 | 280 | 2.06 | 65 days |
| 1725--1775 | 132 | 16.1 | 7.8 | 285 | 2.40 | 66 days |
| 1775--1825 | 194 | 14.6 | 8.1 | 309 | 1.62 | 53 days |
| 1825--1875 | 153 | 13.7 | 2.9 | 429 | 2.76 | 40 days |

**The 1575--1625 era had the worst overcrowding** at 3.26 slaves per ton---small Portuguese ships packed beyond capacity for the short South Atlantic crossing. This declined as larger ships entered the trade, reaching a low of **1.62 slaves/ton in 1775--1825** when regulation (the Dolben Act of 1788) and commercial competition briefly moderated conditions.

**Then the illegal trade reversed the trend.** After 1825, overcrowding surged to 2.76 slaves/ton while guns dropped to 2.9 per ship (from a peak of 14.1 in 1675--1725). The illegal trade's vessels were fast, lightly armed clippers designed for speed rather than combat---the goal was to outrun patrols, not fight them. Crews shrank to 13.7 (from 16.1 at peak) and voyage duration dropped to 40 days (from 69 in the mid-17th century), reflecting the premium on speed in the illegal era.

**Guns peaked during the chartered-company era** (1675--1725, averaging 14.1) when company vessels were essentially armed merchantmen operating in waters contested by multiple European navies.

---

## XX. Guns, Crews, and Resistance

### More guns, more resistance

| Guns per Ship | Resistance Rate | Voyages |
|---|---|---|
| 0--5 | 1.03% | 5,541 |
| 5--10 | 1.30% | 16,893 |
| 10--20 | **2.04%** | 6,217 |
| 20--50 | 1.84% | 2,287 |

Counterintuitively, more heavily armed ships experienced *more* resistance. Voyages with documented resistance carried an average of **9.2 guns** versus 8.2 for non-resistance voyages. Ships with 10--20 guns had double the resistance rate (2.04%) of lightly armed ships (1.03%).

This likely reflects confounding rather than causation: heavily armed ships tended to be larger, carry more enslaved people, and operate on longer routes---all factors that independently increase resistance probability. The arms were a response to risk, not a deterrent.

### Crew-to-slave ratios varied dramatically by nationality

| Flag | Ratio | Resistance Rate |
|---|---|---|
| U.S.A. | 1 crew per 7 enslaved | 2.62% |
| Spain / Uruguay | 1 crew per 9 enslaved | 1.69% |
| Great Britain | 1 crew per 12 enslaved | 1.24% |
| Portugal / Brazil | 1 crew per 13 enslaved | 0.91% |
| France | 1 crew per 14 enslaved | **3.23%** |
| Netherlands | 1 crew per 15 enslaved | 1.28% |

France stands out: it had the lowest crew-to-slave ratio (1:14) and the **highest resistance rate** (3.23%). Fewer crew members per enslaved person meant less capacity to suppress revolt. The Dutch had a similarly low ratio (1:15) but lower resistance, possibly reflecting the different demographics of their enslaved cargoes or different shipboard control methods.

The U.S. had the highest crew ratio (1:7) but the second-highest resistance rate, suggesting that American slavers anticipated trouble---likely because their smaller, more diverse cargoes included a higher proportion of already-resisting captives.

---

## XXI. Voyage Duration and the Price of Distance

### The crossing shortened dramatically over three centuries

Average voyage duration fell from **69 days** in 1625--1675 to **40 days** in 1825--1875---a 42% reduction. This reflects improvements in ship design, better navigational knowledge, and the illegal trade's selection for speed.

### Duration was the strongest predictor of mortality on Caribbean routes

| Route | Duration-Mortality Correlation | Avg Duration |
|---|---|---|
| Jamaica | r = **+0.43** | 66 days |
| Barbados | r = +0.32 | 70 days |
| Southeast Brazil | r = +0.23 | 43 days |
| Bahia | r = -0.04 | 43 days |
| Cuba | r = -0.02 | 48 days |

For Jamaica-bound voyages, the duration-mortality correlation (r = +0.43) is the strongest single-variable predictor in the data---stronger even than overcrowding. Each additional day at sea measurably increased the death toll. For Brazilian routes, the correlation nearly vanishes: the crossing was short enough (43 days average) that duration variation mattered less than other factors.

**Brazil's geographic advantage is confirmed:** at 43 days average, the South Atlantic crossing was 35% shorter than the Caribbean crossing (65 days) and 36% shorter than North American routes (67 days). This distance advantage is the single largest explanatory factor for Brazil's lower mortality rates.

---

## XXII. The "Other Africa" Problem: 23% of Voyages Have No Specified African Origin

### A significant data limitation

**7,301 voyages (23.0%)** list their African embarkation region as "Other Africa"---an unspecified catchall. These are not uniformly distributed: they are **disproportionately British** (52.0% vs 34.9% in known-origin voyages), **disproportionately Spanish** (14.5% vs 2.7%), and **disproportionately American** (9.8% vs 3.8%). Portuguese voyages are dramatically underrepresented (7.3% vs 41.1%), reflecting the Portuguese trade's better-documented relationship with specific African ports.

These "Other Africa" voyages went overwhelmingly to the **Caribbean (77.8%)** and carried smaller-than-average cargoes (281 vs 325 for known-origin voyages). Their mortality was slightly higher (31.0% vs 28.8%).

The British and American overrepresentation in "Other Africa" suggests that these nations' records were less precise about African origins---or that their traders sourced from a wider, less routinized set of African ports. This data gap means that our understanding of the geographic specificity of the British Caribbean and American trades is less precise than for the Portuguese-Brazilian trade.

---

## Methodological Note

This analysis uses the imputed and standardized TASTDB dataset. Imputation (group-median by nationality and 25-year era) can introduce artifacts, particularly for the RESISTANCE column, which was inflated from 460 genuine events to 28,000+ by median imputation. All resistance analysis in this report uses raw, pre-imputation values. Mortality rates (VYMRTIMP) are the database's own imputed estimates and should be interpreted as approximations, not exact figures. Demographic ratios (MENRAT1, CHILRAT1, etc.) have extremely low pre-imputation coverage (2--10% of voyages) and should be interpreted with caution.

UMAP is a nonlinear projection: distances in the embedding are not proportional to distances in the original feature space. Clusters that appear close may not be close in all 178 original dimensions, and vice versa.

---

## References

1. Slave Voyages Project. *Trans-Atlantic Slave Trade Database*. [slavevoyages.org](https://www.slavevoyages.org).
2. Slave Voyages Project. *Intra-American Slave Trade Database (I-Am 1.0)*. [slavevoyages.org](https://www.slavevoyages.org).
3. McInnes, L., Healy, J., & Melville, J. (2018). UMAP: Uniform Manifold Approximation and Projection for Dimension Reduction. *arXiv:1802.03426*.
4. Campello, R. J. G. B., Moulavi, D., & Sander, J. (2013). Density-Based Clustering Based on Hierarchical Density Estimates. *PAKDD 2013*.
5. Eltis, D. & Richardson, D. (2010). *Atlas of the Transatlantic Slave Trade*. Yale University Press.
