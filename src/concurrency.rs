use std::thread;
use std::time::Duration;
use std::sync::mpsc;

///
///fearless  concurrency
///

pub fn create_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("create_thread thread i:{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..3 {
        println!("create_thread main i :{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn create_thread_two() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("create_thread thread i:{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..3 {
        println!("create_thread main i :{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn create_thread_three() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("create_thread_three:{:?}", v);
    });
    handle.join().unwrap();
}


/// thread message passing  
/// from http://golang.org/doc/effective_go.html
/// 不要通过共享内存来通讯；而是通过通讯来共享内存。”（“Do not communicate by sharing memory; instead, share memory by communicating.”
pub fn thread_message_passing(){

    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello channel");
        tx.send(val).unwrap();
    });

    //try_recv() no blocking
    let received =rx.recv().unwrap();
    println!("received val{:?}",received);

}