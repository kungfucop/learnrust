use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;
use std::string::String;
use std::string::ToString;

pub trait Printer {
    fn print(&self);
}

pub trait Summary {
    fn summarize(&self) -> String { // default implementation
        String::from("(Read more...)")
    }
}

impl Printer for i32 {
    fn print(&self) {
        println!("this number is {}", self)
    }
}

struct Student {
    pub age: i32,
    pub name: String,
}

impl Printer for Student {
    fn print(&self) {
        println!("age {} name {}", self.age, self.name)
    }
}


pub fn testPrinter() {
    5.print();
    let student = Student {
        age: 1,
        name: String::from("hongjie"),
    };
    student.print()
}

impl Summary for Student {}

pub fn test_default_trait_imp() {
    let student = Student {
        age: 1,
        name: String::from("hongjie"),
    };
    println!(" sumary is {}", student.summarize())
}

pub fn testTraitReference() {
    let student = Student {
        age: 1,
        name: String::from("hongjie"),
    };
    traitReferenceSummary((student))
}


fn traitReferenceSummary(obj: impl Summary) {
    println!(" sumary is {}", obj.summarize());
    return;
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: impl Summary, item2: impl Summary) {}

pub fn notify3<T: Summary + Display>(item: T) {}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    return 1;
}

fn returns_summarizable() -> impl Summary {
    let student = Student {
        age: 1,
        name: String::from("hongjie"),
    };
    return student;
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn foo() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn fooSame() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

//impl<T: Display> ToString for T {
//    fn to_string(&self) -> String
//    {
//        return String::from("check")
//    }
//    // --snip--
//}

struct LifeTimeTrick<'a> {
    pub name: &'a String
}

fn testLifeCycleTrick() {
    let name = String::from("hello you");
    let lft = LifeTimeTrick { name:&name };
}