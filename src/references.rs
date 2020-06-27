
///
/// 第一，任何借用必须位于比拥有者更小的作用域。第二，对于同一个资源（resource）的借用，以下情况不能同时出现在同一个作用域下：
///* 1 个或多个不可变引用（&T）
///* 唯一 1 个可变引用（&mut T）
/// y 同一个作用域下面，要么有一个可变引用，要么多个不可变引用。不可能同时存在可变与不可变引用（有时候就需要引用额外的作用域）
/// 
/// 数据竞争
/// 
/// 
pub fn  foo(i1:i32,i2:i32)->i32{
      i1+i2
}

pub fn foo_ferences(i1:&i32,i2:&i32)->i32{

    i1+i2
}


pub fn foo_update_value_by_references(){
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("foo_update_value_by_references:{}", x);
}
