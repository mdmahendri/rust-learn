use ch10::{NewsArticle, SocialPost, Summary};

fn main() {
    let numb_list = vec![34, 50, 25, 100, 65];
    // largest_in_vec(&numb_list);
    let result = largest(&numb_list);
    println!("the largest number is {result}");

    let numb_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // largest_in_vec(&numb_list);
    let result = largest(&numb_list);
    println!("the largest number is {result}");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2 = Point { x: 5.1, y: 8.7 };
    println!("distance from origin: {}", p2.distance_from_origin());

    let p3a = Point2 { x: 5, y: 10.4 };
    let p3b = Point2 { x: "Hello", y: 'c' };
    let p3c = p3a.mixup(p3b);
    println!("p3.x = {}, p3.y = {}", p3c.x, p3c.y);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("penguins wins"),
        location: String::from("Pittsburh, PA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "the pittsburgh penguin \
        once again...",
        ),
    };

    println!("1 news article: {}", article.summarize());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    print!("the longest string is {result}");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
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

// fn largest2<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// example is Option<T>
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// example is Result<T, E>
struct Point2<T, U> {
    x: T,
    y: U,
}
