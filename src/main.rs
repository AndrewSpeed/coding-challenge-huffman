use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filepath: String,
}

fn read_file(filepath: &str) -> Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn calculate_character_frequency(input: &str) -> Result<HashMap<char, usize>> {
    let frequency_map = input.chars().fold(HashMap::new(), |mut map, char| {
        if map.contains_key(&char) {
            map.insert(char, map.get(&char).expect("Key not in hashmap") + 1);
        } else {
            map.insert(char, 1);
        }

        map
    });
    Ok(frequency_map)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = read_file(&cli.filepath)?;
    let char_frequency = calculate_character_frequency(&content)?;

    println!(
        "t frequency: {frequency}",
        frequency = char_frequency.get(&'t').expect("t not present in map")
    );
    println!(
        "X frequency: {frequency}",
        frequency = char_frequency.get(&'X').expect("X not present in map")
    );

    Ok(())
}
