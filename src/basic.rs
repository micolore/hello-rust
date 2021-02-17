pub fn basic_main(){
    variable_def();
    shadowing_def();
}
 fn variable_def(){

    let a = 12;
    //a = "abc"; // error .rustc --explain E0308
    //a = 4.56; // Rust 语言不允许精度有损失的自动数据类型转换。
    //a = 456;  //a 不是个可变变量
    println!("variable_def a:{}",a);

    let mut b = 456;
    b=789;
    b=b+1;
    println!("variable_def b:{}",b);
}
//重影
 fn shadowing_def(){
    let x = 6;
    let x = x+1;
    let x = x*2;
    println!("The value of x is:{}",x)
}

// 变量名称重复声明
fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value x is {}", x);

    //let mut s = "123";
    //expected `&str`, found `usize`rustc(E0308)
    //s = s.len();
}

fn test_const() {
    let a = 12;
    println!("a is {}", a);
    //占位符
    //Rust 中格式字符串中的占位符不是"% + 字母"的形式，而是一对 {}
    println!("a is {0}  a is {0}", a);

    let b = 23;
    const BA: i32 = 12;
    println!("ba: {} , b：{}", BA, b);
}

fn test_int() {}

fn test_float() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x:{} , y:{}", x, y)
}
// 基础运算表达式测试
fn test_math() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quiotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "sum:{}, difference:{} ,product:{} ,quiotient:{},remainder:{}",
        sum, difference, product, quiotient, remainder,
    )
}
// tup 测试
fn multi_variable() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x:{},y:{},z:{}", x, y, z);

    let numbers = [23, 6, 7, 31, 50];

    let c: [i32; 5] = [1, 2, 6, 4, 5];

    let d = [3; 5];
    println!("numbers 2 value is :{}", numbers[2]);
    println!("c 2 value is :{}", c[2]);
    println!("d 2 value is :{}", d[2]);
}
