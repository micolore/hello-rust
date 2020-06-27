
pub fn print_tuples(pair:(i32,bool))->(bool,i32){
    let p = pair;
    println!("pair:{:?},{:?}",p.0,p.1);
    //解构
    let (a,b) =p;
    println!("a:{:?},b:{:?}",a,b);
    (false,23)
}