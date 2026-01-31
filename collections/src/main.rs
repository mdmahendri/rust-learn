use std::collections::HashMap;

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

    let string1 = "initial contents".to_string();
    let string2 = String::from("initial contents");
    let mut string3 = String::from("str");
    string3.push_str("bar");
    string3.push('l');
    println!("value of string3: {string3}");

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let string4 = s4 + &s5;
    // s4 now moved into string4, you cant do below
    // println!("value ofs4: {s4}");

    // this method does not move value
    let s4a = String::from("tic");
    let string5 = format!("{s4a}-{s5}");

    let hello = "Здравствуйте";
    // this result in error, must declare explicitly
    // let id1s = hello[0];
    let id2s = hello.chars().nth(1);
    if let Some(cha) = id2s {
        println!("value of id2s: {cha}");
    }
    let id3s = hello.bytes().nth(1);
    if let Some(numb) = id3s {
        println!("value of id3s: {numb}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);
    let yscore = scores.get("Yellow");
    if let Some(numb) = yscore {
        println!("yscore is {numb}");
    }

    scores.entry(String::from("Blue")).or_insert(3);
    scores.entry(String::from("Red")).or_insert(33);
    println!("{scores:?}");

    // exercise 1
    let int_list = [1, 2, 3, 9, 4, 5, 3, 7, 5, 1];
    let mut vec_list = Vec::from(int_list);
    vec_list.sort();
    let len_list = vec_list.len();
    let median: f64 = if len_list % 2 == 0 {
        let med1 = vec_list[len_list / 2 - 1];
        let med2 = vec_list[len_list / 2];
        (med1 + med2) as f64 / 2.0
    } else {
        vec_list[len_list / 2] as f64
    };
    println!("vec: {vec_list:?}");
    println!("len: {len_list}; median: {median}");
    let mut mode_hash = HashMap::new();

    for numb in &vec_list {
        let count = mode_hash.entry(*numb).or_insert(0);
        *count += 1;
    }
    let mut max_val = 0;
    let mut mode = 0;
    for (k, v) in &mode_hash {
        if *v > max_val {
            max_val = *v;
            mode = *k;
        }
    }
    println!("{mode_hash:?} with mode {mode}");

    // exercise 2
    let ex2a = pig_latin("first".to_string()).unwrap();
    let ex2b = pig_latin("apple".to_string()).unwrap();
    println!("res1: {ex2a} res2: {ex2b}");

    // exercise 3
}

enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn pig_latin(some_str: String) -> Option<String> {
    //original iteration
    // let first_char = some_str.chars().nth(0);
    // if let Some(fchar) = first_char {
    //     return if "aiueo".contains(fchar) {
    //         Some(format!("{some_str}-hay"))
    //     } else {
    //         let new_word = some_str[1..].to_string();
    //         Some(format!("{new_word}-{fchar}ay"))
    //     };
    // } else {
    //     None
    // }
    let mut chars = some_str.chars();
    if let Some(fchar) = chars.next() {
        let remainder = chars.as_str();
        if "aiueoAIUEO".contains(fchar) {
            Some(format!("{some_str}-hay"))
        } else {
            Some(format!("{remainder}-{fchar}ay"))
        }
    } else {
        None
    }
}
