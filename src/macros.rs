macro_rules! say_hello {

    ()=>(
        println!("hello macro");
    )
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("you called{:?}()",stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?}==>{:?}",stringify!($expression),$expression);
    };
}

macro_rules! find_min  {
    ($x:expr) =>($x);

    ($x:expr,$($y:expr),+) =>(
        std::cmp::min($x, find_min!($($y),+))
    )
}


/// 指示符 
/// block、expr、ident、item、pat(pattern)、path、stmt(statement)、tt(token tree)、ty(type)
/// 
/// 重载 overload
/// 
/// dry
/// 
pub fn m_call_macro(){

    say_hello!();
    create_function!(foo);
    foo();

    print_result!(2);

    print_result!(
        {
            let x =1;
            x+1;
        }
    );
    println!("{}",find_min!(12u32,28u32,17u32));

}