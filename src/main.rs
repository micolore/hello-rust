mod lifetimes;
mod mutability;
mod ownership;
mod references;
mod slice;
mod function;
mod tuples;
mod vector;
mod concurrency;
mod closures;
mod traits;
mod basic;
fn main() {
    println!("Hello, world!");

    //basic::basic_main();

    //test_const();

    //test_int();

    //vector::vector_demo();

    //let p = tuples::print_tuples((32, true));
   // println!("print_tuples return value:{:?},{:?}", p.0, p.1);

    //slice::slice_test();

    // let result_foo = references::foo(23, 6);
    // println!("result_foo:{:?}", result_foo);
    // let result_foo_ferences = references::foo_ferences(&23, &6);
    // println!("result_foo_ferences:{:?}", result_foo_ferences);
    // references::foo_update_value_by_references();

    // ownership::foo2();

    // lifetimes::exec();

    // mutability::two();

    //closures::exec();
    //traits::trait_person();
    //iterators::vector_iter();

    //macros::m_call_macro();

    concurrency::concurrency_main()
}

