use minecraft_nether_gen_rs::{NetherBiomes, NetherGen};
use std::{any::Any, collections::HashSet, env::{self, args}, fmt::{DebugTuple, Display}, iter::Enumerate, ops::{Index, Neg}, process::exit};
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::io;

fn main() {

    let mut default_dist = "50".to_string();
    let mut default_interval = "10".to_string();
    let mut default_max_tries = "1000".to_string();

    let args: Vec<String> = env::args().collect();

    let distance: i32 = match args
        .get(1)
        .or(Some(&default_dist))
        .unwrap()
        .parse() {
            Ok(x) => x,
            Err(e) => {
                println!("failed to parse distance");
                std::process::exit(1);

            }
        };

    let interval: usize = match args
        .get(2)
        .or(Some(&default_interval))
        .unwrap()
        .parse() {
            Ok(x) => x,
            Err(e) => {
                println!("failed to parse interval");
                std::process::exit(1);
            }
        };

    let max_tries: usize = match args
        .get(3)
        .or(Some(&default_max_tries))
        .unwrap()
        .parse() {
            Ok(x) => x,
            Err(e) => {
                println!("failed to parse max tries");
                std::process::exit(1);
            }
        };

    let mut found_seed = false;
    let mut attempts = 0;

    while found_seed == false {

        let mut rng = thread_rng();
        let seed: u32 = rng.gen();

        let mut nether = NetherGen::new(seed.into());

        let mut found_biomes = HashSet::new();
        let mut last_check = (0,0);

        // actual checking algorithnm

        for i in (distance.neg()..distance + 1).step_by(interval) {

            for n in (distance.neg()..distance + 1).step_by(interval) {

                let biome = nether.get_final_biome(i, 0, n).to_string();
                last_check = (i,n);

                println!("{} at {} {}", biome, i,n);

                if found_biomes.contains(&biome) {
                    // do nothing
                } else {
                    found_biomes.insert(biome);
                    //println!("{:?}", found_biomes);
                }

                attempts += 1;

                if found_biomes.len() == 5 {

                    println!("Found seed with all nether biomes within {} blocks", distance);
                    println!("Seed: {}", seed);
                    found_seed = true;
                    exit(0);

                } else {

                    println!("{} seeds searched", attempts)

                }

                if attempts >= max_tries {

                    println!("Failed to find seed in {} tries", attempts);
                    exit(0);
        
                }

            }

        }



    }

}