pub fn mutString() {
    let mut s = String::from("hello");
    s.push('b');
    println!("string now is {}", s);
}

pub fn run() {
    mutString();
    borrowAfterMove();
    functionMethodPassValue();
}

fn borrowAfterMove() {
    let s1 = String::from("hello");
    let s2 = s1; // now s1 pointer is invalid

    // TODO Learn Borrow after move error
    // println!("{}, world!", s1);
}

fn borrowAfterClone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn primativeVariable() { // primative number is on stack so ok
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn functionMethodPassValue() {
    let s = String::from("hello");  // s 进入作用域
    println!("print the returned value {}", append(s));
}

fn append(item:String) -> String {
    return item;
}

