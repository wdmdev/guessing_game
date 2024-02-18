#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipv4 = IpAddr::V4(0,0,0,0);
    let ipv6 = IpAddr::V6(String::from("2001:db8::8a2e:370:7334"));

    println!("Hello, world!");
    println!("My IP V4: {:?}", ipv4);
    println!("My IP V6: {:?}", ipv6);

    //Using the Option enum for Some or None valued variables
    let some_number = Some(5); //Rust infers the i32 type because of value
    let some_char = Some('a'); //Ruset infers the char type
    let absent_number: Option<i32> = None; //We have to declare the type because the value is None

}
