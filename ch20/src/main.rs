use ch20::{Point, Ypoint, Human, Pilot};

fn main() {
    let p1 = Point::new(5, 10);
    let p2 = Point::new(2, 3);
    println!("{:?}", p1 + p2);

    
    let p3 = Point::new(2, 3);
    let y1 = Ypoint::new(5);
    println!("{:?}", p3 + y1);

    let hum = Human::new();
    hum.fly();
    Pilot::fly(&hum);

    println!("Name: {}", Human::name());
    println!("Name: {}", <Human as Pilot>::name());
}
