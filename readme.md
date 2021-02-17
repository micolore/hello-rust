# Rust Language

>  要作给别人分享讲授的准备（费曼学习法）



### 一、rust组织管理

* 箱（Crate）

  > "箱"是二进制程序文件或者库文件，存在于"包"中。

* 包（Package）

  > 当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。

* 模块（Module）

  ```rust
  mod nation {
      mod government {
          fn govern() {}
      }
      mod congress {
          fn legislate() {}
      }
      mod court {
          fn judicial() {}
      }
  }
  ```

  > Rust 中的路径分隔符是 **::** 。
  >
  > 路径分为绝对路径和相对路径。绝对路径从 crate 关键字开始描述。相对路径从 self 或 super 关键字或一个标识符开始描述

  ```rust
  crate::nation::government::govern();
  nation::government::govern();
  ```

* 访问权限

  ```rust
  //1)
  mod nation {
      pub mod government {
          pub fn govern() {}
      }
  
      mod congress {
          pub fn legislate() {}
      }
     
      mod court {
          fn judicial() {
              super::congress::legislate();
          }
      }
  }
  
  fn main() {
      nation::government::govern();
  }
  
  //2)
  mod back_of_house {
      pub struct Breakfast {
          pub toast: String,
          seasonal_fruit: String,
      }
  
      impl Breakfast {
          pub fn summer(toast: &str) -> Breakfast {
              Breakfast {
                  toast: String::from(toast),
                  seasonal_fruit: String::from("peaches"),
              }
          }
      }
  }
  pub fn eat_at_restaurant() {
      let mut meal = back_of_house::Breakfast::summer("Rye");
      meal.toast = String::from("Wheat");
      println!("I'd like {} toast please", meal.toast);
  }
  fn main() {
      eat_at_restaurant()
  }
  
  // 枚举类枚举项可以内含字段，但不具备类似的性质:
  mod SomeModule {
      pub enum Person {
          King {
              name: String
          },
          Quene
      }
  }
  
  fn main() {
      let person = SomeModule::Person::King{
          name: String::from("Blue")
      };
      match person {
          SomeModule::Person::King {name} => {
              println!("{}", name);
          }
          _ => {}
      }
  }
  ```

* 难以发现的模块

  ```rust
  // main.rs
  mod second_module;
  
  fn main() {
      println!("This is the main module.");
      println!("{}", second_module::message());
  }
  // second_module.rs
  pub fn message() -> String {
      String::from("This is the 2nd module.")
  }
  ```

* use 关键字

  > use 关键字能够将模块标识符引入当前作用域，这样就解决了局部模块路径过长的问题。

  ```rust
  //1）
  mod nation {
      pub mod government {
          pub fn govern() {}
      }
  }
  
  use crate::nation::government::govern;
  
  fn main() {
      govern();
  }
  
  //2）
  mod nation {
      pub mod government {
          pub fn govern() {}
      }
      pub fn govern() {}
  }
     
  use crate::nation::government::govern;
  use crate::nation::govern as nation_govern;
  
  fn main() {
      nation_govern();
      govern();
  }
  ```

  

* 怎么进行调用其他包的方法或者结构体？

* 怎么使用系统包的一些方法？比如标准包的Vec（vector）

  > [List of all items in this crate (rust-lang.org)](https://doc.rust-lang.org/stable/std/all.html)

  ```rust
  use std::f64::consts::PI;
  
  fn main() {
      println!("{}", (PI / 2.0).sin());
  }
  ```

  

* cargo如何进行使用的？



### 二、cargo



### 二、数据结构相关（原型与扩展）  

* 怎么使用数组slice存取数据？

* Struct 

  > 都可以将若干个类型不一定相同的数据捆绑在一起形成整体，但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。

  ```rust
  struct Site {
      domain: String,
      name: String,
      nation: String,
      found: u32
  }
  ```

  * 结构体实例

    ```rust
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };
    
    //简化
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        traffic: 2013
    };
    
    //更新
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };
    ```

  * 元组结构体

    > 元组结构体是一种形式是元组的结构体。

    ```rust
    struct Color(u8, u8, u8);
    struct Point(f64, f64);
    
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    
    fn main() {
        struct Color(u8, u8, u8);
        struct Point(f64, f64);
    
        let black = Color(0, 0, 0);
        let origin = Point(0.0, 0.0);
    
        println!("black = ({}, {}, {})", black.0, black.1, black.2);
        println!("origin = ({}, {})", origin.0, origin.1);
    }
    ```

  * 结构体所有权

    > 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。

    ```rust
    #[derive(Debug)]
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
    
        println!("rect1 is {:?}", rect1);
    }
    ```

  * 结构体方法

    > 方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的

    ```rust
    struct Rectangle {
        width: u32,
        height: u32,
    }
       
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
        println!("rect1's area is {}", rect1.area());
    }
    ```

  * 结构体关联函数(类的静态方法)

    > 之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。

    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn create(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
    
    fn main() {
        let rect = Rectangle::create(30, 50);
        println!("{:?}", rect);
    }
    ```

  * 单元结构体

    > ```rust
    > struct UnitStruct;
    > ```

* Tuple 是什么？可以详细介绍一下吗？

  > 元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。

* 枚举

  ```rust
  #[derive(Debug)]
  
  enum Book {
      Papery, Electronic
  }
  
  fn main() {
      let book = Book::Papery;
      println!("{:?}", book);
  }
  
  enum Book {
      Papery(u32),
      Electronic(String),
  }
  
  let book = Book::Papery(1001);
  let ebook = Book::Electronic(String::from("url://..."));
  
  enum Book {
      Papery { index: u32 },
      Electronic { url: String },
  }
  let book = Book::Papery{index: 1001};
  
  ```

  * match 语法

    ```rust
    fn main() {
        enum Book {
            Papery {index: u32},
            Electronic {url: String},
        }
       
        let book = Book::Papery{index: 1001};
        let ebook = Book::Electronic{url: String::from("url...")};
       
        match book {
            Book::Papery { index } => {
                println!("Papery book {}", index);
            },
            Book::Electronic { url } => {
                println!("E-book {}", url);
            }
        }
    }
    ```

  * Option 枚举类

  * if let 语法

### 三、内存管理

* Rust 为什么是内存安全的语言？

  

* Rust 的生命周期是什么？作用域是什么意思？

* 什么是所有权？

  > 为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念

  * 规则

    * Rust 中的每个值都有一个变量，称为其所有者
    * 一次只能有一个所有者
    * 当所有者不在程序运行范围时，该值将被删除

  * 变量范围

    ```rust
    {
        // 在声明以前，变量 s 无效
        let s = "runoob";
        // 这里是变量 s 的可用范围
    }
    // 变量范围已经结束，变量 s 无效
    ```

    > **变量范围**是变量的一个属性，其代表变量的可行域，默认从声明变量开始有效直到变量所在域结束。

  * 内存和分配

    > Rust 之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust 编译器自动添加了调用释放资源函数的步骤。
    >
    > 这种机制看似很简单了：它不过是帮助程序员在适当的地方添加了一个释放资源的函数调用而已

  * 变量与数据交互的方式

    变量与数据交互方式主要有移动（Move）和克隆（Clone）两种：

    * 移动

      ```rust
      let x = 5;
      let y = x; //仅在栈中的数据的"移动"方式是直接复制
      
      let s1 = String::from("hello");
      let s2 = s1;
      println!("{}, world!", s1); // 错误！s1 已经失效
      ```

      

    * 克隆

      ```rust
      fn main() {
          let s1 = String::from("hello");
          let s2 = s1.clone();
          println!("s1 = {}, s2 = {}", s1, s2);
      }
      ```

  * 涉及函数的所有权机制

    * 原理

      ```rust
      fn main() {
          let s = String::from("hello");
          // s 被声明有效
      
          takes_ownership(s);
          // s 的值被当作参数传入函数
          // 所以可以当作 s 已经被移动，从这里开始已经无效
      
          let x = 5;
          // x 被声明有效
      
          makes_copy(x);
          // x 的值被当作参数传入函数
          // 但 x 是基本类型，依然有效
          // 在这里依然可以使用 x 却不能使用 s
      
      } // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放
      
      
      fn takes_ownership(some_string: String) {
          // 一个 String 参数 some_string 传入，有效
          println!("{}", some_string);
      } // 函数结束, 参数 some_string 在这里释放
      
      fn makes_copy(some_integer: i32) {
          // 一个 i32 参数 some_integer 传入，有效
          println!("{}", some_integer);
      } // 函数结束, 参数 some_integer 是基本类型, 无需释放
      ```

  * 函数返回值的所有权机制

    ```rust
    fn main() {
        let s1 = gives_ownership();
        // gives_ownership 移动它的返回值到 s1
    
        let s2 = String::from("hello");
        // s2 被声明有效
    
        let s3 = takes_and_gives_back(s2);
        // s2 被当作参数移动, s3 获得返回值所有权
    } // s3 无效被释放, s2 被移动, s1 无效被释放.
    
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        // some_string 被声明有效
    
        return some_string;
        // some_string 被当作返回值移动出函数
    }
    
    fn takes_and_gives_back(a_string: String) -> String { 
        // a_string 被声明有效
    
        a_string  // a_string 被当作返回值移出函数
    }
    ```

  * 引用与租借

    ```rust
    fn main() {
        let s1 = String::from("hello");
        let s2 = &s1; //& 运算符可以取变量的"引用"。
        println!("s1 is {}, s2 is {}", s1, s2);
    }
    
    fn main() {
        let s1 = String::from("hello");
    
        let len = calculate_length(&s1);
    
        println!("The length of '{}' is {}.", s1, len);
    }
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```

    > 引用不会获得值的所有权。
    >
    > 引用只能租借（Borrow）值的所有权。
    >
    > 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：

  * 垂悬引用（Dangling References）

    ```rust
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String {
        let s = String::from("hello");
        &s
    }
    ```

    

* 借用与引用的区别是什么？

* 什么是数据竞争？

* 生命周期注解是什么？

  > 生命周期注释是描述引用生命周期的办法。

  ```rust
  &i32        // 常规引用
  &'a i32     // 含有生命周期注释的引用
  &'a mut i32 // 可变型含有生命周期注释的引用
  
  fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
      if s2.len() > s1.len() {
          s2
      } else {
          s1
      }
  }
  
  fn main() {
      let r;
      {
          let s1 = "rust";
          let s2 = "ecmascript";
          r = longer(s1, s2);
          println!("{} is longer", r);
      }
  }
  ```

  

* 描述性与规定性的区别在哪里？

* 生命周期省略？

* 输入与输出生命周期，等于函数的参数与返回值

##### 1、生命周期

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

fn longer(s1: &str, s2: &str) -> &str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

fn main() {
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
    }
    println!("{} is longer", r);
}
```



### 四、并发编程

* rust如何创建多线程

* 多线程常见的问题
  * 竞争状态
  * 死锁
  * 难以重现

* 实现方式有下面这些
  * 消息传递
  * 共享状态
  * Sync&Send trait

### 五、变量&函数

* 可变性相关的概念

  *  内部可变性、外部可变性、字段级别可变性

* 声明变量的特殊之处

  声明成int之后，就不能用别的值进行赋值了。

  ```rust
   --> src/basic.rs:4:10
    |
  4 |     a  = 2.1;
    |          ^^^ expected integer, found floating-point number
    |
    = note: expected type `{integer}`
               found type `{float}`
  
  error: aborting due to 2 previous errors
  
  Some errors have detailed explanations: E0308, E0603.
  For more information about an error, try `rustc --explain E0308`.
  ```

  

* 常量与不可变变量的区别

  ```rust
  // 允许
  let a = 123;
  let a = 456;
  
  // 不允许
  const a: i32 = 123;
  let a = 456;
  ```

  > 变量的值可以"重新绑定"，但在"重新绑定"以前不能私自被改变，这样可以确保在每一次"绑定"之后的区域里编译器可以充分的推理程序逻辑。 虽然 Rust 有自动判断类型的功能，但有些情况下声明类型更加方便。

  重影（Shadowing）

  > 重影就是指变量的名称可以被重新使用的机制：

* mut

  ```rust
      let mut b = 456;
      b = 789;
      println!("variable_def b:{}",b);
  ```

* 函数体的语句和表达式

  * 语句(语句是执行某些操作且没有返回值的步骤)

    ```rust
    let a = 6;
    
    let a = (let b = 2);//error
    ```

    

  * 表达式(表达式有计算步骤且有返回值)

    ```rust
    a = 7
    b + 2
    c * (a + b)
    ```

    

  * 复杂表达式({}包含)

    ```rust    
    fn main() {
      let x = 5;
    
      let y = {
        let x = 3;
        x + 1 //注意：**x + 1** 之后没有分号，否则它将变成一条语句！
      };
    
      println!("x 的值为 : {}", x);
      println!("y 的值为 : {}", y);
    }
    ```

    

​        

* 函数嵌套

  ```rust
  fn main() {
      fn five() -> i32 {
          5
      }
      println!("five() 的值为: {}", five());
  }
  ```

* 函数返回值

  ```rust
  fn add(a: i32, b: i32) -> i32 {
      return a + b;
  }
  ```

  





### 七、其他

#### 一、错误处理

> 程序中一般会出现两种错误，可恢复错误和不可恢复错误，可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决，但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。

* rust处理方式

> 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。

##### 1、不可恢复错误

```rust
fn main() {
    panic!("error occured");
    println!("Hello, Rust");
}

//thread 'main' panicked at 'error occured', src\main.rs:3:5
//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

> 回溯是不可恢复错误的另一种处理方式，它会展开运行的栈并输出所有的信息，然后程序依然会退出

##### 2、可恢复的错误

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//1）
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }
}

//2）
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}

//3）
use std::fs::File;

fn main() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
}

```

##### 3、可恢复的错误的传递

```rust
//1）
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

fn main() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}

//2）
fn g(i: i32) -> Result<i32, bool> {
    let t = f(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}

//3）
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool> {
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

fn main() {
    let r = g(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}
```

##### 4、kind 方法

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
```

语言的线程模型指的是什么？

m:n线程模型是什么？

语言的运行时与线程有什么关系

java的线程间通信是怎样实现的？共享内存？通信？

golang、rust都是采用的channel通信的方式进行通信，而java是使用共享内存来实现的。

共享内存是怎么进行工作的？

多个线程使用同一个变量，谁拿到了锁谁就执行。

java的线程通信方式有几种？

同步、while轮询、wait/notify

rust标准库与语言本身有什么关系？send&sync

#### 二、闭包

* rust的闭包与函数有什么区别？

* rust闭包与其他语言的闭包有什么区别？

* trait
  * trait是什么

​              类似java的接口（interface）

#### 三、范型

##### 1、在函数中定义泛型

```rust
//1)
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

fn main() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max(&a));
}

//2)
fn max<T>(array: &[T]) -> T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}
```

##### 2、结构体与枚举类中的泛型

```rust
//1)
struct Point<T> {
    x: T,
    y: T
}
let p1 = Point {x: 1, y: 2};
let p2 = Point {x: 1.0, y: 2.0};

let p = Point {x: 1, y: 2.0};//error

//2)
struct Point<T1, T2> {
    x: T1,
    y: T2
}

//3)
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

//4)
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
}

impl Point<f64> {
    fn x(&self) -> f64 {
        self.x
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

##### 3、特性

```rust
trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8
}

//impl <特性名> for <所实现的类型名>

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}


//2)
trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
}
```

##### 4、特性做参数

```rust
fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

fn output<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}

fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

fn notify(item: impl Summary + Display)
fn notify<T: Summary + Display>(item: T)

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U)

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
          
trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object { 1 }
        else if &self == &object { 0 }
        else { -1 }
    }
}

fn main() {
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
}
```

##### 5、特性做返回值

```
fn person() -> impl Descriptive {
    Person {
        name: String::from("Cali"),
        age: 24
    }
}

fn some_function(bool bl) -> impl Descriptive {
    if bl {
        return A {};
    } else {
        return B {};
    }
}
```

##### 6、有条件实现方法

```rust
struct A<T> {}

impl<T: B + C> A<T> {
    fn d(&self) {}
}
```



#### 四、生命周期



### 八、编程语言

不同的编程语言之间的区别，以及他们的设计思想是什么？

* lua

* perl

* python

* golang

* erlang

* java

* scala

* c++

* lisp

* 易语言
