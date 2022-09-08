fn main() {
    let text = String::from("Introducing an all new iPhone lineup rebuilt AirPods Pro three new Apple Watch models and an exciting update to Apple Fitness");
    let mut v = Vec::new();
    for word in text.split_whitespace() {
        let c = process_string(word.to_string());
        v.push(c);
    }
    println!("{:?}", v);
    println!("{} -> {}", String::from("first"), process_string(String::from("first")));
    println!("{} -> {}", String::from("apple"), process_string(String::from("apple")));
}

fn process_string(word: String) -> String {
    let chs: Vec<char> = word.chars().collect();
    for i in 0..chs.len() {
        let c = chs[i];
        if i ==0 && is_vowel(c) {
            return format!("{}-hay", word);
        }
        if !is_vowel(c) {
            return format!("{}{}-{}ay", &word[..i], &word[i+1..chs.len()], &word[i..i+1]);
        }
    }
    return word
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _other => false,
    }
}