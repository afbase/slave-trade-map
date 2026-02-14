your goal is to make a dioxus d3.js based tool that helps one visualize the transatlantic slave trade database - see SUMMARY.md and SPSS_Codebook_2023-11-06.pdf

How you will do so is:
- refactor main.rs
- load your preferred tastdb-*.csv file with `include_str!` macro and load it into the database similarly to how `src/main.rs` does it currently.  Please redo the database table to fit the entire format of the tastdb-*.csv that you choose.  Ideally a second table would be good as well to use I-Am1.0.csv too.
- dioxus_use_js is the tool you'll want to use. Use this to integrate D3.js for visualization. see `https://docs.rs/dioxus-use-js/latest/dioxus_use_js/index.html` feel free to make a second crate for this if you so desire.
- example.html gives a basic example.  I don't care for the css as I want this to look like a research tool instead of whatever the design language is presented here.
- instead of the functionality that example.html gives, I want functionality that does the following:
    - I have the option to select ports or regions on the map
        - if i select ports, i can click on a port.  when I do so, the map has a collection of stems from the origins of africa, each with a thickness representing the volume of slaves that were taken to that port relatively from the total number of slaves cumulatively.  the stems of these arrows connect to a trunk.  the trunk extends to a pointed arrow pointing to the port.  Note that 09.jpg does something like this but insead has many pointed arrows from a trunk to all origins and destinations where I am describing one destination from all origins.
        - Similar functionality occurs if i select regions
    - at the bottom of the map I have a sliding window of time.  I can select the window of time I want to research where the trade occured to that destination port/region.  Example might be I want to look at all the slave trade between 1700-1750 to new orleans.  or all the slave trade to cuba from 1600-1827.
    - when i select a time window, the map and arrow renders only the data for that time period for that destination and origins


You must use dioxus.  you must use d3.js. this is strictly a wasm32-unknown-unknown blob that is generated. So no Server side code. 