use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

mod huffman_tree;

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

fn calculate_character_frequency(input: &str) -> HashMap<char, usize> {
    input.chars().fold(HashMap::new(), |mut map, char| {
        if map.contains_key(&char) {
            map.insert(char, map.get(&char).expect("Key not in hashmap") + 1);
        } else {
            map.insert(char, 1);
        }

        map
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = read_file(&cli.filepath)?;
    let _char_frequency = calculate_character_frequency(&content);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_character_frequency() {
        let char_freq = calculate_character_frequency("This is some input for character frequency");
        assert_eq!(
            char_freq,
            HashMap::from([
                ('T', 1),
                ('h', 2),
                ('i', 3),
                ('s', 3),
                (' ', 6),
                ('o', 2),
                ('m', 1),
                ('e', 4),
                ('n', 2),
                ('p', 1),
                ('u', 2),
                ('t', 2),
                ('f', 2),
                ('r', 4),
                ('c', 3),
                ('a', 2),
                ('q', 1),
                ('y', 1)
            ])
        );
    }

    #[test]
    fn test_calculate_character_frequency_with_empty_input() {
        let char_freq = calculate_character_frequency("");
        assert_eq!(char_freq, HashMap::new());
    }
}
