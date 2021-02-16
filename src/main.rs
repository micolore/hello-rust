mod lifetimes;
mod mutability;
mod ownership;
mod references;
mod slice;
mod function;
mod tuples;
mod vector;
mod concurrency;
mod closures;
mod traits;
mod basic;
fn main() {
    println!("Hello, world!");

    basic::basic_main();

    test_const();

    shadowing();

    test_float();

    test_int();

    test_math();

    multi_variable();

    vector::vector_demo();

    let p = tuples::print_tuples((32, true));
    println!("print_tuples return value:{:?},{:?}", p.0, p.1);

    slice::slice_test();

    // let result_foo = references::foo(23, 6);
    // println!("result_foo:{:?}", result_foo);
    // let result_foo_ferences = references::foo_ferences(&23, &6);
    // println!("result_foo_ferences:{:?}", result_foo_ferences);
    // references::foo_update_value_by_references();

    // ownership::foo2();

    // lifetimes::exec();

    // mutability::two();

    closures::exec();
    traits::trait_person();
    iterators::vector_iter();

    macros::m_call_macro();
}

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
