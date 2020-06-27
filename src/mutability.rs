use std::sync::Arc;
use std::cell::Cell;

fn first(){
    let x =Arc::new(5);
    let y =x.clone();
}

struct Point {

    x:i32,
    y:Cell<i32>,

}

pub fn two(){

    let point  = Point{x:32,y:Cell::new(54)};
    point.y.set(8);
    println!("mutability y:{:?}",point.y);
}