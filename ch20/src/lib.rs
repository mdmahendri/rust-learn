use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Ypoint {
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Ypoint {
    pub fn new(y: i32) -> Ypoint {
        Ypoint { y }
    }
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Ypoint> for Point {
    type Output = Point;

    fn add(self, other: Ypoint) -> Point {
        Point {
            x: self.x,
            y: self.y + other.y,
        }
    }
}

pub trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

pub struct Human;

impl Human {
    pub fn new() -> Human {
        Human
    }

    pub fn fly(&self) {
        println!("Human need flying course!");
    }

    pub fn name() -> String {
        String::from("Human")
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot is flying the plane!");
    }
    fn name() -> String {
        String::from("Pilot")
    }
}