mod tabind;

fn main() {
    let result = tabind::perform_tabind();

    match result {
        Ok(legal_words) => {
            println!("Legal Words: {}", legal_words.join(","));
            println!("Word Count: {}", legal_words.len());
        }
        Err(e) => eprintln!("Error opening file: {:?}", e)
    }
}
