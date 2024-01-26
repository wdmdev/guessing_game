use std::io;

//Constant array of months
const MONTHS : [&str; 12] = ["January", "February", "March", 
                            "April", "May", "June", "July",
                            "August", "September", "October",
                            "November", "December"];

fn main() {
    loop {
        println!("Please enter a month index. (1 to 12)");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        //Shadowing
        let index: usize = match index.trim().parse::<usize>() {
            Ok(num) => {
                if num >= 1 && num <= 12 {
                    num - 1
                } else {
                    continue;
                }
            },
            Err(_) => continue,
        };
        
        println!("The month is {}", MONTHS[index]);
        break;
    }
}