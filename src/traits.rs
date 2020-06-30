///
/// 
/// copy trait(借用) 基本数据类型(栈)
/// 
fn first_trait(){

    let v =1;
    let v2 = v;
    println!("v:{:?}",v); //value is i32
}

fn copy_trait(){
 
    let a = 5;
    let _y = double(a);
    println!("_y{:?}",_y);
}

fn double(i: i32)->i32{
    8*i
}

//交还所有权
fn foo(v: Vec<i32>) ->i32{

    32
}

fn foo_two(v: Vec<i32>, v1: Vec<i8>)-> (Vec<i32>,Vec<i8>){

    (v,v1)
}

pub trait Perpson  {
    fn say(&self){
        println!("i am default trait");
    }
}

pub  struct Man{
    pub name: String,
}

impl Perpson for Man {
    fn say(&self){
        println!("i am man");
    }
}

pub  struct Woman{
    pub name: String,
}

impl Perpson for Woman{
    fn say(&self){
        println!("i am woman");
    }
}
pub fn notify(item: impl Perpson){
    item.say();
}
pub fn notify_two<T:Perpson>(item: T){
    item.say();
}
/// 同一个trait都不可以
pub fn notify_three<T:Perpson>(item: T,item1: T){
    item.say();
    item1.say();
}

///同时进行实现两个不同的trait
// pub fn notify_four(item: impl Woman + Man){
   
// }
// pub fn notify_five<T: Woman + Man>(item: T) {

// }

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// fn easy_format_trait<T,U>(t:T,u:U)->i32 
//    where T:Woman+Clone,
//          U:Man+Clone   
// {

// }
fn easy_format_trait<T>(t:T)->i32 
   where T:Perpson+Clone
{
  23
}

fn trait_return ()-> impl Perpson{

    // let m =Man{
    //     name : String::from("lbj"),
    // };
    // return m;
    println!("return execute!");

    Man {
            name : String::from("lbj"),
    }
}
pub fn trait_person(){
    let hong = Woman{
        name: String::from("h"),
    };
    let hong2 = Woman{
        name: String::from("h"),
    };
    let fa = Man{
        name: String::from("f"),
    };
    // hong.say();
    // notify(hong);
    // notify_two(fa);
    notify_three(hong2,hong);

    let m = trait_return();
   
}

