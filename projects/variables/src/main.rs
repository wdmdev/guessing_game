use std::{cmp::Ordering, io};

//Constant array of months
const MONTHS : [&str; 12] = ["January", "February", "March", 
                            "April", "May", "June", "July",
                            "August", "September", "October",
                            "November", "December"];
const MIN_MONTH : usize = 0;
const NUM_MONTHS : usize = 11;

fn main() {
    loop {
        println!("Please enter a month index. (1 to 12)");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        //Shadowing
        let index: usize = match index.trim().parse() {
            Ok(num) => num - 1, //make 0 indexed
            Err(_) => continue,
        };
        let month_name = MONTHS[index];

        match index.cmp(&MIN_MONTH) {
            Ordering::Less => continue,
            Ordering::Equal => (),
            Ordering::Greater => (),
        }

        match index.cmp(&NUM_MONTHS) {
            Ordering::Greater => continue,
            Ordering::Equal => (),
            Ordering::Less => (),
        }

        println!("The name of month {index} is: {month_name}.");

    }
}