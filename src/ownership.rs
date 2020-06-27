
/// 变量范围
/// 程序运行时主动申请的区域-堆
/// 
/// 变量与数据的交换方式有两种 move and clone
/// 栈中的数据移动方式是复制包括Tuples
/// 
/// 所有权之外
///  函数如何进行转移所有权
/// 

fn data_deap(){

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 value is {}",s2);
    //println!("s1 value is {}",s1);//error 
}

fn data_clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 values is {} s2 value is {}",s1,s2);

}

fn data_ownership(){
    let s = String::from("rust");

    takes_ownership(s);

    let x =5;
    
    makes_copy(x);


}
///
/// rust变量绑定有一个属性，它们有它们所绑定的值的所有权，意味着一个绑定离开作用域它们绑定的资源就会被释放
/// 

fn foo(){
    let v =vec![1,2,3];
}

///
/// 移动语义
/// 对于任何给定的资源都正好只有一个绑定与之对应
/// 意思就是无论如何它都有一个绑定
/// 

pub fn foo2(){

    let v = vec![1,2,3];
    let v2 = v;
    //println!("v:{:?}",v[0]);//error-use of moved value

    let v2 = vec![6,23,4,5];
    take(v2);
    //println!("v2:{:?}",v2);//error-use of moved value-(value borrowed here after move)


    let v3 = vec![1,8,0];
    let v5 =v3;
    //v5.truncate(2);// cannot borrow as mutable
    //println!("v3:{:?}",v3[2]);

}

fn take(i: Vec<i32>){

    //
}
fn takes_ownership(somestring: String){

    println!("{}",somestring);
}

fn makes_copy(someinteger: i32){

    println!("{}",someinteger);
}