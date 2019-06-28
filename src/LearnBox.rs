// https://kaisery.github.io/trpl-zh-cn/ch15-01-box.html
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn demoBox() {}

pub fn store_on_help() {
    let b = Box::new(5);
    println!("b = {}", b);
}

pub fn recursive_type() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}