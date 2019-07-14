use std::collections::HashMap;

pub fn run() {
    basicUsage();
    zipHashMap();
    enumerate();
    find();
}

fn basicUsage() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let v_take = v.iter()
        .cloned() // we need this one to take i32 v instead of pointer to i32
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(v_take, vec![1, 2])
}

fn zipHashMap() {
    let names = vec!["WaySLOG", "Mike", "Elton"];
    let scores = vec![60, 80, 100];
    let score_map: HashMap<_, _> = names.iter()
        .zip(scores.iter())
        .collect();
    println!("{:?}", score_map);
}

fn enumerate() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 迭代生成标，并且每两个元素剔除一个
        .filter(|&(idx, _)| idx % 2 == 0)
        // 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用
        // .unzip::<_,_, vec<_>, vec<_>>().1
        .map(|(idx, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + *acm);

    println!("{}", val);
}

fn find() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let result = v.iter().find((|idx| { *idx % 2 == 0 }));
    match result {
        Some(i) => {
            println!("find it : {:?}", i);
        }
        None => {
            println!("find no value");
        }
    }
}