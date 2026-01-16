fn main() {
    let x = 5;
    // x and y stored entirely on stack, same behavior
    // for deep and shalow
    let y = x;
    println!("x: {x} and y: {y}");

    let s1 = String::from("hello");
    // add clone method to avoid invalidation and move of s1
    let s2 = s1.clone();
    println!("s1: {s1} and s2: {s2}");

    let s3 = String::from("hello");
    let len_s3 = calculate_length(&s3);
    println!("The length of {s1} is {len_s3}");

    let mut s4 = String::from("hello");
    change_ref(&mut s4);
    println!("{s4}");

    let mut s5 = String::from("hello");
    let r1 = &s5;
    println!("{r1}");
    let r2 = &mut s5;
    r2.push_str(", world");
    // code below will trigger compilation error
    // cannot borrow `s5` as mutable because it is also borrowed as immutable
    // println!("{r1}");

    let mut s6 = String::from("hello world");
    let word = first_word(&s6);
    // code below will trigger compilation error
    // cannot borrow `s6` as mutable because it is also borrowed as immutable
    // s6.clear();
    println!("the first word is: {word}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_ref(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}