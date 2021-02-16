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
