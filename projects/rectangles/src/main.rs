
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// implement block to define struct functions
impl Rectangle {
    fn area (&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, rect:&Rectangle) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }

    fn square(size:f64) -> Self {
        Self { width: size, height: size }
    }
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

    let rect3 = Rectangle {
        width: 5.,
        ..rect2
    };

    println!("{:?}\nHas area: {}", rect3, rect3.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let square1 = Rectangle::square(1.5);

    println!("{:?}\nHas area: {}", square1, square1.area());
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
}

fn area(rectangle: &Rectangle) -> f64 {
    rectangle.height * rectangle.width
}
