fn normalClousure() {
    let add = |x| {
        1 + x
    };
    let add2 = |x: i32| -> i32 { x + 2 };
    println!("show the number {:?}, and another {:?}", add(2), add2(2));
    let num = 5;
    let plus_num = |x: i32| x + num;

    assert_eq!(10, plus_num(5));
}

fn errorClousreWithBorrow() {
    let mut num = 5;
    let plus_num = |x: i32| x + num;

    let y = &mut num;
    println!("this is try error closure with borrow");
    assert_eq!(5, num);
}

fn moveClousureModifyOutside() {
    let mut num = 5;
    let mut add = move |x| num = x + 2;
    add(5);
    assert_eq!(5, num);
}

fn clousureModifyOutside() {
    let mut num = 5;
    let mut add = |x| num = x + 2;
    add(5);
    assert_eq!(7, num);
}

fn closureAsParameter() {
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    assert_eq!(3, answer);
}

fn clousreAsPrameterDyanmic() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(&|x| x + 2);

    assert_eq!(3, answer);
}

fn useFunctionAsClousureParams() {
    fn add(x: i32) -> i32 {
        return x + 1;
    }

    fn callClousure(clousure: &Fn(i32) -> i32) -> i32 {
        clousure(1)
    }

    let value = callClousure(&add);
    println!("ready to verify use function as clousure params");
    assert_eq!(2, value);
}

fn returnClousure() {
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer)
}

pub fn run() {
    clousureModifyOutside();
    normalClousure();
    errorClousreWithBorrow();
    useFunctionAsClousureParams();
    returnClousure();
}
