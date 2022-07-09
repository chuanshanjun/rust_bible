fn main() {
    another_function(5, 6.1);
    let x: u32 = 4;
    let y: u32 = 5;
    add(x, y);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn add(x:u32,y:u32) -> u32 {
    x + y
}