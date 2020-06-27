fn skipe_prefix<'a,'b>(line:&'a str, prefix: &'b str) -> &'a str{

    return line;
}

pub fn exec(){

    let line ="Lang=en hello world";
    let lang ="en";
    let v;
    {
        let p =format!("lang{}",lang);
        v = skipe_prefix(line, p.as_str());
    }
    println!("lifetimes exec{}",v);

    let name ="lifetimes";
    let foo = Foo{name:name};
    println!("lifetimes foo.name:{}",foo.name);
    let r_x = foo.x();
    println!("lifetimes r_x:{}",r_x);

    let x: &'static str ="hello world";
    static FOO:i32 = 6;
    static y:&'static i32 =&FOO; 
    println!("lifetimes y{}",y)
}

/// 如果Foo的引用比name的引用活的久，就意味着有时候name会无效
struct Foo<'a> {
    name: &'a str,
}

impl <'a> Foo <'a>{

    fn x(&self)-> &'a str{
        self.name
    }
}