fn main() {
    let numb_list = vec![34, 50, 25, 100, 65];
    // largest_in_vec(&numb_list);
    let result = largest(&numb_list);
    println!("the largest number is {result}");

    let numb_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // largest_in_vec(&numb_list);
    let result = largest(&numb_list);
    println!("the largest number is {result}");
}

// trying out before looking 10.3
fn largest_in_vec(numb_vec: &Vec<i32>) {
    let mut largest = numb_vec[0];

    for numb in numb_vec {
        if *numb > largest {
            largest = *numb;
        }
    }

    println!("the largest number is {largest}")
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    // Rust is smart here. It sees you are comparing
    // two references (&i32 vs &i32) and automatically
    // "looks through" them to compare the numbers.
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
