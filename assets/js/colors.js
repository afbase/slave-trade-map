// African source region colors (warm earth tones — transatlantic)
var africanColors = {
  60100: "#c8a050",
  60200: "#b89040",
  60300: "#a88838",
  60400: "#d4a856",
  60500: "#c09848",
  60600: "#b08838",
  60700: "#9a7830",
  60800: "#8a6828",
  60900: "#7a5820",
};

// American source region colors (cool blue-teal — inter-American)
var americanColors = {
  // North America (20xxx)
  20100: "#4a7a9a", 20300: "#4878a0", 20400: "#4670a0", 20500: "#4468a0",
  20600: "#4a7090", 20700: "#486888", 20800: "#466080", 20900: "#445878",
  21000: "#425070", 21100: "#405868", 21200: "#3e5060", 21300: "#3c4858",
  21400: "#3a4050", 21500: "#384860", 21600: "#365070", 21700: "#345878",
  21800: "#326080", 21900: "#305878", 22000: "#305878", 22100: "#305878",
  // Spanish Caribbean (31xxx)
  31100: "#2a8a7a", 31200: "#288678", 31300: "#268270", 31400: "#247e68",
  // Dutch (32xxx)
  32100: "#3a9a8a", 32200: "#409890",
  // British Caribbean (33-35xxx)
  33200: "#3a8878", 33300: "#388474", 33400: "#368070", 33500: "#347c6c",
  33600: "#327868", 33700: "#307464", 33800: "#2e7060",
  34100: "#2c6c5c", 34200: "#2a6858", 34300: "#286454", 34400: "#266050",
  34500: "#245c4c", 34600: "#225848",
  35100: "#3a8070", 35200: "#387c6c", 35300: "#367868", 35400: "#347464",
  35500: "#327060",
  // French Caribbean (36xxx)
  36100: "#4878a0", 36200: "#4674a0", 36300: "#4470a0", 36400: "#3c6898",
  36500: "#3a6498",
  // Danish (37xxx)
  37000: "#506a90",
  // Other Caribbean
  38100: "#4e6688", 39000: "#4c6280",
  // Spanish mainland (41-44xxx)
  41200: "#2a7868", 42000: "#287060", 43000: "#266858", 44000: "#246050",
  // Brazil (50xxx)
  50100: "#2a8070", 50200: "#288078", 50300: "#207060", 50400: "#186858",
  50500: "#106050",
  // Unspecified
  80200: "#607888", 80400: "#586878", 80700: "#506070",
  // European (as I-Am sources)
  10100: "#5070a0", 10200: "#4e6c9a", 10300: "#4c6894", 10400: "#4a648e",
  10500: "#486088", 10700: "#465c82", 10800: "#44587c",
};

// Get source color based on code and dataset
function getSourceColor(code, dataset) {
  if (dataset === "transatlantic") {
    if (africanColors[code]) return africanColors[code];
    var parent = Math.floor(code / 100) * 100;
    if (africanColors[parent]) return africanColors[parent];
    return "#8a7848"; // default warm
  } else {
    if (americanColors[code]) return americanColors[code];
    var parent = Math.floor(code / 100) * 100;
    if (americanColors[parent]) return americanColors[parent];
    return "#4a7090"; // default cool
  }
}
