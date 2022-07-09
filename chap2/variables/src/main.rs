// const MAC_POINTS: u32 = 100_000;

fn main() {
    // 1 变量可变性
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 2 使用下划线开头忽略未使用的变量
    // let _x = 5;
    // let y = 10;

    // 3 变量解构
    // let (a, mut b): (bool, bool) = (true, false);
    // // a = true, 不可变; b = false 可变
    // println!("a = {:?}, b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    // 4 解构式赋值
    // let (a, b, c, d, e);

    // (a, b) = (1, 2);
    // // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有是一个变量名而是使用了 _
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct {e, ..} = Struct { e: 5};

    // assert_er!([1,2,1,4,5], [a, b, c, d, e]);

    //5 变量遮蔽
    // let x = 5;
    // // 在main函数的作用域内对之前的x进行遮蔽
    // let x = x + 1;

    // {
    //     // 在当前的花括号作用域内，对之前的x进行遮蔽
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x is: {}", x);

    // 5-2 变量遮蔽
    // let mut spaces = " ";
    // spaces = spaces.len();

    let _x = 5;
    let _y = 10;
}

// 4 解构式赋值
// struct Struct {
//     e: i32
// }
