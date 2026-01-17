fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(&rect1)
    );
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(16);
    println!("rect4 is {rect4:#?}");
}

// fn area(width: u32, height: u32) -> u32 {
// fn area(dimensions: (u32, u32)) -> u32 {
fn area(rect: &Rectangle) -> u32 {
    // width * height
    // dimensions.0 * dimensions.1
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width >= rect2.width && self.height >= rect2.height
    }
}
