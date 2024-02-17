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
}
