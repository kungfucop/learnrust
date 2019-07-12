pub fn run() {
    basicUsage();
}


fn basicUsage() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    for i in &vec {
        print!("value is {:?}", i);
    }
    // try overflow
    assert_eq!(vec.get(5), None);
    assert_eq!(vec.get(1), Some(&2));
    // trying to modify the vec content
    for i in &mut vec {

        *i = 5;
        println!("i is {:?}", i);
    }
    for i in vec { // todo what is the difference
        println!(" the value is {:?}", i);
    }
}