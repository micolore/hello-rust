use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex,Arc};
use std::rc::Rc;

// concurrency main
pub fn concurrency_main(){
    create_thread();
    // create_thread_two();
    //create_thread_three();
    //thread_message_passing();
    //thread_message_passing_two();
    //thread_message_passing_clone();
    //mutex();
    //thread_shared_state();
}
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
pub fn thread_message_passing() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello channel");
        tx.send(val).unwrap();
    });

    //try_recv() no blocking
    let received = rx.recv().unwrap();
    println!("received val{:?}", received);
}

pub fn thread_message_passing_two() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let v = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("thread_message_passing_two recv:{:?}", recv);
    }
}

pub fn thread_message_passing_clone() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let v = vec![
            String::from("one-clone"),
            String::from("two-clone"),
            String::from("three-clone"),
        ];
        for val in v {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("thread_message_passing_two recv:{:?}", recv);
    }
}


pub fn mutex(){

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num=6;
    }
    print!("mutex m={:?}",m);
}
/// arc atomically reference counted
pub fn thread_shared_state(){
    let m  =Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles{
        h.join().unwrap();
    }
    println!("result:{:?}",*m.lock().unwrap());
}