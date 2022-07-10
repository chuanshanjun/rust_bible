fn main() {
    // let s1 = "hello";

    // println!("{}", s1);

    // let mut s2 = String::from("hello");

    // s2.push_str(", world!");

    // println!("{}", s2);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // let x: &str = "hello, world!";
    // let y = x;
    // println!("{}, {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello"); // s 进入作用域 

    // takes_ownership(s); // s 的值移动到函数里...
    //                     // ... 所以这里不再有效

    // // println!("s = {}", s);

    // let x = 5;          // x 进入作用域 

    // makes_copy(x);      // x 应该移动函数里
    //                     // 但 i32 是 Copy 的，所以在后面可以继续使用x

    // println!("x = {}", x);

    let s1 = gives_ownership(); // gives_ownership 将返回值 移除s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动 函数中，它也将返回值 移给 s3

    // 这里，s3 移除作用域并被丢弃。 s2 也将移出作用域，但已被移走 所以什么也不会发生。 s1 移出作用域并被丢弃
}

fn gives_ownership() -> String { // 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域

    some_string                 // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {  // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里,some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里,some_integer 移除作用域。不会有特殊操作