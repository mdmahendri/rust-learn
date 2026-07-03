use std::thread;
use std::time::Duration;

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: {x}, {y}");
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("{val} is at index {idx}");
    }

    let point = (3, 5);
    print_coordinates(&point);

    let x = None;
    match x {
        Some(20) => println!("got twenty"),
        Some(y) => println!("matched y= {y}"),
        _ => println!("got default {x:?}"),
    }

    let y = 'c';
    match y {
        'a'..='j' => println!("early alphabet letter"),
        'k'..='z' => println!("late alphabet letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7};
    let Point { x: v, y: w } = p;
    println!("point destructuring: {v} {w}");
}
