
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let rect1 = Rectangle{
        width: 7.5,
        height: 2.,
    };

    println!("{:?} \nHas area: {}", rect1, area(&rect1));

    let rect2 = Rectangle {
        width: 5.5,
        height: 5.5,
    };

    dbg!(&rect2); //prints to standard error console stream
    println!("Has area: {}", area(&rect2));
}

fn area(rectangle: &Rectangle) -> f64 {
    rectangle.height * rectangle.width
}
