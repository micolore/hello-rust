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

    v
}

fn foo_two(v: Vec<i32>, v1: Vec<i8>)-> (Vec<i32>,Vec<i8>){

    (v,v1)
}