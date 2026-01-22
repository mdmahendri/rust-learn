fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    v.push(5);
    v.push(-2);
    println!("v value is {:?}", v);

    let mut v3 = vec![5, 3, 6];
    // will make program crash and panic
    // let v3_oob1 = &v3[90];
    // will output option, allowing check
    let v3_oob3 = v3.get(100);

    let v3_test1 = v3.get(1);
    match v3_test1 {
        Some(&element) => println!("number is {element}"),
        None => println!("okay"),
    };
    let v3_test2 = &v3[1];
    v3.push(9);
    // will trigger borrow error
    // println!("number is {v3_test2}");

    for elm in &mut v3 {
        *elm += 50;
        println!("number is {elm}");
    }

    let row = vec![
        Cell::Int(1),
        Cell::Float(6.3),
        Cell::Text(String::from("sdf")),
    ];
}

enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}
