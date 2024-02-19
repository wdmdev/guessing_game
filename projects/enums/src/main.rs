enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Device {
    Laptop(IpAddr),
    Mobile(IpAddr),
    SmartWatch,
    VRHeadset
}

fn main() {
    //Using the Option enum for Some or None valued variables
    let some_number = Some(5); //Rust infers the i32 type because of value
    let some_char = Some('a'); //Ruset infers the char type
    let absent_number: Option<i32> = None; //We have to declare the type because the value is None

    match some_number { //We have to do match on the option to have logic both for values and None
        None => println!("I have no number!"),
        Some(i) => println!("My number is: {}", i),
    }

    match some_char {
        None => println!("I have no char!"),
        Some(c) => println!("My char is: {}", c),
    }

    match absent_number {
        None => println!{"I have no number!"},
        _ => println!("My number is not absent anyways!"),
    }


    println!("Let's look at some devices!");

    let d1 = Device::Laptop(IpAddr::V6(String::from("FE80:CD00:0000:0CDE:1257:0000:211E:729C")));
    let d2 = Device::Mobile(IpAddr::V4(0,0,0,0));
    let d3 = Device::SmartWatch;
    let d4 = Device::VRHeadset;

    check_device(d1);
    check_device(d2);
    check_device(d3);
    check_device(d4);

}

fn check_device(device: Device) {
    match device {
        Device::Laptop(ip) => {
            println!("On the workstation!");
            match ip {
                IpAddr::V4(i1, i2, i3, i4) => println!("Your ip is oooold: {},{},{},{}", i1, i2, i3, i4),
                IpAddr::V6(id) => println!("You have a new and long ip: {}", id),
            }
        },
        Device::Mobile(ip) => {
            println!("You're going mobile!");
            match ip {
                IpAddr::V4(i1, i2, i3, i4) => println!("Your ip is oooold: {}.{}.{}.{}", i1, i2, i3, i4),
                IpAddr::V6(id) => println!("You have a new and long ip: {}", id),
            }
        }
        Device::SmartWatch => println!("Keep that pulse going!"),
        Device::VRHeadset => println!("Welcome to the matrix!"),
    }
}
