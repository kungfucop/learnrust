pub fn mutString() {
    let mut s = String::from("hello");
    s.push('b');
    println!("string now is {}", s);
}

pub fn run() {
    mutString();
    let str = implicitLifecycle("test");
    println!("test string is {:?}", str);
    borrowAfterMove();
    borrowWithoutMove();
    functionMethodPassValue();
    borrowMutablity();
}

fn borrowAfterMove() {
    let s1 = String::from("hello");
    let s2 = s1; // now s1 pointer is invalid

    // TODO Learn Borrow after move error
    // println!("{}, world!", s1);
}

fn borrowWithoutMove() {
    let s1 = String::from("Test");
    let s2 = &s1;
    println!("x is{:?},y is {:?}", s1, s2);

    let mut d1 = 1;
    {
        let d3: &mut i32 = &mut d1;
        let d2: &mut i32 = &mut d1;
        *d2 = 5;
    }
    let d3: &mut i32 = &mut d1;

    println!("d1 is now {}", d1);
}

fn borrowMutablity() {
    let mut x: Vec<i32> = vec!(1i32, 2, 3);
    let mut x2 = 1;

    //只能有一个可变借用
    let y = &mut x;
    let y2 = &mut x2;
    let y3 = &mut x2;// this mut is ok
//    let z = &mut x; //错误
    y.push(100);

    //ok
    println!("{:?}", y);

    //错误，可变借用未释放，源变量不可访问
    // println!("{:?}", x);
}

fn complexBorrow() {
    let mut x: Vec<i32> = vec!(1i32, 2, 3);

    //更新数组
    //push中对数组进行了可变借用，并在push函数退出时销毁这个借用
    x.push(10);

    {
        //可变借用1
        let mut y = &mut x;
        y.push(100);

        //可变借用2，注意：此处是对y的借用，不可再对x进行借用，
        //因为y在此时依然存活。
        let z = &mut y;
        z.push(1000);

        println!("{:?}", z); //打印: [1, 2, 3, 10, 100, 1000]
    } //y和z在此处被销毁，并释放借用。


    //访问x正常
    println!("{:?}", x); //打印: [1, 2, 3, 10, 100, 1000]
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

fn append(item: String) -> String {
    return item;
}

fn implicitLifecycle(x: &str) -> &str {
    return &"hello";
    // if *x = "abc" it will shows error
}


fn explicitLifecycle<'a>(x: &'a str, y: &'a str) -> &'a str {
    if true {
        return x;
    } else {
        return y;
    }
}
//fn explicitLifecycleError(x: &str, y: &str) -> &str {
    //if true {
      //  return x;
    //} else {
      //  return y;
    //}
//}