use csv::Error;
use rand::Rng; 
use rayon::prelude::*;
use std::time::{Duration, Instant};
use indicatif::ParallelProgressIterator;
use dashmap::DashMap;
use serde::Deserialize;

use h3o::{CellIndex, LatLng, Resolution};


#[derive(Debug, Deserialize)]
struct Record {
    id: i64,
    cell: u64,
}

fn get_duration(start: Instant, number_of_cells: u64) {
    let duration: Duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    println!("s/iter: {:?}", duration/number_of_cells as u32);
}

fn generate_random_locations(number_of_random_locations: u64) -> Vec<LatLng> {
    let mut rng = rand::thread_rng();

    let locations: Vec<LatLng> = (0..number_of_random_locations).map(|_| LatLng::new(rng.gen_range(-90.0..90.0), rng.gen_range(-180.0..180.0)).expect("bla")).collect();

    return locations
}

fn init_lookup_map(map: &DashMap<CellIndex, i64>) -> Result<(), Error>{
    let mut reader = csv::Reader::from_path("ddpi_cells.csv")?;

    for record in  reader.deserialize() {
        let record: Record = record?;
        let cell = CellIndex::try_from(record.cell).expect("fehler");
        map.insert(cell, record.id);
    };

    Ok(())
}

fn main() {
    const NUMBER_OF_CELLS : u64= 100_000_000;
    const H3_RESOLUTION : Resolution = Resolution::Ten;

    let map: DashMap<CellIndex, i64> = DashMap::new();

    println!("HashMap initialization");
    init_lookup_map(&map).expect("could not init hashmap");
 
    println!("{:?}", DashMap::capacity(&map));

    println!("creating random locations ...");
    let locations = generate_random_locations(NUMBER_OF_CELLS);

    println!("location to h3 ...");
    let start = Instant::now();
    let ports: Vec<_>  = locations.into_par_iter().progress_count(NUMBER_OF_CELLS).map(|location| {
        let cell = location.to_cell(H3_RESOLUTION);
        return map.get(&cell);
    })
    .collect();

    get_duration(start, NUMBER_OF_CELLS);

    // for port in ports {
    //     match port {
    //         Some(port) => println!("{:?}", port.value()),
    //         None => ()
    //      }
    // }
}

