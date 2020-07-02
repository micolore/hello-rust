# Rust Language

每个问题都要作给别人分享讲授的准备？（费曼学习法）

## 一、what、how&why

###   包

#### 怎么调用其他包的方法或者结构体？

#### 怎么使用系统包的一些方法？比如标准包的Vec（vector）

####  cargo

### 数据结构（原型与扩展）  

#### 怎么使用数组slice存取数据？

#### tuples 是什么？可以详细介绍一下吗？

#### 泛型

### 内存

#### Rust 为什么是内存安全的语言？

#### Rust 的生命周期是什么？作用域是什么意思？

#### 所有权是什么意思？

#### 借用与引用的区别是什么？

#### 什么是数据竞争？

#### 生命周期注解是什么？

####  描述性与规定性的区别在哪里？

#### 生命周期省略？

输入与输出生命周期，等于函数的参数与返回值

### 并发

#### 如何创建多线程

##### 多线程常见的问题

* 竞争状态
* 死锁
* 难以重现

实现方式有下面这些

* 消息传递

* 共享状态

* Sync&Send trait

### 变量

#### 可变性相关的概念

内部可变性、外部可变性、字段级别可变性

### 闭包

#### rust的闭包与函数有什么区别？

#### rust闭包与其他语言的闭包有什么区别？

### trait

#### trait是什么

 类似java的接口（interface）



### 其他的底层问题

#### 语言的线程模型指的是什么？

#### m:n线程模型是什么？

#### 语言的运行时与线程有什么关系

#### java的线程间通信是怎样实现的？共享内存？通信？

golang、rust都是采用的channel通信的方式进行通信，而java是使用共享内存来实现的。

#### 共享内存是怎么进行工作的？

多个线程使用同一个变量，谁拿到了锁谁就执行。

#### java的线程通信方式有几种？

同步、while轮询、wait/notify

#### rust标准库与语言本身有什么关系？send&sync




## 三、编程语言

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

  

  