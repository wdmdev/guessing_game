fn main() {
    let s = String::from("Hello world!");

    let _first_word_idx = first_word(&s); //we return an index
    //_first_word_idx can easily become invalid, e.g. from s.clear()

    let fw = first_word_slice(&s); 
    //fw__idx will give compile error if e.g. s.clear() is used

    println!("The first word is: {}", fw);
}

fn first_word(s:&String) -> usize {
    //first word without using slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; //we only return index
        }
    }

    return s.len();
}

fn first_word_slice(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; //returning string slice, reference and length
        }
    }

    &s[..]
}