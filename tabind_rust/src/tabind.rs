use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

pub fn perform_tabind() -> io::Result<Vec<String>> {
    //! # Description: 
    //! Given a word list of scrabble words, find all the words containing letters
    //! in the string "TABIND" that only appear once.
    //!
    //! In this process, the txt file will be read. Check every word in
    //! the txt file to see if it matches the constraints above.
    //!
    //! To do the constraint above, we will transform the letters inside of the word into a set
    //! because of the property that there is no duplicates.
    //! Ex. ABA = {"A", "B"}
    //!
    //! After that we will perform an intersection between the word_letters set and the tabind_letters set.
    //! Ex. ABA = {"A", "B"} ∩ {"T","A","B","I","N","D"} = {"A","B"}
    //!
    //! Once we do the intersection, we can compare the length of the the intersection 
    //! to the len of the word and see that they do not match.
    //! Ex. {"A","B"}.count() = 2, "ABA".len() = 3.
    //!
    //! # Notes:
    //! The ? operator is also used with io::Result<Vec<String>> for easier error handling.
    let file = File::open("../Collins_Scrabble_Words_2019.txt")?;
    let reader = BufReader::new(file);
    let mut legal_words = Vec::new();
    let tabind_letters = HashSet::from(['T','A','B','I','N','D']);

    for word in reader.lines() {
        let word = word?;
        let mut line_letters = HashSet::new();
        for letter in word.chars() {
            line_letters.insert(letter);
        }

        let intersection = tabind_letters.intersection(&line_letters);
        if intersection.count() == word.len() {
            legal_words.push(word.to_string());
        }
    }
    return Ok(legal_words)
}