

fn borrowing(){

    let i = 3 ;
    borrowing_two(&i);

    let mut v = 6;
    borrowing_three(&v);
}

fn borrowing_two(v:&i32){

    println!("v:{}",v)

}

fn borrowing_three(v:&i32){

    v=v+1;

    let mut x =5;
    {
        let y=&mut x;
        *y+=1;
    }
    println!("borrowing_three:{}",x)
}