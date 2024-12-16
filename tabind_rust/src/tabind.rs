use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

pub fn perform_tabind() -> io::Result<Vec<String>> {

    let file = File::open("../Collins_Scrabble_Words_2019.txt")?;
    let reader = BufReader::new(file);
    let mut legal_words = Vec::new();
    let tabind_letters = HashSet::from(['T','A','B','I','N','D']);

    for line in reader.lines() {
        let line = line?;
        let mut line_letters = HashSet::new();
        for letter in line.chars() {
            line_letters.insert(letter);
        }

        let intersection = tabind_letters.intersection(&line_letters);
        if intersection.count() == line.len() {
            legal_words.push(line.to_string());
        }
    }
    Ok(legal_words)
}