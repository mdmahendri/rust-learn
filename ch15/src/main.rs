use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {name}");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: over quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning, it's over 90");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Not urgent, it's over 75");
        }
    }
}

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};
use crate::List3::{Cons3, Nil3};

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    if let Cons(value, next) = list {
        println!("the value of first is {value}");
        println!("{:?}", next);
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);


    let z = MyBox::new(String::from("Rusticrab"));
    hello(&z);

    let _a = CustomSmartPointer {
        data: String::from("CSP a"),
    };

    drop(_a);
    println!("end.");


    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    let _c = Cons2(4, Rc::clone(&a));
    println!("count after adding b _c = {}", Rc::strong_count(&a));

    if let Cons2(value, next) = b {
        println!("the value of first is {value}");
        println!("{:?}", next);
    }

    let mut f = 5;
    let g = &mut f;
    *g += 1;
    println!("g = {g}");

    let value3 = Rc::new(RefCell::new(5));
    let a3 = Rc::new(Cons3(Rc::clone(&value3), Rc::new(Nil3)));
    let b3 = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a3));

    *value3.borrow_mut() += 10;
    println!("a3 after = {:?}", a3);
    println!("b3 after = {:?}", b3);

    if let Cons3(value, next) = &*a3 {
        println!("the value of first is {}", value.borrow());
        println!("{:?}", next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_over_75_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}