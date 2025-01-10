use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string(path)
    // let mut file_content =
    //     File::open("sample.txt").expect("sample.txt should be in project folder");
    let mut file_content = File::open("sample.txt")?;
    let mut content = String::new();
    // let bytes_read = file_content.read_to_string(&mut content).unwrap();
    match file_content.read_to_string(&mut content) {
        Ok(bytes_read) => println!("Bytes read = {}", bytes_read),
        Err(e) => panic!("Cannot read the file: {:?}", e),
    };
    let last_char = last_char_of_first_line(&content);
    // println!("{content}");
    println!("{:?}", last_char.unwrap());
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in content.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted: Vec<_> = word_count.into_iter().collect();
    println!("----------------SORTED WORD COUNTER ---------------");
    sort_hash_map(&mut sorted);
    println!("{:?}", sorted);
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn sort_hash_map(sorted: &mut Vec<(&str, i32)>) {
    sorted.sort_by_key(|a| -a.1);
}
