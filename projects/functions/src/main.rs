fn main() {
    println!("Hello, world!");

    another_function(-5, "test", 'u');
}

fn another_function(number: i32, text: &str, c: char) {
    println!("This is a number: {number} \n and this is a text: {text} \n and this is a char: {c}");
}
