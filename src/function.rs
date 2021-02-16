// test_fn_mod 入口
fn function_main(){
    fn_return();

    ft(1, 2);

    print_number(23);

}

/// 
/// 表达式与语句的区别在什么地方？
///  a: 表达式返回一个值，语句不会
/// rust是基于什么类型的语言？
/// 声明语句与表达式语句，其余一切都是表达式
/// let mut var =(let y = 5) error
/// let y = 5 right
/// 表达式语句的目的是把所有的表达式变成语句
/// 为什么rust不推荐使用return

//发散函数
fn diverges() -> ! {
    panic!("diverges");
  //println!!("diverges");error
}

fn plus_one(i: i32) -> i32{
    println!("plus_one:{:?}",1+i);
    1+i
}

/// function ptr
pub fn ft(x: i32 , y:i64){
    //let f: fn(i32)->i32= plus_one;

    let f = plus_one;
    f(1);

    println!("i am mod basic x:{} , y:{}",x,y);
}

fn exceotion(){
    let y =1;
    let x = {
        let x = 3;
        x +1
    };
    println!("v value is {}",x);
}

pub fn fn_return() -> i32{
    println!("fn_return exec...");
    23
}

fn test_for(){

    let is =[1,2,3];
    for i in is.iter(){
        println!("value is {}",i);
    }
}

fn test_loop(){
    let x =0;

    loop {
        println!("loop is {}",x);
    }
}

pub fn print_number(number: i32){
    println!("print_number: {:?}",number);
}