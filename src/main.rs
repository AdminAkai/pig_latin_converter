fn main() {
    let string = String::from("Hello, world! I like apples.");
    println!("Hello, world! I like apples.");
    println!("Converted to pig latin: {}", convert_to_pig_latin(string));
}

fn convert_to_pig_latin(s: String) -> String {
    let mut pig_latin: String = String::new();

    for c in s.split_whitespace() {
        let (formatted, punctuation) = check_punctuation(c);
        if c.to_lowercase().starts_with("a") ||
            c.to_lowercase().starts_with("e") ||
            c.to_lowercase().starts_with("i") ||
            c.to_lowercase().starts_with("o") ||
            c.to_lowercase().starts_with("u") {
                pig_latin = pig_latin + formatted + "-hay" + punctuation + " ";
        } else {
            pig_latin = pig_latin + &formatted[1..] + "-" + &c[0..1] + "ay" + punctuation + " "; 
        }
        
    }

    String::from(pig_latin.trim_end())
}

fn check_punctuation(s: &str) -> (&str, &str) {
    if s.ends_with(".") || s.ends_with("!") || s.ends_with("?") || s.ends_with(";") || s.ends_with(",") || s.ends_with(":") {
        return (&s[0..s.len() - 1], (&s[s.len() - 1..]))
    }

    (s, "")
}