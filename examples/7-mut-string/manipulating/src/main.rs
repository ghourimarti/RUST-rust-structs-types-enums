fn main() {

    println!("\n<=====================================>\n");

    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    println!("<=====================================>");
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    println!("<================== for ===================>");
    // iterate over the characters in the sentence
    println!("{}", &sentence);
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel! {}", c),
            ' ' => println!("Got a space" ),
            _ => println!("Got a consonant! {}", c),
            _ => continue,
        }
    }

    // Split and collect into a vector
    println!("<===============================================>");
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);
    println!("<===============================================>");
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    println!("<===============================================>");
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
