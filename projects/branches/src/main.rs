use rand::Rng;

fn main() {
    let condition = get_rand_int();
    //trying single line control flow
    let number = if condition < 30 {1} else if condition > 30 && condition < 60 {2} else {condition};

    if number == 1 {
        println!("Hello, world!");
    } else if number == 2 {
        println!("Hello, universe!");
    } else {
        println!("Hello, number {number}!")
    }
}

fn get_rand_int() -> u8 {
    rand::thread_rng().gen_range(1..=100)
}
