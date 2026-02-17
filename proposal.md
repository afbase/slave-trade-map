
# Goal

Make a planning agent to analyze the remainder of this prompt and the codebase to develop a plan for claude agent teams.

There are several subproblems to consider with the data sets based on I-Am1.0.csv and tastdb-exp-2020-raw.csv (see SPSS_Codebook_2023-11-06.pdf for details) and the description of your proposal.

# My prompt from a prior session:

Suppose you have several origin points `O = {(x_i, y_i)}` , a point P distinct from all points in O and C, and a set C where each element in C is a connected, compact convex hull.  C can broken in two independent sets, C = Origin \union Destination.  There is also a set of points `\Delta = {(x_i, y_i)}` that is a subset of extreme points in the set Destination.

Your objective is that that for each point in O, you must create a S_i(t) where t [0,1] polyline (or spline be it catmull, bezier, etc)  that:
1. for any i \neq j, S_i must not intersect S_j, with the except of meeting at point P.
2. O is a collection of extreme points of the set Origin.
3. S_i does not intersect Origin set with the exception of the point (x_i, y_i) in O.
4.  for any i \neq j,  the limit of the derivative of S_i(t) as t goes to 1 is equal to the limit of the derivative of S_j(t) as t goes to 1.
5. S_i(1) = P for all i
In addition to this, There is a polyline (or spline be it catmull, bezier, etc) T(t,d) for t \in [0,1], and d \in \Delta where T(0,d) = P, and T(1,d) = d and the limit of the derivative of T(t,d) a t goes to 0 is equalt to the derivative of S_i(t) as t goes to 1 .

P is not in C. 

The problem described above is the ideal.


tests/interamerican-route-test.html
```
Inter-American Route Planner Tests

--- 1. Brazilian sources -> Rio de la Plata (42000): non-null waypoints ---
  PASS: 50100 -> 42000 non-null with length > 0
  PASS: 50200 -> 42000 non-null with length > 0
  PASS: 50300 -> 42000 non-null with length > 0
  PASS: 50400 -> 42000 non-null with length > 0
  PASS: 50500 -> 42000 non-null with length > 0

--- 2. Brazilian sources -> Caribbean Havana (31312): non-null waypoints ---
  PASS: 50100 -> 31312 non-null with length > 0
  PASS: 50200 -> 31312 non-null with length > 0
  PASS: 50300 -> 31312 non-null with length > 0
  PASS: 50400 -> 31312 non-null with length > 0
  PASS: 50500 -> 31312 non-null with length > 0

--- 3. Caribbean sources -> Tortola (33200): null or valid ---
  PASS: 34200 -> 33200 null (simple arc) or valid array
  PASS: 35100 -> 33200 null (simple arc) or valid array
  PASS: 33500 -> 33200 null (simple arc) or valid array

--- 4. North America (21300) -> Havana (31312): null or valid ---
  PASS: 21300 -> 31312 null (simple arc) or valid array

--- 5. Caribbean sources -> Rio de la Plata (42000): non-null waypoints ---
  PASS: 34200 -> 42000 non-null with length > 0
  PASS: 35100 -> 42000 non-null with length > 0

--- 6. No waypoint on land: Brazilian sources -> Rio de la Plata ---
  PASS: 50100 -> 42000 no waypoint on land
  PASS: 50200 -> 42000 no waypoint on land
  PASS: 50300 -> 42000 no waypoint on land
  PASS: 50400 -> 42000 no waypoint on land

--- 7. No waypoint on land: Bahia (50200) -> Havana (31312) ---
  PASS: 50200 -> 31312 no waypoint on land

--- 8. SE Brazil (50400) -> Rio de la Plata: all waypoints offshore (lng < -35) ---
  PASS: 50400 -> 42000 all waypoints have lng < -35

--- 9. Pernambuco (50300) -> Rio de la Plata: Ponta do Seixas vicinity waypoint ---
  PASS: 50300 -> 42000 has waypoint near Ponta do Seixas

--- 10-14. Destination Quadrant Classification ---
  PASS: classifyDestQuadrant(42001) === 'rio-plata' (Buenos Aires)
  PASS: classifyDestQuadrant(42002) === 'rio-plata' (Montevideo)
  PASS: classifyDestQuadrant(43001) === 'pacific-south' (Lima)
  PASS: classifyDestQuadrant(44002) === 'pacific-south' (Talcahuano)
  PASS: classifyDestQuadrant(31312) === 'caribbean' (Havana)

--- 15-18. Source Quadrant Classification ---
  PASS: classifySourceQuadrant(50200) === 'brazil-north' (Bahia)
  PASS: classifySourceQuadrant(50400) === 'brazil-south' (SE Brazil)
  PASS: classifySourceQuadrant(35100) === 'caribbean' (Jamaica)
  PASS: classifySourceQuadrant(21300) === 'north-america' (South Carolina)

--- 19. I-Am convergence: Caribbean -> Tortola ---
  PASS: Convergence X between source centroid X and dest X

--- 20. I-Am convergence: pan-American -> Havana not on land ---
  PASS: Pan-American -> Havana convergence not on land

--- 21. I-Am convergence: Brazilian -> Rio de la Plata off coast (lng < -30) ---
  PASS: Brazilian -> Rio de la Plata convergence lng < -30

--- 22-25. Port Resolution ---
  PASS: regionCoords[50109] exists and name === 'São Luis'
  PASS: regionCoords[44002] exists and name === 'Talcahuano'
  PASS: regionCoords[80713] exists (Rescued at sea)
  PASS: regionCoords[44000].name === 'Chile'

--- 26. Cache: repeated call returns same object reference ---
  PASS: Cached result is same object

--- 27. Backward compat: African sources still produce non-null waypoints ---
  PASS: TA 60100 -> Caribbean (Jamaica) non-null
  PASS: TA 60100 -> Brazil (Bahia) non-null
  PASS: TA 60100 -> North America (Charleston) non-null
  PASS: TA 60200 -> Caribbean (Jamaica) non-null
  PASS: TA 60200 -> Brazil (Bahia) non-null
  PASS: TA 60200 -> North America (Charleston) non-null
  PASS: TA 60300 -> Caribbean (Jamaica) non-null
  PASS: TA 60300 -> Brazil (Bahia) non-null
  PASS: TA 60300 -> North America (Charleston) non-null
  PASS: TA 60400 -> Caribbean (Jamaica) non-null
  PASS: TA 60400 -> Brazil (Bahia) non-null
  PASS: TA 60400 -> North America (Charleston) non-null
  PASS: TA 60500 -> Caribbean (Jamaica) non-null
  PASS: TA 60500 -> Brazil (Bahia) non-null
  PASS: TA 60500 -> North America (Charleston) non-null
  PASS: TA 60600 -> Caribbean (Jamaica) non-null
  PASS: TA 60600 -> Brazil (Bahia) non-null
  PASS: TA 60600 -> North America (Charleston) non-null
  PASS: TA 60700 -> Caribbean (Jamaica) non-null
  PASS: TA 60700 -> Brazil (Bahia) non-null
  PASS: TA 60700 -> North America (Charleston) non-null
  PASS: TA 60800 -> Caribbean (Jamaica) non-null
  PASS: TA 60800 -> Brazil (Bahia) non-null
  PASS: TA 60800 -> North America (Charleston) non-null
  PASS: TA 60900 -> Caribbean (Jamaica) non-null
  PASS: TA 60900 -> Brazil (Bahia) non-null
  PASS: TA 60900 -> North America (Charleston) non-null

============================================
Total: 67  Passed: 67  Failed: 0
ALL TESTS PASSED
```

tests/route-planner-test.html
```
Route Planner Unit Tests

--- 1. Non-null waypoints for all sources x 3 destinations ---
  PASS: 60100 -> Caribbean (Jamaica) non-null
  PASS: 60100 -> Brazil (Bahia) non-null
  PASS: 60100 -> North America (Charleston) non-null
  PASS: 60200 -> Caribbean (Jamaica) non-null
  PASS: 60200 -> Brazil (Bahia) non-null
  PASS: 60200 -> North America (Charleston) non-null
  PASS: 60300 -> Caribbean (Jamaica) non-null
  PASS: 60300 -> Brazil (Bahia) non-null
  PASS: 60300 -> North America (Charleston) non-null
  PASS: 60400 -> Caribbean (Jamaica) non-null
  PASS: 60400 -> Brazil (Bahia) non-null
  PASS: 60400 -> North America (Charleston) non-null
  PASS: 60500 -> Caribbean (Jamaica) non-null
  PASS: 60500 -> Brazil (Bahia) non-null
  PASS: 60500 -> North America (Charleston) non-null
  PASS: 60600 -> Caribbean (Jamaica) non-null
  PASS: 60600 -> Brazil (Bahia) non-null
  PASS: 60600 -> North America (Charleston) non-null
  PASS: 60700 -> Caribbean (Jamaica) non-null
  PASS: 60700 -> Brazil (Bahia) non-null
  PASS: 60700 -> North America (Charleston) non-null
  PASS: 60800 -> Caribbean (Jamaica) non-null
  PASS: 60800 -> Brazil (Bahia) non-null
  PASS: 60800 -> North America (Charleston) non-null
  PASS: 60900 -> Caribbean (Jamaica) non-null
  PASS: 60900 -> Brazil (Bahia) non-null
  PASS: 60900 -> North America (Charleston) non-null

--- 2. No waypoint on land for sources 60100-60800 ---
  PASS: 60100 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60100 -> Brazil (Bahia) no waypoint on land
  PASS: 60100 -> North America (Charleston) no waypoint on land
  PASS: 60200 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60200 -> Brazil (Bahia) no waypoint on land
  PASS: 60200 -> North America (Charleston) no waypoint on land
  PASS: 60300 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60300 -> Brazil (Bahia) no waypoint on land
  PASS: 60300 -> North America (Charleston) no waypoint on land
  PASS: 60400 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60400 -> Brazil (Bahia) no waypoint on land
  PASS: 60400 -> North America (Charleston) no waypoint on land
  PASS: 60500 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60500 -> Brazil (Bahia) no waypoint on land
  PASS: 60500 -> North America (Charleston) no waypoint on land
  PASS: 60600 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60600 -> Brazil (Bahia) no waypoint on land
  PASS: 60600 -> North America (Charleston) no waypoint on land
  PASS: 60700 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60700 -> Brazil (Bahia) no waypoint on land
  PASS: 60700 -> North America (Charleston) no waypoint on land
  PASS: 60800 -> Caribbean (Jamaica) no waypoint on land
  PASS: 60800 -> Brazil (Bahia) no waypoint on land
  PASS: 60800 -> North America (Charleston) no waypoint on land

--- 3. SE Africa route has point south of -30 lat ---
  PASS: 60800 -> Caribbean (Jamaica) has point < -30 lat
  PASS: 60800 -> Brazil (Bahia) has point < -30 lat
  PASS: 60800 -> North America (Charleston) has point < -30 lat

--- 4. Waypoints differ for different destinations ---
  PASS: 60100 Caribbean vs Brazil differ
  PASS: 60200 Caribbean vs Brazil differ
  PASS: 60300 Caribbean vs Brazil differ
  PASS: 60400 Caribbean vs Brazil differ
  PASS: 60500 Caribbean vs Brazil differ
  PASS: 60600 Caribbean vs Brazil differ
  PASS: 60700 Caribbean vs Brazil differ
  PASS: 60800 Caribbean vs Brazil differ
  PASS: 60900 Caribbean vs Brazil differ

--- 5. Senegambia gets waypoints ---
  PASS: 60100 -> Caribbean has waypoints
  PASS: 60100 -> Brazil has waypoints

--- 6. Cache returns identical result on second call ---
  PASS: Cached result is same object

--- 7. Fallback: null when _landFeatures is null ---
  PASS: Returns null without _landFeatures

--- 8. Adjacent stems (60500 + 60600) produce different waypoints ---
  PASS: 60500 and 60600 paths differ for Caribbean

--- 9. I-Am backward compat: TA routes still work with new code ---
  PASS: 50200 -> Rio de la Plata doesn't error
  PASS: 50400 -> Rio de la Plata doesn't error
  PASS: 35100 -> Rio de la Plata doesn't error
  PASS: 21300 -> Rio de la Plata doesn't error
  PASS: 34200 -> Rio de la Plata doesn't error
  PASS: TA 60100 -> Caribbean still works
  PASS: TA 60200 -> Caribbean still works
  PASS: TA 60300 -> Caribbean still works
  PASS: TA 60400 -> Caribbean still works
  PASS: TA 60500 -> Caribbean still works
  PASS: TA 60600 -> Caribbean still works
  PASS: TA 60700 -> Caribbean still works
  PASS: TA 60800 -> Caribbean still works
  PASS: TA 60900 -> Caribbean still works

============================================
Total: 82  Passed: 82  Failed: 0
ALL TESTS PASSED
```

tests/stem-intersection-test.html
```
Stem Intersection & Convergence Ocean Tests

--- A. Quadrant classification for test destinations ---
  PASS: Jamaica (35100) => caribbean
  PASS: Saint-Domingue (36400) => caribbean
  PASS: Havana (31312) => caribbean
  PASS: Barbados (34200) => caribbean
  PASS: Martinique (36100) => caribbean
  PASS: Guadeloupe (36200) => caribbean
  PASS: Antigua (33400) => caribbean
  PASS: Trinidad (34500) => caribbean
  PASS: St. Croix (37010) => caribbean
  PASS: Curacao (32110) => caribbean
  PASS: Grenada (34400) => caribbean
  PASS: Veracruz (41203) => caribbean
  PASS: Cartagena (41207) => caribbean
  PASS: Venezuela (41210) => caribbean
  PASS: Charleston (21300) => north-america
  PASS: New Orleans (21601) => north-america
  PASS: Rhode Island (20100) => north-america
  PASS: Massachusetts (20400) => north-america
  PASS: Georgia (21400) => north-america
  PASS: Virginia (21100) => north-america
  PASS: Pernambuco (50300) => brazil-north
  PASS: Amazonia (50100) => brazil-north
  PASS: French Guiana (36300) => brazil-north
  PASS: Maranhao (50102) => brazil-north
  PASS: Bahia (50200) => brazil-south
  PASS: Rio de Janeiro (50422) => brazil-south
  PASS: Other Brazil (50500) => brazil-south
  PASS: Santos (50428) => brazil-south
  PASS: Buenos Aires (42001) => rio-plata
  PASS: Montevideo (42002) => rio-plata
  PASS: Lisbon (10203) => europe
  PASS: Bristol (10404) => europe
  PASS: Liverpool (10432) => europe
  PASS: Nantes (10720) => europe
  PASS: Bordeaux (10703) => europe
  PASS: Amsterdam (10810) => europe
  PASS: Cadiz (10104) => europe
  PASS: Copenhagen (10901) => europe

--- B. Convergence point is over ocean for all destinations ---
  PASS: Jamaica (35100) convergePt over ocean
  PASS: Saint-Domingue (36400) convergePt over ocean
  PASS: Havana (31312) convergePt over ocean
  PASS: Barbados (34200) convergePt over ocean
  PASS: Martinique (36100) convergePt over ocean
  PASS: Guadeloupe (36200) convergePt over ocean
  PASS: Antigua (33400) convergePt over ocean
  PASS: Trinidad (34500) convergePt over ocean
  PASS: St. Croix (37010) convergePt over ocean
  PASS: Curacao (32110) convergePt over ocean
  PASS: Grenada (34400) convergePt over ocean
  PASS: Veracruz (41203) convergePt over ocean
  PASS: Cartagena (41207) convergePt over ocean
  PASS: Venezuela (41210) convergePt over ocean
  PASS: Charleston (21300) convergePt over ocean
  PASS: New Orleans (21601) convergePt over ocean
  PASS: Rhode Island (20100) convergePt over ocean
  PASS: Massachusetts (20400) convergePt over ocean
  PASS: Georgia (21400) convergePt over ocean
  PASS: Virginia (21100) convergePt over ocean
  PASS: Pernambuco (50300) convergePt over ocean
  PASS: Amazonia (50100) convergePt over ocean
  PASS: French Guiana (36300) convergePt over ocean
  PASS: Maranhao (50102) convergePt over ocean
  PASS: Bahia (50200) convergePt over ocean
  PASS: Rio de Janeiro (50422) convergePt over ocean
  PASS: Other Brazil (50500) convergePt over ocean
  PASS: Santos (50428) convergePt over ocean
  PASS: Buenos Aires (42001) convergePt over ocean
  PASS: Montevideo (42002) convergePt over ocean
  PASS: Lisbon (10203) convergePt over ocean
  PASS: Bristol (10404) convergePt over ocean
  PASS: Liverpool (10432) convergePt over ocean
  PASS: Nantes (10720) convergePt over ocean
  PASS: Bordeaux (10703) convergePt over ocean
  PASS: Amsterdam (10810) convergePt over ocean
  PASS: Cadiz (10104) convergePt over ocean
  PASS: Copenhagen (10901) convergePt over ocean

--- C. No stem crossings (excluding convergence/source zones) ---
  FAIL: Jamaica (35100) no stem crossings (60200 x 60300 at 519,295)
  FAIL: Saint-Domingue (36400) no stem crossings (60200 x 60300 at 521,296)
  FAIL: Havana (31312) no stem crossings (60100 x 60200 at 499,279)
  FAIL: Barbados (34200) no stem crossings (60100 x 60200 at 554,293)
  FAIL: Martinique (36100) no stem crossings (60100 x 60200 at 554,293)
  FAIL: Guadeloupe (36200) no stem crossings (60100 x 60200 at 554,293)
  FAIL: Antigua (33400) no stem crossings (60100 x 60200 at 554,293)
  FAIL: Trinidad (34500) no stem crossings (60100 x 60200 at 554,293)
  FAIL: St. Croix (37010) no stem crossings (60100 x 60200 at 552,292)
  FAIL: Curacao (32110) no stem crossings (60100 x 60200 at 551,292)
  FAIL: Grenada (34400) no stem crossings (60100 x 60200 at 553,293)
  FAIL: Veracruz (41203) no stem crossings (60100 x 60200 at 494,279)
  FAIL: Cartagena (41207) no stem crossings (60200 x 60300 at 520,295)
  FAIL: Venezuela (41210) no stem crossings (60100 x 60200 at 551,292)
  FAIL: Charleston (21300) no stem crossings (60100 x 60200 at 521,277)
  FAIL: New Orleans (21601) no stem crossings (60100 x 60500 at 496,264)
  FAIL: Rhode Island (20100) no stem crossings (60200 x 60300 at 544,294)
  FAIL: Massachusetts (20400) no stem crossings (60200 x 60300 at 545,295)
  FAIL: Georgia (21400) no stem crossings (60100 x 60200 at 521,276)
  FAIL: Virginia (21100) no stem crossings (60100 x 60200 at 523,276)
  FAIL: Pernambuco (50300) no stem crossings (60300 x 60500 at 586,354)
  FAIL: Amazonia (50100) no stem crossings (60300 x 60500 at 587,353)
  FAIL: French Guiana (36300) no stem crossings (60300 x 60500 at 589,354)
  FAIL: Maranhao (50102) no stem crossings (60300 x 60500 at 587,353)
  FAIL: Bahia (50200) no stem crossings (60400 x 60500 at 604,345)
  FAIL: Rio de Janeiro (50422) no stem crossings (60300 x 60500 at 576,364)
  FAIL: Other Brazil (50500) no stem crossings (60300 x 60500 at 581,360)
  FAIL: Santos (50428) no stem crossings (60300 x 60500 at 580,361)
  FAIL: Buenos Aires (42001) no stem crossings (60400 x 60500 at 606,343)
  FAIL: Montevideo (42002) no stem crossings (60400 x 60500 at 606,343)
  FAIL: Lisbon (10203) no stem crossings (60100 x 60200 at 592,239)
  FAIL: Bristol (10404) no stem crossings (60100 x 60200 at 592,238)
  FAIL: Liverpool (10432) no stem crossings (60100 x 60200 at 592,238)
  FAIL: Nantes (10720) no stem crossings (60100 x 60200 at 592,238)
  FAIL: Bordeaux (10703) no stem crossings (60100 x 60200 at 592,237)
  FAIL: Amsterdam (10810) no stem crossings (60100 x 60200 at 592,236)
  FAIL: Cadiz (10104) no stem crossings (60100 x 60200 at 592,239)
  FAIL: Copenhagen (10901) no stem crossings (60100 x 60200 at 595,220)

--- D. Adjacent stem coupling lines do not cross (parallel behavior) ---
  PASS: Jamaica 60100↔60200 coupling ok
  FAIL: Jamaica 60200↔60500 coupling ok (coupling idx 0, 31 crossings)
  FAIL: Jamaica 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Jamaica 60300↔60400 coupling ok
  FAIL: Jamaica 60400↔60600 coupling ok (coupling idx 8, 18 crossings)
  FAIL: Jamaica 60600↔60900 coupling ok (coupling idx 45, 2 crossings)
  PASS: Jamaica 60900↔60700 coupling ok
  PASS: Jamaica 60700↔60800 coupling ok
  PASS: Saint-Domingue 60100↔60200 coupling ok
  FAIL: Saint-Domingue 60200↔60500 coupling ok (coupling idx 0, 25 crossings)
  FAIL: Saint-Domingue 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Saint-Domingue 60300↔60400 coupling ok
  FAIL: Saint-Domingue 60400↔60600 coupling ok (coupling idx 9, 18 crossings)
  FAIL: Saint-Domingue 60600↔60900 coupling ok (coupling idx 48, 1 crossings)
  PASS: Saint-Domingue 60900↔60700 coupling ok
  PASS: Saint-Domingue 60700↔60800 coupling ok
  FAIL: Havana 60100↔60200 coupling ok (coupling idx 74, 2 crossings)
  FAIL: Havana 60200↔60500 coupling ok (coupling idx 1, 14 crossings)
  FAIL: Havana 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Havana 60300↔60400 coupling ok (coupling idx 2, 1 crossings)
  FAIL: Havana 60400↔60600 coupling ok (coupling idx 8, 17 crossings)
  FAIL: Havana 60600↔60900 coupling ok (coupling idx 44, 1 crossings)
  PASS: Havana 60900↔60700 coupling ok
  PASS: Havana 60700↔60800 coupling ok
  FAIL: Barbados 60100↔60200 coupling ok (coupling idx 57, 1 crossings)
  FAIL: Barbados 60200↔60500 coupling ok (coupling idx 0, 16 crossings)
  FAIL: Barbados 60500↔60300 coupling ok (coupling idx 0, 22 crossings)
  FAIL: Barbados 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Barbados 60400↔60600 coupling ok (coupling idx 10, 16 crossings)
  FAIL: Barbados 60600↔60900 coupling ok (coupling idx 50, 2 crossings)
  PASS: Barbados 60900↔60700 coupling ok
  FAIL: Barbados 60700↔60800 coupling ok (coupling idx 107, 3 crossings)
  PASS: Martinique 60100↔60200 coupling ok
  FAIL: Martinique 60200↔60500 coupling ok (coupling idx 0, 16 crossings)
  FAIL: Martinique 60500↔60300 coupling ok (coupling idx 0, 22 crossings)
  FAIL: Martinique 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Martinique 60400↔60600 coupling ok (coupling idx 10, 18 crossings)
  FAIL: Martinique 60600↔60900 coupling ok (coupling idx 51, 1 crossings)
  PASS: Martinique 60900↔60700 coupling ok
  FAIL: Martinique 60700↔60800 coupling ok (coupling idx 107, 3 crossings)
  PASS: Guadeloupe 60100↔60200 coupling ok
  FAIL: Guadeloupe 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Guadeloupe 60500↔60300 coupling ok (coupling idx 0, 22 crossings)
  FAIL: Guadeloupe 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Guadeloupe 60400↔60600 coupling ok (coupling idx 10, 17 crossings)
  FAIL: Guadeloupe 60600↔60900 coupling ok (coupling idx 51, 1 crossings)
  PASS: Guadeloupe 60900↔60700 coupling ok
  FAIL: Guadeloupe 60700↔60800 coupling ok (coupling idx 107, 2 crossings)
  PASS: Antigua 60100↔60200 coupling ok
  FAIL: Antigua 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Antigua 60500↔60300 coupling ok (coupling idx 0, 22 crossings)
  FAIL: Antigua 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Antigua 60400↔60600 coupling ok (coupling idx 10, 17 crossings)
  FAIL: Antigua 60600↔60900 coupling ok (coupling idx 51, 1 crossings)
  PASS: Antigua 60900↔60700 coupling ok
  FAIL: Antigua 60700↔60800 coupling ok (coupling idx 107, 2 crossings)
  PASS: Trinidad 60100↔60200 coupling ok
  FAIL: Trinidad 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Trinidad 60500↔60300 coupling ok (coupling idx 0, 22 crossings)
  FAIL: Trinidad 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Trinidad 60400↔60600 coupling ok (coupling idx 10, 17 crossings)
  FAIL: Trinidad 60600↔60900 coupling ok (coupling idx 51, 1 crossings)
  PASS: Trinidad 60900↔60700 coupling ok
  FAIL: Trinidad 60700↔60800 coupling ok (coupling idx 107, 2 crossings)
  PASS: St. Croix 60100↔60200 coupling ok
  FAIL: St. Croix 60200↔60500 coupling ok (coupling idx 0, 16 crossings)
  FAIL: St. Croix 60500↔60300 coupling ok (coupling idx 0, 13 crossings)
  FAIL: St. Croix 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: St. Croix 60400↔60600 coupling ok (coupling idx 9, 20 crossings)
  PASS: St. Croix 60600↔60900 coupling ok
  PASS: St. Croix 60900↔60700 coupling ok
  FAIL: St. Croix 60700↔60800 coupling ok (coupling idx 108, 2 crossings)
  FAIL: Curacao 60100↔60200 coupling ok (coupling idx 54, 1 crossings)
  FAIL: Curacao 60200↔60500 coupling ok (coupling idx 0, 21 crossings)
  FAIL: Curacao 60500↔60300 coupling ok (coupling idx 0, 13 crossings)
  FAIL: Curacao 60300↔60400 coupling ok (coupling idx 1, 4 crossings)
  FAIL: Curacao 60400↔60600 coupling ok (coupling idx 9, 22 crossings)
  FAIL: Curacao 60600↔60900 coupling ok (coupling idx 49, 1 crossings)
  PASS: Curacao 60900↔60700 coupling ok
  PASS: Curacao 60700↔60800 coupling ok
  PASS: Grenada 60100↔60200 coupling ok
  FAIL: Grenada 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Grenada 60500↔60300 coupling ok (coupling idx 0, 21 crossings)
  FAIL: Grenada 60300↔60400 coupling ok (coupling idx 2, 2 crossings)
  FAIL: Grenada 60400↔60600 coupling ok (coupling idx 10, 17 crossings)
  FAIL: Grenada 60600↔60900 coupling ok (coupling idx 51, 1 crossings)
  PASS: Grenada 60900↔60700 coupling ok
  FAIL: Grenada 60700↔60800 coupling ok (coupling idx 107, 2 crossings)
  PASS: Veracruz 60100↔60200 coupling ok
  FAIL: Veracruz 60200↔60500 coupling ok (coupling idx 0, 16 crossings)
  FAIL: Veracruz 60500↔60300 coupling ok (coupling idx 0, 14 crossings)
  FAIL: Veracruz 60300↔60400 coupling ok (coupling idx 2, 1 crossings)
  FAIL: Veracruz 60400↔60600 coupling ok (coupling idx 7, 15 crossings)
  FAIL: Veracruz 60600↔60900 coupling ok (coupling idx 39, 1 crossings)
  PASS: Veracruz 60900↔60700 coupling ok
  FAIL: Veracruz 60700↔60800 coupling ok (coupling idx 73, 11 crossings)
  PASS: Cartagena 60100↔60200 coupling ok
  FAIL: Cartagena 60200↔60500 coupling ok (coupling idx 0, 30 crossings)
  FAIL: Cartagena 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Cartagena 60300↔60400 coupling ok
  FAIL: Cartagena 60400↔60600 coupling ok (coupling idx 8, 17 crossings)
  FAIL: Cartagena 60600↔60900 coupling ok (coupling idx 46, 1 crossings)
  PASS: Cartagena 60900↔60700 coupling ok
  PASS: Cartagena 60700↔60800 coupling ok
  FAIL: Venezuela 60100↔60200 coupling ok (coupling idx 55, 1 crossings)
  FAIL: Venezuela 60200↔60500 coupling ok (coupling idx 0, 18 crossings)
  FAIL: Venezuela 60500↔60300 coupling ok (coupling idx 0, 13 crossings)
  FAIL: Venezuela 60300↔60400 coupling ok (coupling idx 1, 6 crossings)
  FAIL: Venezuela 60400↔60600 coupling ok (coupling idx 9, 24 crossings)
  FAIL: Venezuela 60600↔60900 coupling ok (coupling idx 50, 1 crossings)
  PASS: Venezuela 60900↔60700 coupling ok
  FAIL: Venezuela 60700↔60800 coupling ok (coupling idx 113, 1 crossings)
  FAIL: Charleston 60100↔60200 coupling ok (coupling idx 64, 7 crossings)
  FAIL: Charleston 60200↔60500 coupling ok (coupling idx 0, 18 crossings)
  FAIL: Charleston 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Charleston 60300↔60400 coupling ok
  FAIL: Charleston 60400↔60600 coupling ok (coupling idx 7, 15 crossings)
  FAIL: Charleston 60600↔60900 coupling ok (coupling idx 36, 3 crossings)
  PASS: Charleston 60900↔60700 coupling ok
  FAIL: Charleston 60700↔60800 coupling ok (coupling idx 61, 23 crossings)
  PASS: New Orleans 60100↔60200 coupling ok
  FAIL: New Orleans 60200↔60500 coupling ok (coupling idx 0, 14 crossings)
  FAIL: New Orleans 60500↔60300 coupling ok (coupling idx 0, 15 crossings)
  PASS: New Orleans 60300↔60400 coupling ok
  FAIL: New Orleans 60400↔60600 coupling ok (coupling idx 6, 17 crossings)
  FAIL: New Orleans 60600↔60900 coupling ok (coupling idx 35, 2 crossings)
  PASS: New Orleans 60900↔60700 coupling ok
  FAIL: New Orleans 60700↔60800 coupling ok (coupling idx 59, 23 crossings)
  PASS: Rhode Island 60100↔60200 coupling ok
  FAIL: Rhode Island 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Rhode Island 60500↔60300 coupling ok (coupling idx 0, 13 crossings)
  FAIL: Rhode Island 60300↔60400 coupling ok (coupling idx 2, 4 crossings)
  FAIL: Rhode Island 60400↔60600 coupling ok (coupling idx 7, 4 crossings)
  FAIL: Rhode Island 60600↔60900 coupling ok (coupling idx 37, 3 crossings)
  PASS: Rhode Island 60900↔60700 coupling ok
  FAIL: Rhode Island 60700↔60800 coupling ok (coupling idx 62, 24 crossings)
  PASS: Massachusetts 60100↔60200 coupling ok
  FAIL: Massachusetts 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Massachusetts 60500↔60300 coupling ok (coupling idx 0, 13 crossings)
  FAIL: Massachusetts 60300↔60400 coupling ok (coupling idx 2, 4 crossings)
  FAIL: Massachusetts 60400↔60600 coupling ok (coupling idx 7, 4 crossings)
  FAIL: Massachusetts 60600↔60900 coupling ok (coupling idx 37, 3 crossings)
  PASS: Massachusetts 60900↔60700 coupling ok
  FAIL: Massachusetts 60700↔60800 coupling ok (coupling idx 62, 24 crossings)
  FAIL: Georgia 60100↔60200 coupling ok (coupling idx 65, 5 crossings)
  FAIL: Georgia 60200↔60500 coupling ok (coupling idx 0, 18 crossings)
  FAIL: Georgia 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Georgia 60300↔60400 coupling ok
  FAIL: Georgia 60400↔60600 coupling ok (coupling idx 7, 18 crossings)
  FAIL: Georgia 60600↔60900 coupling ok (coupling idx 36, 3 crossings)
  PASS: Georgia 60900↔60700 coupling ok
  FAIL: Georgia 60700↔60800 coupling ok (coupling idx 61, 23 crossings)
  FAIL: Virginia 60100↔60200 coupling ok (coupling idx 63, 8 crossings)
  FAIL: Virginia 60200↔60500 coupling ok (coupling idx 0, 17 crossings)
  FAIL: Virginia 60500↔60300 coupling ok (coupling idx 0, 17 crossings)
  PASS: Virginia 60300↔60400 coupling ok
  FAIL: Virginia 60400↔60600 coupling ok (coupling idx 7, 15 crossings)
  FAIL: Virginia 60600↔60900 coupling ok (coupling idx 37, 2 crossings)
  PASS: Virginia 60900↔60700 coupling ok
  FAIL: Virginia 60700↔60800 coupling ok (coupling idx 62, 22 crossings)
  PASS: Pernambuco 60100↔60200 coupling ok
  PASS: Pernambuco 60200↔60500 coupling ok
  FAIL: Pernambuco 60500↔60300 coupling ok (coupling idx 72, 2 crossings)
  PASS: Pernambuco 60300↔60400 coupling ok
  FAIL: Pernambuco 60400↔60600 coupling ok (coupling idx 1, 7 crossings)
  FAIL: Pernambuco 60600↔60900 coupling ok (coupling idx 25, 23 crossings)
  PASS: Pernambuco 60900↔60700 coupling ok
  FAIL: Pernambuco 60700↔60800 coupling ok (coupling idx 57, 21 crossings)
  PASS: Amazonia 60100↔60200 coupling ok
  PASS: Amazonia 60200↔60500 coupling ok
  FAIL: Amazonia 60500↔60300 coupling ok (coupling idx 56, 1 crossings)
  PASS: Amazonia 60300↔60400 coupling ok
  FAIL: Amazonia 60400↔60600 coupling ok (coupling idx 1, 6 crossings)
  FAIL: Amazonia 60600↔60900 coupling ok (coupling idx 22, 22 crossings)
  PASS: Amazonia 60900↔60700 coupling ok
  FAIL: Amazonia 60700↔60800 coupling ok (coupling idx 52, 18 crossings)
  PASS: French Guiana 60100↔60200 coupling ok
  PASS: French Guiana 60200↔60500 coupling ok
  FAIL: French Guiana 60500↔60300 coupling ok (coupling idx 53, 1 crossings)
  PASS: French Guiana 60300↔60400 coupling ok
  FAIL: French Guiana 60400↔60600 coupling ok (coupling idx 1, 6 crossings)
  FAIL: French Guiana 60600↔60900 coupling ok (coupling idx 21, 23 crossings)
  PASS: French Guiana 60900↔60700 coupling ok
  FAIL: French Guiana 60700↔60800 coupling ok (coupling idx 51, 18 crossings)
  PASS: Maranhao 60100↔60200 coupling ok
  PASS: Maranhao 60200↔60500 coupling ok
  FAIL: Maranhao 60500↔60300 coupling ok (coupling idx 61, 1 crossings)
  PASS: Maranhao 60300↔60400 coupling ok
  FAIL: Maranhao 60400↔60600 coupling ok (coupling idx 1, 6 crossings)
  FAIL: Maranhao 60600↔60900 coupling ok (coupling idx 23, 23 crossings)
  PASS: Maranhao 60900↔60700 coupling ok
  FAIL: Maranhao 60700↔60800 coupling ok (coupling idx 54, 19 crossings)
  PASS: Bahia 60100↔60200 coupling ok
  PASS: Bahia 60200↔60500 coupling ok
  PASS: Bahia 60500↔60300 coupling ok
  PASS: Bahia 60300↔60400 coupling ok
  FAIL: Bahia 60400↔60600 coupling ok (coupling idx 1, 7 crossings)
  FAIL: Bahia 60600↔60900 coupling ok (coupling idx 24, 25 crossings)
  PASS: Bahia 60900↔60700 coupling ok
  FAIL: Bahia 60700↔60800 coupling ok (coupling idx 60, 17 crossings)
  PASS: Rio de Janeiro 60100↔60200 coupling ok
  PASS: Rio de Janeiro 60200↔60500 coupling ok
  FAIL: Rio de Janeiro 60500↔60300 coupling ok (coupling idx 69, 6 crossings)
  PASS: Rio de Janeiro 60300↔60400 coupling ok
  FAIL: Rio de Janeiro 60400↔60600 coupling ok (coupling idx 1, 7 crossings)
  FAIL: Rio de Janeiro 60600↔60900 coupling ok (coupling idx 23, 25 crossings)
  PASS: Rio de Janeiro 60900↔60700 coupling ok
  FAIL: Rio de Janeiro 60700↔60800 coupling ok (coupling idx 58, 19 crossings)
  PASS: Other Brazil 60100↔60200 coupling ok
  PASS: Other Brazil 60200↔60500 coupling ok
  FAIL: Other Brazil 60500↔60300 coupling ok (coupling idx 58, 8 crossings)
  PASS: Other Brazil 60300↔60400 coupling ok
  FAIL: Other Brazil 60400↔60600 coupling ok (coupling idx 1, 6 crossings)
  FAIL: Other Brazil 60600↔60900 coupling ok (coupling idx 22, 25 crossings)
  PASS: Other Brazil 60900↔60700 coupling ok
  FAIL: Other Brazil 60700↔60800 coupling ok (coupling idx 56, 27 crossings)
  PASS: Santos 60100↔60200 coupling ok
  PASS: Santos 60200↔60500 coupling ok
  FAIL: Santos 60500↔60300 coupling ok (coupling idx 61, 7 crossings)
  PASS: Santos 60300↔60400 coupling ok
  FAIL: Santos 60400↔60600 coupling ok (coupling idx 1, 7 crossings)
  FAIL: Santos 60600↔60900 coupling ok (coupling idx 23, 23 crossings)
  PASS: Santos 60900↔60700 coupling ok
  FAIL: Santos 60700↔60800 coupling ok (coupling idx 57, 23 crossings)
  PASS: Buenos Aires 60100↔60200 coupling ok
  PASS: Buenos Aires 60200↔60500 coupling ok
  PASS: Buenos Aires 60500↔60300 coupling ok
  PASS: Buenos Aires 60300↔60400 coupling ok
  FAIL: Buenos Aires 60400↔60600 coupling ok (coupling idx 1, 5 crossings)
  FAIL: Buenos Aires 60600↔60900 coupling ok (coupling idx 19, 21 crossings)
  FAIL: Buenos Aires 60900↔60700 coupling ok (coupling idx 89, 2 crossings)
  FAIL: Buenos Aires 60700↔60800 coupling ok (coupling idx 50, 33 crossings)
  PASS: Montevideo 60100↔60200 coupling ok
  PASS: Montevideo 60200↔60500 coupling ok
  PASS: Montevideo 60500↔60300 coupling ok
  PASS: Montevideo 60300↔60400 coupling ok
  FAIL: Montevideo 60400↔60600 coupling ok (coupling idx 1, 5 crossings)
  FAIL: Montevideo 60600↔60900 coupling ok (coupling idx 19, 21 crossings)
  FAIL: Montevideo 60900↔60700 coupling ok (coupling idx 89, 1 crossings)
  FAIL: Montevideo 60700↔60800 coupling ok (coupling idx 51, 32 crossings)
  FAIL: Lisbon 60100↔60200 coupling ok (coupling idx 1, 11 crossings)
  FAIL: Lisbon 60200↔60500 coupling ok (coupling idx 0, 55 crossings)
  FAIL: Lisbon 60500↔60300 coupling ok (coupling idx 0, 26 crossings)
  FAIL: Lisbon 60300↔60400 coupling ok (coupling idx 3, 14 crossings)
  FAIL: Lisbon 60400↔60600 coupling ok (coupling idx 10, 14 crossings)
  FAIL: Lisbon 60600↔60900 coupling ok (coupling idx 21, 27 crossings)
  FAIL: Lisbon 60900↔60700 coupling ok (coupling idx 79, 5 crossings)
  FAIL: Lisbon 60700↔60800 coupling ok (coupling idx 33, 34 crossings)
  FAIL: Bristol 60100↔60200 coupling ok (coupling idx 0, 12 crossings)
  FAIL: Bristol 60200↔60500 coupling ok (coupling idx 0, 55 crossings)
  FAIL: Bristol 60500↔60300 coupling ok (coupling idx 0, 30 crossings)
  FAIL: Bristol 60300↔60400 coupling ok (coupling idx 3, 12 crossings)
  FAIL: Bristol 60400↔60600 coupling ok (coupling idx 10, 13 crossings)
  FAIL: Bristol 60600↔60900 coupling ok (coupling idx 20, 28 crossings)
  FAIL: Bristol 60900↔60700 coupling ok (coupling idx 78, 3 crossings)
  FAIL: Bristol 60700↔60800 coupling ok (coupling idx 33, 38 crossings)
  FAIL: Liverpool 60100↔60200 coupling ok (coupling idx 1, 11 crossings)
  FAIL: Liverpool 60200↔60500 coupling ok (coupling idx 0, 56 crossings)
  FAIL: Liverpool 60500↔60300 coupling ok (coupling idx 0, 31 crossings)
  FAIL: Liverpool 60300↔60400 coupling ok (coupling idx 3, 13 crossings)
  FAIL: Liverpool 60400↔60600 coupling ok (coupling idx 10, 13 crossings)
  FAIL: Liverpool 60600↔60900 coupling ok (coupling idx 20, 30 crossings)
  FAIL: Liverpool 60900↔60700 coupling ok (coupling idx 78, 3 crossings)
  FAIL: Liverpool 60700↔60800 coupling ok (coupling idx 33, 38 crossings)
  FAIL: Nantes 60100↔60200 coupling ok (coupling idx 0, 12 crossings)
  FAIL: Nantes 60200↔60500 coupling ok (coupling idx 0, 55 crossings)
  FAIL: Nantes 60500↔60300 coupling ok (coupling idx 0, 29 crossings)
  FAIL: Nantes 60300↔60400 coupling ok (coupling idx 3, 12 crossings)
  FAIL: Nantes 60400↔60600 coupling ok (coupling idx 10, 13 crossings)
  FAIL: Nantes 60600↔60900 coupling ok (coupling idx 20, 28 crossings)
  FAIL: Nantes 60900↔60700 coupling ok (coupling idx 78, 3 crossings)
  FAIL: Nantes 60700↔60800 coupling ok (coupling idx 33, 40 crossings)
  FAIL: Bordeaux 60100↔60200 coupling ok (coupling idx 0, 12 crossings)
  FAIL: Bordeaux 60200↔60500 coupling ok (coupling idx 0, 55 crossings)
  FAIL: Bordeaux 60500↔60300 coupling ok (coupling idx 0, 29 crossings)
  FAIL: Bordeaux 60300↔60400 coupling ok (coupling idx 3, 13 crossings)
  FAIL: Bordeaux 60400↔60600 coupling ok (coupling idx 10, 13 crossings)
  FAIL: Bordeaux 60600↔60900 coupling ok (coupling idx 20, 28 crossings)
  FAIL: Bordeaux 60900↔60700 coupling ok (coupling idx 77, 3 crossings)
  FAIL: Bordeaux 60700↔60800 coupling ok (coupling idx 33, 40 crossings)
  FAIL: Amsterdam 60100↔60200 coupling ok (coupling idx 0, 11 crossings)
  FAIL: Amsterdam 60200↔60500 coupling ok (coupling idx 0, 52 crossings)
  FAIL: Amsterdam 60500↔60300 coupling ok (coupling idx 0, 26 crossings)
  FAIL: Amsterdam 60300↔60400 coupling ok (coupling idx 3, 12 crossings)
  FAIL: Amsterdam 60400↔60600 coupling ok (coupling idx 10, 12 crossings)
  FAIL: Amsterdam 60600↔60900 coupling ok (coupling idx 20, 29 crossings)
  FAIL: Amsterdam 60900↔60700 coupling ok (coupling idx 76, 4 crossings)
  FAIL: Amsterdam 60700↔60800 coupling ok (coupling idx 33, 39 crossings)
  FAIL: Cadiz 60100↔60200 coupling ok (coupling idx 1, 12 crossings)
  FAIL: Cadiz 60200↔60500 coupling ok (coupling idx 0, 57 crossings)
  FAIL: Cadiz 60500↔60300 coupling ok (coupling idx 0, 33 crossings)
  FAIL: Cadiz 60300↔60400 coupling ok (coupling idx 3, 14 crossings)
  FAIL: Cadiz 60400↔60600 coupling ok (coupling idx 10, 15 crossings)
  FAIL: Cadiz 60600↔60900 coupling ok (coupling idx 20, 29 crossings)
  FAIL: Cadiz 60900↔60700 coupling ok (coupling idx 79, 2 crossings)
  FAIL: Cadiz 60700↔60800 coupling ok (coupling idx 33, 36 crossings)
  FAIL: Copenhagen 60100↔60200 coupling ok (coupling idx 0, 9 crossings)
  FAIL: Copenhagen 60200↔60500 coupling ok (coupling idx 0, 49 crossings)
  FAIL: Copenhagen 60500↔60300 coupling ok (coupling idx 0, 28 crossings)
  FAIL: Copenhagen 60300↔60400 coupling ok (coupling idx 3, 12 crossings)
  FAIL: Copenhagen 60400↔60600 coupling ok (coupling idx 8, 12 crossings)
  FAIL: Copenhagen 60600↔60900 coupling ok (coupling idx 16, 32 crossings)
  FAIL: Copenhagen 60900↔60700 coupling ok (coupling idx 64, 5 crossings)
  FAIL: Copenhagen 60700↔60800 coupling ok (coupling idx 31, 34 crossings)

============================================
Total: 418  Passed: 163  Failed: 255
255 TEST(S) FAILED
```

# The high level framework you proposed.

This is a deep algorithmic problem in your own codebase, so let me analyze the failures and propose a concrete architectural change rather than sending this to web research.

## Diagnosis

Looking at the test results, the pattern is clear:

**Section C (255/255 fail):** Every destination has stem crossings. The current approach in `buildWaypointChain` computes *independent* waypoint chains per source region, then `separateStems` tries post-hoc Fréchet-based nudging. The nudging is insufficient because it only pushes pathB perpendicular to its local flow direction, can't guarantee global non-intersection, and 3 iterations of local perturbation can't resolve topological ordering violations.

**Section D (widespread coupling failures):** Adjacent stems swap relative position along their paths. The pairs 60200↔60500, 60500↔60300, 60400↔60600 consistently fail because independent waypoint chains route through overlapping ocean corridors without respecting a consistent left-to-right ordering.

## Root Cause

The fundamental issue is architectural: you're building **independent paths** then trying to **fix them**. Your mathematical formulation requires the S_i to be non-intersecting with matching derivatives at P — this is a **parallel ribbon** constraint, not a set of independent curves with post-hoc separation.

## Proposed Solution: Spine-and-Ribbon Construction

Replace the current `separateStems` approach with a **spine-based ribbon** that guarantees non-intersection by construction.

### The Algorithm

**Step 1 — Compute a single reference spine** from the weighted centroid of source projections to the convergence point P, using the existing waypoint infrastructure to route it through ocean space:

```
spine(t): [0,1] → R²
```

This spine passes through ocean waypoints derived from `buildWaypointChain` for the "median" source (by latitude ordering).

**Step 2 — Compute the Frenet frame** along the spine. At each parameter value t, you have a tangent vector T(t) and a normal vector N(t) = perp(T(t)). Since the spine is a 2D curve, N is just the 90° rotation of T.

**Step 3 — Assign ordered offsets.** Sort sources by their angular position relative to the spine's start. Source i gets offset d_i where d_1 < d_2 < ... < d_n with consistent spacing (e.g., `STEM_MIN_SPACING`). The offset function tapers:

```
S_i(t) = spine(t) + d_i · α(t) · N(t)
```

where α(t) is a taper function with α(0) = 1, α(1) = 0, and α'(1) = 0 (ensuring all derivatives match at P). A good choice is α(t) = (1-t)² or a smooth hermite blend.

**Step 4 — Source connection.** Each S_i(0) won't exactly equal the source point O_i. Add a short approach segment from O_i to S_i(0) using a simple arc or cubic that blends from the source position to the ribbon entry point.

### Why This Satisfies the Requirements

Your five conditions map directly:

| Requirement | How ribbon satisfies it |
|---|---|
| 1. S_i ∩ S_j = {P} | Monotone offset ordering d_1 < ... < d_n with same spine guarantees non-intersection |
| 2. O are extreme points of Origin | Unchanged — source coordinates stay the same |
| 3. S_i doesn't cross Origin (land) | Spine inherits ocean waypoints from existing `buildWaypointChain`; offsets are small perturbations |
| 4. lim S_i'(1) = lim S_j'(1) | α(1) = 0 and α'(1) = 0 means all S_i'(1) = spine'(1) |
| 5. S_i(1) = P | α(1) = 0 means S_i(1) = spine(1) = P |

And the trunk T(t,d) starts at P with T'(0,d) = spine'(1), matching condition 6.

### Concrete Code Changes

Here's what I'd propose changing in `route-planner.js`:

**1. New function: `computeSpine`** — takes the destination quadrant, convergence point, and source set. Picks the "median" source's waypoint chain as the spine backbone, projects it, and returns a sampled polyline.

**2. New function: `computeFrenetFrame`** — given the spine polyline, computes tangent and normal at each sample point using finite differences.

**3. Replace `separateStems`** with `computeRibbonOffsets` — takes sorted stem infos, the spine, the Frenet frame, and spacing. Returns per-source waypoint chains that are offset copies of the spine with taper.

**4. Modify `generateRoutedPath`** — instead of calling `computeDynamicWaypoints` independently per source, call the ribbon system that computes all stems jointly.

**5. Approach segments** — add a short cubic from each source point to the ribbon entry point, ensuring C1 continuity.

Want me to implement this? I'd start with the spine-and-ribbon core in `route-planner.js`, then update the test expectations. The approach would look roughly like:

```javascript
function computeRibbonStems(stemInfos, convergePt, destCode) {
    // 1. Sort stems by angular position relative to spine direction
    // 2. Compute spine from median source's waypoint chain
    // 3. Sample spine, compute Frenet frame
    // 4. Assign offsets: d_i = (i - (n-1)/2) * STEM_MIN_SPACING
    // 5. For each stem, offset spine by d_i * alpha(t) * N(t)
    // 6. Prepend approach segment from source to ribbon entry
    // 7. Return per-source projected waypoint chains
}
```

The taper function α(t) = (1-t)² gives you C1 at convergence, and a cosine taper α(t) = (1 + cos(πt))/2 gives smoother results if you want C2.

Should I go ahead and implement this, or would you like to discuss the approach further first — particularly around how to handle the SE Africa (60800) route which has a fundamentally different geometry from the West African sources?

These are the two hardest geometric sub-problems in the whole system. Let me work through them carefully.

## SE Africa (60800): The Fundamental Asymmetry

The spine-and-ribbon model assumes all sources approach P from roughly the same direction — you can imagine a bundle of parallel wires fanning out from P backward toward a shared corridor. That works for the West African sources (60100–60700, 60900) because they're strung along a roughly north-south coastline between Senegambia and the Gulf of Guinea, and they all head westward across the Atlantic. Their approach vectors to P are within maybe 40° of each other.

60800 breaks this entirely. It starts on the *east* coast of Africa at roughly (38°E, -18°S), routes south past Cabo Delgado, rounds the Cape at ~(20°E, -34°S), then turns northwest past St. Helena at (-5.7°W, -16°S) before crossing the mid-Atlantic. By the time it approaches P, it's coming from the *south or southeast*, while the West African sources arrive from the *east or northeast*. The approach angle difference can be 90°+ depending on the destination quadrant.

A single spine with offset ribbons can't accommodate this — you'd need an offset so large for 60800 that it would push the path onto land or create a visually absurd detour.

### Proposed: Hierarchical Spine with Late Join

The solution is to decompose the stems into **approach groups** that share a spine within each group, then merge groups at a **pre-convergence junction**:

**Group A (West Coast):** 60100, 60200, 60300, 60400, 60500, 60600, 60700, 60900 — eight sources along the West African coast. These share a single spine with ribbon offsets. The spine runs from a point in the mid-Atlantic (~15°W) westward toward P.

**Group B (Cape Route):** 60800 alone. It has a unique path: coast south, round the Cape, northwest past St. Helena, then a long crossing.

The two groups merge at a **junction point J** that sits upstream of P:

```
Group A sources ──ribbon──→ J ──→ P ──trunk──→ destination
                              ↗
60800 ──cape route──────────→ J
```

At J, 60800's path must join the ribbon without crossing any Group A stems. Since 60800 approaches from the south and Group A approaches from the east/northeast, 60800 naturally enters the ribbon at the southern edge. This means 60800 should be assigned the most-southern offset in the ribbon, and J should be positioned where the Cape route naturally meets the West African corridor — roughly in the vicinity of St. Helena or slightly west of it, depending on destination.

The taper constraint still holds: all stems must have matching derivatives at P. Since J is upstream of P, the ribbon from J to P provides that — all stems (including 60800) are offset copies of the same spine segment from J to P, so they naturally converge with matching tangents.

The approach segment from 60800's St. Helena waypoint to its ribbon entry at J is where the join happens. This needs to be a smooth cubic that blends 60800's arrival direction (roughly northwest) into the ribbon's flow direction. The key geometric constraint is: 60800's entry point on the ribbon must be on the correct side (southern edge) so it doesn't cross the Group A stems.

### Sorting Logic for the Ribbon

Within Group A, the natural ordering is by latitude of the source — Senegambia (60100, 14.5°N) at the northern edge of the ribbon, West Central Africa (60700, -6.5°S) at the southern edge. 60900 (Other Africa, centered at 1°N, 20°E) is trickier — it's inland and needs routing to the coast first, but once it reaches its offshore point, it should slot in by latitude.

60800 then enters at the bottom (south of 60700), maintaining the overall south-to-north ordering of the ribbon cross-section.

### What J Looks Like per Destination Quadrant

The junction point J should vary by destination:

For **Caribbean / North America** destinations, J is in the central Atlantic around (-25°W, -5°S to 5°N). Group A's spine runs east-to-west through this zone. 60800 arrives from the southeast (St. Helena → northwest). J sits where the Cape route naturally intersects the Atlantic corridor.

For **Brazil** destinations, J shifts south and east, maybe (-20°W, -10°S). The Cape route is shorter here — 60800 might only need St. Helena plus one waypoint before joining the ribbon.

For **Rio de la Plata**, J shifts further south (-25°W, -20°S). Here 60800's path is actually more direct than for Caribbean destinations, and the approach angle difference between Group A and 60800 is smaller — both are heading southwest.

For **Europe** destinations, J is in the north Atlantic (-15°W, 10°N to 20°N). 60800 has a long journey north past St. Helena, and the ribbon's spine runs roughly south-to-north. 60800 enters at the southern end of the ribbon as it sweeps northward.

## Inter-American Trade: The Americas Geometry

The inter-American problem is structurally different from the transatlantic one. Instead of a roughly parallel set of sources on one coast heading to destinations on another coast, you have sources *scattered around* the Americas (Brazil, Caribbean, North America) heading to destinations that can be anywhere from Cuba to Chile.

### Source Groups for Inter-American

The sources fall into natural geographic clusters:

**Brazilian sources** (50100–50500): Strung along the Brazilian coast from Amazonia (-2°S) to SE Brazil (-23°S). These are roughly collinear along a north-south coastline.

**Caribbean sources** (31xxx–39xxx): Scattered across the island arc from Trinidad to Cuba. These have significant east-west spread.

**North American sources** (20xxx–21xxx): US East Coast from Massachusetts to Gulf Coast. North-south spread with significant distance from Caribbean sources.

**European sources** (10xxx, rare in I-Am): Occasional, can be treated as simple arcs.

### The Pacific Destination Problem

For Caribbean or Brazilian destinations, the I-Am geometry is relatively benign — sources and destinations are on the same ocean basin, and P can sit in open Atlantic or Caribbean water. The existing `computeIAmConvergence` handles this reasonably.

For **Rio de la Plata** (42xxx), the challenge is moderate: sources are north of the destination, and P needs to be in the South Atlantic offshore of the Brazilian coast. The main land avoidance concern is the Brazilian bulge — stems from Caribbean or North American sources must route east of the Ponta do Seixas (-7.15°S, -33.8°W) to avoid cutting through Brazil.

For **Pacific South** destinations — Lima (43001) and Talcahuano (44002) — the problem is severe. These destinations are on the Pacific coast of South America. Any route from the Atlantic side must either go around Cape Horn or through the Strait of Magellan region. This means:

P cannot be in the Atlantic for Pacific destinations. It must be in the Pacific, or at the very southern tip of South America where Atlantic and Pacific effectively meet.

Every stem — whether from Brazil, the Caribbean, or North America — must route south along the Atlantic coast, round the southern tip, then north up the Pacific coast. This is analogous to the 60800 Cape route but in reverse and for *all* sources, not just one.

### Proposed: Route Corridor System for I-Am

Rather than a single ribbon approach, I'd propose a **corridor system** where the path from any source to any destination follows a sequence of **corridor segments** that are shared infrastructure:

**Atlantic Northbound Corridor**: Caribbean → Florida Straits → US East Coast (for north-going routes)

**Atlantic Southbound Corridor**: Caribbean → Lesser Antilles → Ponta do Seixas → Brazilian coast → Rio de la Plata (for south-going routes)

**Cape Horn Corridor**: Rio de la Plata → south past Falklands → round Cape Horn at ~(-56°S, -67°W) → north up Chilean coast (for Pacific destinations)

**Intra-Caribbean Corridor**: Short arcs between nearby Caribbean islands (no waypoints needed — simple arcs suffice)

For a specific source-destination pair, the route planner selects which corridor segments to use:

```
Brazil → Lima:   Ponta do Seixas → south coast → Cape Horn corridor → Pacific northbound
Caribbean → Lima: Southbound corridor → Cape Horn corridor → Pacific northbound
N. America → Lima: Southbound corridor → Cape Horn corridor → Pacific northbound
Brazil → Rio de la Plata: Coast southbound (short, direct)
Caribbean → Rio de la Plata: Southbound corridor → coast
```

Within each corridor segment, the ribbon construction applies — sources using the same corridor get parallel offsets. The ordering within the ribbon is determined by source latitude: northernmost source on the outer (eastern) edge, southernmost on the inner (western/coastal) edge for southbound corridors, and vice versa.

### Where P Goes for Pacific Destinations

For Pacific South destinations, P should be positioned in the Pacific, offshore of the destination:

For Lima (-12°S, -77°W): P at roughly (-15°S, -80°W) — open Pacific west of Peru.

For Talcahuano (-36.7°S, -73.1°W): P at roughly (-38°S, -76°W) — open Pacific west of Chile.

The trunk T(t,d) is then a short segment from P to the destination along the coast. The stems arrive at P from the south (having come up through the Cape Horn corridor) or from the north (if there are Pacific-side sources, which is rare in the I-Am data).

The critical constraint is that P must not be on land. The Andes are a narrow but tall barrier, so P needs to be sufficiently west (offshore) to clear the continental shelf. A longitude of about -78°W to -82°W should work for most Pacific coast destinations.

### Compound Problem: SE Africa → Pacific Destination

This is the extreme case. An SE Africa source going to Lima would need to round *two* capes: Cape of Good Hope, then Cape Horn. The waypoint chain is enormous, but it's historically accurate — these routes existed.

In the current code, this case likely produces a garbage path because `buildWaypointChain` for 60800 has no Pacific-destination logic. The chain would need:

```
60800 → Cape Agulhas → west past St. Helena → South Atlantic → 
  join southbound corridor → Cape Horn → Pacific northbound → P (off Lima)
```

This is a concatenation of the 60800-specific approach (Cape route) with the I-Am Cape Horn corridor. The spine would be this full chain, and 60800 would be the only source using it (since no other African sources went to Pacific destinations in significant numbers — if there are data rows for this, they're statistical noise).

## Summary of Architecture

The proposal breaks into three layers:

**Layer 1 — Corridor infrastructure**: Pre-defined ocean corridors (Atlantic southbound, Cape Horn, Pacific northbound, etc.) as sequences of waypoints. These are shared across all source-destination pairs.

**Layer 2 — Spine construction**: For a given destination, select the appropriate corridor segments, concatenate them, and smooth into a single spine from the mid-ocean approach region to P.

**Layer 3 — Ribbon offsets**: Sort active sources by their natural position relative to the spine, assign offsets with taper, add approach segments from source to ribbon entry.

For the SE Africa exception in transatlantic routes: hierarchical merge with a junction point J where the Cape route joins the West African ribbon.

For Pacific I-Am routes: the Cape Horn corridor provides the routing backbone, with P positioned in the Pacific offshore of the destination.

Should I start implementing this, or do you want to dig into any particular aspect — like the taper function choice, the exact corridor waypoint sequences, or how the junction point J should be computed dynamically?

Good — these are the two most load-bearing design decisions in the whole ribbon system. Let me work through them carefully.

## Taper Function Choice

### What the taper must satisfy

Recall the ribbon construction:

```
S_i(t) = spine(t) + d_i · α(t) · N(t)
```

where `spine(t)` is the reference curve, `d_i` is the signed offset for source i, `N(t)` is the unit normal to the spine, and `α(t)` is the taper. Differentiating:

```
S_i'(t) = spine'(t) + d_i · [α'(t) · N(t) + α(t) · N'(t)]
```

Your requirement 4 says all stems must have matching derivatives at P, meaning S_i'(1) = S_j'(1) for all i,j. Since d_i ≠ d_j in general, the bracketed term must vanish at t=1:

```
α'(1) · N(1) + α(1) · N'(1) = 0
```

N(1) is a unit vector (it can't be zero), and N'(1) may or may not be zero depending on spine curvature. The cleanest sufficient condition that works regardless of spine geometry is:

```
α(1) = 0   AND   α'(1) = 0
```

These are the **hard constraints**. Beyond them, we have soft desiderata: monotonicity (so offsets don't change sign and cause crossings), visual naturalness, and optionally C2 at both endpoints for smoother rendering.

### Candidate analysis

**α(t) = (1-t)²**

The simplest option. α(0)=1, α(1)=0, α'(1)=0 — satisfies the hard constraints. But α'(0) = -2, meaning the ribbon starts converging immediately at full rate. Visually, the stems begin pinching together right from their entry point, which looks rushed. It also means the approach segment from the source to the ribbon entry has a discontinuity in convergence rate — the source arc arrives at a point where the ribbon is already actively narrowing.

**α(t) = ½(1 + cos πt)**

The raised cosine. Satisfies both endpoint conditions: α(0)=1, α(1)=0, α'(0)=0, α'(1)=0. The zero derivative at t=0 is a bonus — it means stems run parallel for a stretch near the ribbon entry before they begin converging. This creates a visual "holding zone" where the separate origins are clearly distinguishable before they funnel toward P.

The issue is α''(1) = π²/2 ≠ 0, so it's only C1 at the convergence point. For a Catmull-Rom or cubic Bézier rendering pipeline this doesn't matter — you won't see the C2 discontinuity. But if you ever move to a higher-order spline representation, it could introduce a subtle kink at P.

The other issue is the rate profile. α'(t) = -(π/2)sin(πt), which peaks at t=0.5. The maximum convergence rate is π/2 ≈ 1.57. This means the stems converge fastest in the middle of their journey — a visually pleasing "S-curve" profile where the ribbon holds width, then narrows, then gently closes.

**α(t) = 1 - 3t² + 2t³  (smoothstep, i.e. Hermite basis H₀₀)**

This is the cubic Hermite interpolant with α(0)=1, α(1)=0, α'(0)=0, α'(1)=0. Identical endpoint conditions to the cosine taper. The convergence rate is α'(t) = -6t(1-t), peaking at t=0.5 with magnitude 1.5 — nearly identical to the cosine.

The difference is in the higher derivatives: α''(1) = 6 ≠ 0 (same C1-only as cosine). And the shape is marginally different in the middle — the cubic is slightly "flatter" near the endpoints and slightly "sharper" in transition compared to cosine. In practice, visually indistinguishable.

The advantage over cosine is computational: polynomial evaluation is cheaper than a trig call, and more importantly, the derivative is a simple polynomial too, which matters if you ever need to compute exact tangent vectors for the Frenet frame rather than finite differences.

**α(t) = 6t⁵ - 15t⁴ + 10t³ subtracted from 1, i.e. α(t) = 1 - 6t⁵ + 15t⁴ - 10t³  (smootherstep)**

The quintic Hermite with α(0)=1, α(1)=0, α'(0)=0, α'(1)=0, **and** α''(0)=0, α''(1)=0. This is C2 at both endpoints. The convergence rate is α'(t) = -30t²(1-t)², which is zero at both ends and peaks at t=0.5 with magnitude 30/16 ≈ 1.875.

The C2 property means the curvature of each stem is continuous as it enters the ribbon and as it arrives at P. In a Bézier or Catmull-Rom rendering this is overkill — but it provides a safety margin. If the rendering pipeline ever introduces artifacts at parameter boundaries (which Catmull-Rom can do at knot points), C2 input data helps.

The monotonicity is guaranteed: α'(t) = -30t²(1-t)² ≤ 0 for all t ∈ [0,1].

### My recommendation

Use the **smoothstep** (cubic Hermite) as the default, with the option to swap in smootherstep if you see artifacts:

```javascript
function taperAlpha(t) {
    // Hermite H00: α(0)=1, α(1)=0, α'(0)=0, α'(1)=0
    return 1 - 3*t*t + 2*t*t*t;
}
```

The reasoning: it's the simplest function that gives you zero derivatives at both endpoints (so stems run parallel at entry and merge smoothly at P), it's a polynomial (cheap to evaluate and differentiate), it's monotone, and it matches the visual intuition of "hold width → converge → merge." The cosine gives you functionally the same result but with unnecessary trig. The quadratic (1-t)² lacks the zero-derivative-at-entry property and will look visually worse. The quintic is available if needed but adds complexity for minimal visual gain at the spline rendering level you're working at.

### One nuance: non-uniform parameterization

The taper function assumes t ∈ [0,1] is parameterized uniformly along the spine. But the spine's arc length between waypoints is non-uniform — the segment from St. Helena to mid-Atlantic is much longer than the segment through the Gold Coast offshore zone. If you parameterize t by waypoint index rather than arc length, the taper will behave differently in different segments.

For visual quality, you should **reparameterize by arc length**. Sample the spine at uniform arc-length intervals, then evaluate α at the corresponding normalized parameter. This ensures the convergence rate is spatially uniform — the ribbon narrows at a consistent rate in screen pixels, not in waypoint-index space.

In `route-planner.js`, this means: after computing the spine polyline, compute cumulative arc lengths, normalize to [0,1], and use those normalized values as the t for each sample point when computing offsets.

## Junction Point J: Dynamic Computation

### The problem precisely

Group A (West Coast sources) travels along a spine from some mid-Atlantic approach region to P. Source 60800 (SE Africa) takes the Cape route — south around Africa, northwest past St. Helena, across the South Atlantic. At some point, 60800's path must merge into the Group A ribbon so that from the merge point onward, all nine stems share the same spine and converge together at P.

J is the point where this merge happens. It must satisfy:

1. **Ocean constraint**: J is not on land.
2. **Non-crossing constraint**: 60800 enters the ribbon on the correct side (southern edge) so it doesn't cross any Group A stem.
3. **Smoothness constraint**: The transition from 60800's independent path to its ribbon offset should be at least C1 — no visible kink.
4. **Visual constraint**: J should be far enough upstream of P that the merge looks natural, not like 60800 suddenly teleports into the ribbon at the last moment.

### Computing J: the tangent alignment approach

The key geometric insight is that J should be where 60800's natural travel direction aligns with the ribbon's flow direction. Before this point, 60800 is heading in a different direction than Group A (say, northwest vs. west). After this point, 60800 should be traveling roughly parallel to Group A, just offset to the south.

Formally: let **h**(s) be the 60800 cape-route path parameterized by arc length, and let **spine**(t) be the Group A spine. Define the alignment angle:

```
θ(s, t) = angle between h'(s) and spine'(t)
```

J is at the (s*, t*) that minimizes θ subject to the constraint that **h**(s*) is geometrically close to **spine**(t*) — close enough for a smooth connecting curve.

In practice, you don't need to solve this optimization analytically. Here's a discrete algorithm:

**Step 1.** Compute the 60800 cape-route waypoint chain (Cape Agulhas → west of Cape → St. Helena → mid-Atlantic approach). Call the last point in this chain H — the point where 60800 reaches the general Atlantic corridor.

**Step 2.** Compute the Group A spine. Sample it at, say, 20 uniform arc-length points. At each sample point spine(t_k), compute the tangent direction as the normalized vector (spine(t_{k+1}) - spine(t_{k-1})).

**Step 3.** Compute the direction from H to each spine sample: **v**_k = normalize(spine(t_k) - H).

**Step 4.** For each k, compute the angle between **v**_k and spine'(t_k). This measures how well 60800 can approach spine(t_k) while matching the spine's flow direction.

**Step 5.** J = spine(t_k*) where k* minimizes this angle, subject to: t_k* is in the first 70% of the spine (don't merge too close to P), and J is over ocean.

```javascript
function computeJunction(capeEndpoint, spineSamples, spineTangents, landFeatures) {
    var bestK = -1, bestAngle = Infinity;
    var maxK = Math.floor(spineSamples.length * 0.7); // don't merge too late
    
    for (var k = 1; k < maxK; k++) {
        // Direction from cape endpoint to this spine point
        var dx = spineSamples[k][0] - capeEndpoint[0];
        var dy = spineSamples[k][1] - capeEndpoint[1];
        var len = Math.sqrt(dx*dx + dy*dy);
        if (len < 1) continue;
        var vx = dx / len, vy = dy / len;
        
        // Angle to spine tangent
        var dot = vx * spineTangents[k][0] + vy * spineTangents[k][1];
        var angle = Math.acos(Math.max(-1, Math.min(1, dot)));
        
        // Check ocean
        var geo = projection.invert(spineSamples[k]);
        if (geo && isPointOnLand(geo[1], geo[0], landFeatures)) continue;
        
        if (angle < bestAngle) {
            bestAngle = angle;
            bestK = k;
        }
    }
    
    return bestK >= 0 ? spineSamples[bestK] : null;
}
```

### What J looks like per destination quadrant

The algorithm above will naturally produce different J positions depending on where the spine goes:

**Caribbean destinations**: The spine runs roughly east→west through the tropical Atlantic. 60800's cape route arrives from the south-southeast (heading northwest after St. Helena). The tangent alignment happens relatively late — somewhere around (-20°W, 0°–5°N), where the cape route has swung far enough west that its northwesterly heading aligns with the spine's westerly heading. J is in the equatorial mid-Atlantic.

**Brazil destinations**: The spine heads southwest toward Brazil. 60800's cape route, after St. Helena, is also heading roughly southwest. The tangent alignment happens earlier and further east — around (-10°W, -5°S to -10°S). J is in the South Atlantic, not far west of St. Helena.

**Rio de la Plata**: The spine heads due south. After St. Helena, 60800 turns south-southwest. Alignment happens around (-15°W, -15°S to -20°S). J is in the central South Atlantic.

**Europe destinations**: The spine heads north-northeast. After St. Helena, 60800 would need to turn north — this is the worst case for alignment angle because the cape route has strong southwesterly momentum that must be reversed. J ends up somewhere around (-10°W, 5°N–10°N) where 60800 has finally turned northward enough. The connecting curve from H to J will have significant curvature here.

### The connecting curve from H to J

Once J is determined, we need a smooth curve from 60800's cape route endpoint H to the junction point J on the spine. This is a Hermite interpolation:

```
Given: H, H' (position and tangent at cape route end)
       J, spine'(J) (position and tangent at junction)
Find:  cubic Bézier from H to J with these boundary conditions
```

The control points are:

```
P0 = H
P1 = H + (1/3) · |HJ| · normalize(H')
P2 = J - (1/3) · |HJ| · normalize(spine'(J))
P3 = J
```

The 1/3 factor is the standard Hermite-to-Bézier conversion. The result is a C1 curve that departs H in the direction 60800 was already traveling and arrives at J in the direction the spine is flowing. No kink at either end.

### Ribbon entry at J

After the connecting curve delivers 60800 to J, it needs to join the ribbon with offset d_60800 (the southernmost offset). At J, the ribbon has width — Group A stems are spread out by their offsets. 60800 enters at the southern edge.

The taper for 60800 differs from Group A. Group A stems have α(t) evaluated from t=0 (ribbon start) to t=1 (P). Source 60800 joins at t = t_J (the parameter where J sits on the spine). So 60800's effective taper is:

```
α_60800(t) = α(t)  for t ∈ [t_J, 1]
```

and before t_J, 60800 is on its independent connecting curve. At the junction, the connecting curve must deliver 60800 to position:

```
spine(t_J) + d_60800 · α(t_J) · N(t_J)
```

which is not exactly J (it's offset from J by d_60800 · α(t_J) in the normal direction). So the connecting curve's target isn't J itself — it's J shifted by the ribbon offset. This is important: if you aim the connecting curve at J exactly, 60800 will be at the spine centerline when it enters the ribbon, and it'll have to jump to its offset position, creating a discontinuity.

The corrected Hermite target is:

```
J_offset = J + d_60800 · α(t_J) · N(t_J)
```

and the arrival tangent should be:

```
spine'(t_J) + d_60800 · [α'(t_J) · N(t_J) + α(t_J) · N'(t_J)]
```

which is just S_60800'(t_J) — the derivative of 60800's ribbon path at the junction parameter. This ensures C1 continuity from the connecting curve into the ribbon.

### An important subtlety: the 70% cutoff

I mentioned constraining J to the first 70% of the spine. The reason: if J is too close to P, the ribbon section from J to P is short, and 60800's offset has very little distance to taper to zero. With the smoothstep taper, the offset at t_J = 0.9 is α(0.9) = 1 - 3(0.81) + 2(0.729) = 1 - 2.43 + 1.458 = 0.028 — nearly zero. That means 60800 would enter the ribbon already almost merged, defeating the purpose of the visualization (you want to *see* 60800 as a distinct stem).

The 70% cutoff ensures α(t_J) ≥ α(0.7) = 1 - 3(0.49) + 2(0.343) = 1 - 1.47 + 0.686 = 0.216. So 60800's offset at entry is at least 21.6% of its full offset — visible but already converging. Depending on the visual result, you might want to tighten this to 50% or 60%.

Alternatively, you could use a **modified taper for 60800** that's rescaled to start at t_J:

```
α_60800(t) = α((t - t_J) / (1 - t_J))   for t ∈ [t_J, 1]
```

This gives 60800 a full taper cycle from its junction point to P, so it enters at full offset and converges over the remaining spine length. The other Group A stems would use the standard α(t) over the full [0,1] range. The trade-off: Group A stems are more converged than 60800 at the junction region, creating a visual asymmetry. But this might actually look good — it shows 60800 arriving later and catching up.

This is the kind of thing where you'd want to see both options rendered before committing. Should I move toward implementation, starting with the spine computation and ribbon offset system for the Group A sources, then adding the junction logic for 60800?