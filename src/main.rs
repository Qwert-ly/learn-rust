/// document
/// # 文档是Markdown
/// 函数的基本格式：fn <函数名> (<参数>) <函数体>
/// ```
/// let x = add(1, 2);
/// ```
fn add(a: i32, b: i32) -> i32 {
    return a+b;
}

mod another_rs_file;

fn main() {
    let a = "pioneer";  //语句statement：执行操作、无返回值
    println!("Hello, {}!", a);

    let mut v = 123;
    let vv: i64 = 987654321;
    let vv = 123; //未使用，便可重新赋值
    v += 456;
    println!("v={}", v);

    const aaa: i32 = 123;
    //let aaa = 456; const不可变

    //Shadowing 重影
    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("x的值是{}", x);

    /*
    let mut s = "123";
    s = s.len(); //不能给str变量赋整型值
    */

    /*整数：
    8-bit   i8  u8
    16-bit  i16 u16
    32-bit  i32 u32
    64-bit  i64 u64
    128-bit i128 u128
    arch    isize usize 用于衡量数据大小，位长度取决于运行的目标平台  32位架构CPU将使用32位位长度的int
    */

    /*
    十进制 98_222
    十六进制 0xff
    八进制 0o77
    二进制 0b1111_0000
    字节(仅u8) b'A'
    */

    /*“基本数据类型”还包括
    bool: true, false
    float: f32, f64
    char
    Tuple(above)
    */

    let _x = 2.0; //f64
    
    
    let sum = 5+10;
    let dif = 95.5-4.3;
    let prod = 4*30;
    let quot = 56.7/32.2;
    let remainder = 43%5;
    let b: bool = true;
    let c = "我1";
    let t: (i32, f64, u8) = (500, 6., 1);
    println!("tuple.1={}", t.1);
    let (x_, y_, z_) = t;
    println!("y_={}", y_);

    let aa = [1, 2, 3, 4, 5];
    let bb = ["January", "二月", "🤩"];
    let cc = [3; 5];
    let first = aa[0];
    let second = aa[1];

    //a[0] = 123; 数组a不可变
    let mut a = [1, 2, 3];
    a[0] = 4;

    println!("{}", add(2, 3));
    //cargo doc可将工程中的注释转换成HTML说明文档


    //表达式expression a=7|b+2|c*(a+b)
    let m=5;
    let n = {let m=3; m+1 }; //函数体表达式：可用函数语句，最后一步骤是表达式，其结果值是整个块代表的值；不能使用return

    println!("m={}", m);
    println!("n={}", n);

    fn five() -> i32 {//函数定义可嵌套；rust不支持自动判断返回值类型，不声明return类型，则函数将被视为纯过程，不允许产生返回值
        5
    }
    println!("five()={}", five());


    let mut num=3;
    if num<5{ //条件*可以*用()包裹；必须是bool
        println!("true");  //不允许用一个语句代替一个块；但仍然支持传统else-if语法
    }else if num<0{
        println!("negative");
    }else{
        println!("false");
    }

    //用函数体表达式 实现类三元条件
    let number = if num>0 {1} else {-1};  // 前后两函数体表达式的类型须一致，且必须有else{}
    println!("number={}", number);

    //##循环
    while num != 1 {  //rust无三元for循环
        println!("num in loop{}", num);
        num -= 1;
    }

    let array=[10, 20, 30, 40, 50];
    for i in array.iter() { //rust的for循环实际上使用迭代器.iter()，下详
        println!("i={}", i);
    }
    for i in 0..5 {
        println!("array[{}] = {}", i, array[i]);
    }

    let s=['P', 'I', 'O', 'N', 'E', 'E', 'R'];
    let mut i=0;
    loop{  //无限循环
        let ch = s[i];
        if ch == 'O' {break;}
        println!("\'{}\'", ch);
        i += 1;
    }

    let loc = loop{  //无限循环并返回值
        let ch = s[i];
        if ch == 'O' {break i;}
        println!("\'{}\'", ch);
        i += 1;
    };
    println!("\'O\' is at {}", loc);


    //##迭代
    //迭代器iterator：实现了Iterator trait的类型都可作为迭代器使用
    pub trait Iterator { //惰性laziness求值：方法不会立刻执行，需要时才产生值。用于对集合进行逐步的访问和操作
        type Item;
        fn next(&mut self) -> Option<Self::Item>; //trait方法next：逐一返回iter中的下一个元素，返回None表示结束
    }

    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter(); //集合的不可变引用iter
/*  let iter_mut = vec.iter_mut(); //集合的可变引用iter
    let into_iter = vec.into_iter(); //转移集合所有权，生成值迭代器（获取所有权的迭代器）*/

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
    /*
    map()		对每个元素应用给定的转换函数
    filter()	根据给定条件，过滤集合元素
    fold()		对集合元素进行累积处理
    skip()		跳过指定数量的元素
    take()		获取指定数量的元素
    enumerate()	为每个元素提供索引
    */
    let squared_v: Vec<i32> = v.iter().map(|x| x*x).collect();
    let filtered_v: Vec<i32> = v.into_iter().filter(|&x| x%2==0).collect();

    for num in squared_v { //for循环遍历迭代器，将每个元素赋值给num变量
        println!("squared_num={}", num);
    }

    for num in filtered_v { //for循环遍历迭代器，将每个元素赋值给num变量
        println!("filtered_num={}", num);
    }

    let v = vec![1, 2, 3, 4, 5];
    for &num in v.iter(){
        println!("num={}", num);
    }

    //#消耗型适配器：有些方法可以消耗迭代器
/*  collect()	将迭代器转换为集合（向量、hash集）
    sum()		迭代器所有元素之和
    product()	迭代器所有元素之积
    count()		迭代器元素个数*/
    let sum:i32 = v.iter().sum();
    assert_eq!(sum, 15);

    //#适配器：通过方法链改变/过滤迭代器内容，而不立刻消耗
    let doubled: Vec<i32> = v.iter().map(|x| x*2).collect();
    assert_eq!(doubled ,vec![2, 4, 6, 8, 10]);

    //#迭代器链：将多个iter适配器链接在一起
    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val%2 == 0 {continue;}
        println!("val={}", val);
    }
    

    //#自定义迭代器
    struct Counter{count: usize,}

    impl Counter{  //实现构造函数，类似python __init__(self): self.count = 0
        fn new() -> Counter {
            Counter {count: 0}
        }
    }

    impl Iterator for Counter { //trait系统允许为自定义类型添加std定义的行为
        type Item = usize;
        fn next(&mut self) -> Option <Self::Item> { //使用Option枚举处理可能无值的情形（比较c语言设特殊值-1）
            self.count += 1;
            if self.count <= 5 {Some(self.count)}
            else {None}
        }
    }

    let mut counter = Counter::new(); //new()创建Counter迭代器；struct Counter* counter = Counter_new()
    while let Some(num) = counter.next(){ //for num in counter(= Counter())
        println!("mynum={}", num);
    }


    let numbers = vec![1, 2, 3, 4, 5];
    println!("Iterating through the array:");
    for num in numbers.iter() {
        println!("{}", num);
    }
    let squared_num: Vec<i32> = numbers.iter().map(|x| x*x).collect();
    println!("Squared={:?}", squared_num);

    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x%2==0).cloned().collect();
    println!("Even={:?}", even_numbers);


    //##闭包：匿名函数，捕获并存储其环境中的变量。可在作用域外访问变量、在需要时移动/借给闭包
    //|参数| {表达式}
    let calculate = |a: i32, b: i32, c: i32| a*b+c;
    let result = calculate(1, 2, 3);
    println!("cal={}", calculate(2, 3, 4));

    let x = 5;
    let square = |num| num*x;
    println!("square={}, x={}", square(3), x); //借用：捕获外部变量，不能改变所有权；闭包与外部都可使用变量

    let s = String::from("hello");
    let print_s = move || println!("{}", s);
    print_s();
    //println!("{}", s); 所有权已转移给闭包

    fn call_fn<F>(f: F) where F: Fn() {
        f();
    }
    call_fn(move || println!("Hello from closure"));


    fn find_first_positive(nums: &[i32], is_positive: impl Fn(i32) -> bool) -> Option<usize> {
        nums.iter().position(|&x| is_positive(x)) //闭包返回Option类型
    }

    use std::thread; //多线程
    let nums = vec![1, 2, 3, 4, 5];
    let handles = nums.into_iter().map(|num|{
        thread::spawn(move || {num*2})
    }).collect::<Vec<_>>();

    for handle in handles {
        println!("Result: {}", handle.join().unwrap());
    }

    //闭包的类型
    //Fn：不可变地借用环境变量||FnMut：可变地借用环境变了||FnOnce：获取环境变量所有权，只能被调用1次
    fn apply<F>(num: i32, operation: F) -> i32
    where
        F: Fn(i32) -> i32, //接受闭包作为参数
        {
            operation(num)
        }
    
    let num = 5;
    let square = |x| x*x;
    println!("square of {} is {}", num, apply(num, square));

    
    //##所有权
    //每个值都有变量，称作所有者；一次只能有一个所有者。所有者不在运行范围时，值就会被删除
    //变量范围：
    {
        //声明之前，s无效
        let s="pioneer";
        //s可用
    }//变量范围已结束，s无效，编译器自动释放资源

    let x=5; //基本数据类型，大小确定，不需存入堆
    let y=x; //移动：栈中将有两个值5
    let s1 = String::from("hello"); //s1大小不确定，需存入堆
    let s2 = s1; //现在，栈中s2的指针指向堆中的"hello"，s1已经无效了。这是变量超出范围时，方便释放起见
    //println!("{}", s1);

    let s1 = s2.clone();
    println!("s1={}, s2={}", s1, s2);


    //涉及函数的所有权机制：变量作为参数传入函数，相当于移动
    takes_ownership(s1); //s1的值当作参数，传入函数；可以当做s1已被移动，已经无效
    let x = 5;
    makes_copy(x);
    println!("x={}", x);

    //返回值的所有权机制：return val的所有权移动出函数、回到调用处，不会直接释放
    let s1 = gives_ownership(); //函数将返回值移动到s1
    let s2 = String::from("hel");
    let s3 = takes_gives_back(s2); //s2被当做参数而无效，s3获得返回值所有权
    println!("s3={}", s3);


    //引用reference：变量的间接访问，不具有值的所有权，只能借borrow
    let s2 = &s3;
    println!("s2={}, s3={}", s2, s3); //指针没有在栈中复制变量的值，被引用时不会释放
    let len = cal_len(&s3);
    println!("len({})={}", s3, len);

    let mut s2 = &s3;
    let s1 = s3;
    s2 = &s1; //重新从s1借所有权
    println!("s2={}", s2);
    
    let s2 = &s1;
    //s2.push_str("pioneer");  //不能修改租借的值
    println!("s2={}", s2);

    let mut s1 = String::from("hello");
    let s2 = &mut s1; //s2是可变引用
    //let s3 = &mut s1;  可变引用不能多重引用；不可变引用可以。出于并发状态下数据访问碰撞的考虑
    s2.push_str("pioneer");
    println!("s2={}", s2);

    //悬垂引用dangling refernece：即空指针
    /*
    let ref_to_o = dangle();
    fn dangle() -> &String {
        let s = String::from("hello");
        &s
    }*/

    //##切片slice：对数据的部分引用
    let s = String::from("pioneer"); //String记录字符在内存中的起始位置和其长度
    let part1 = &s[..5];  //习惯同python: s[0:5]，表示[x,y)
    let part2 = &s[5..]; //注意到是&str类型。let val: &str = "string";
    //s.push_str("neu");  被切片的str不能改变值
    println!("{}={}+{}", s, part1, part2);
    let str = &s[..]; //将String转为&str

    let part_arr = &arr[..3];
    for i in part_arr.iter()
//或for i: &i32 in arr[..3].iter()
    {println!("{}", i);}


    //##结构体struct：成员称作“字段”
    #[derive(Debug)] //导入调试库
    struct Site { //注意struct只能用来定义，不能用来声明
        domain: String,
        name: String,
        found: u32
    } //结尾不需要;
    
    let name = String::from("PIONEER");
    let pioneer = Site {
        domain: String::from("pioneer.com"),
        name, //等于name: name,
        found: 2001
    };

    let new_pioneer = Site { //更新结构体
        domain: String::from("www.pioneer.com"),
        ..pioneer
    };
    println!("{:?}", new_pioneer);
    println!("{:#?}", new_pioneer);

    //元组结构体：形式是元组的结构体，有名字、固定类型格式
    struct Color(u8, u8, u8);
    let black = Color(0, 0 ,0);
    println!("black=({}, {}, {})", black.0, black.1, black.2);

    //结构体掌握所有字段的所有权：失效时将释放所有字段



    //方法method类似于函数，用于操作结构体实例
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle { //可以分开写，效果等于拼接
        fn area(&self) -> u32
            {self.width*self.height}
        fn wider(&self, rect: &Rectangle) -> bool
            {self.width>rect.width} //调用结构体方法不需填self
        
        fn create(width: u32, height: u32) -> Rectangle
            {Rectangle{width, height}} //结构体关联函数
    }

    let r1 = Rectangle{width: 30, height: 50};
    println!("S(r1)={}", r1.area());
    println!("{}", r1.wider(&Rectangle{width: 40, height: 20}));

    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);

    struct UnitStruct; //无成员的结构体：单元结构体unit struct


    //##枚举类enum
    #[derive(Debug)]
    enum Book {Papery, Electronic}
    let book = Book::Papery;
    let ebook = Book::Electronic;
    println!("book={:?}", book);

    #[derive(Debug)]
    enum Book_ {Papery(u32), Electronic{url: String}}
    let book_ = Book_::Papery(1001);
    // let ebook = Book_::Electronic(String::from("url://..."));
    println!("book_={:?}", book_);

    #[derive(Debug)]
    enum _Book {
        Papery{index: u32},
        Electronic{url: String},
    }
    let _book = _Book::Papery{index: 1001};
    let _ebook = _Book::Electronic{url: String::from("url...")};
    println!("_book={:?}", _book);


    //枚举类绑定的属性要用match访问，match可实现switch型的分支结构
    match _book{
        _Book::Papery{index} => {println!("Papery book {}", index);},
        _Book::Electronic{url} => {println!("E-book {}", url);}
    }

    //match返回值：返回值表达式类型须一致
    match book_{
        Book_::Papery(i) => {println!("{}", i);},
        Book_::Electronic{url} => {println!("{}", url);},
        _ => {} //例外情形
    }

    //#Option枚举类：std库，支持null引用

    enum _Option<T> {
        Some(T),
        None,
    }
    let opt = _Option::Some("Hello"); //定义一个可以为空值的类
    match opt {
        _Option::Some(something) => {println!("{}", something);},
        _Option::None => {println!("opt=Nothing");}
    }

    let opt: _Option<&str> = _Option::None; //初值为空的Option必须明确类型；_Option::可以省略，直接写None或Some()
    match opt {
        _Option::Some(something) => {println!("{}", something);},
        _Option::None => {println!("opt is nothing");}
    }


    let t = Some(64);
    match t {
        Some(64) => println!("Yes"), //
        _ => println!("No"),
    }


    let i = 0;
    match i {
        0 => println!("zero"), //
        _ => {},
    }
    //上面的函数可用if-let缩短：
    if let 0=i {println!("zero");}
    //if-let是2分类match的语法糖，对枚举类也适用
    enum _Book_{
        Papery(u32),
        Electronic(String)
    }
    let book = _Book_::Electronic(String::from("url"));
    if let _Book_::Papery(index) = book {println!("Papery {}", index);}
    else {println!("not papery book");}



//##组织管理
//箱crate：binary程序/库文件，存于包；是树状结构的，根是编译器开始运行时源文件所编译的程序
//包package：cargo new时会创建Cargo.toml。工程实际上就是一个包，由Cargo.toml管理，其描述包的基本信息、依赖项。包至少包含1个箱，可任意多个二进制箱，最多1个库箱
//模块module：一般按所用语言的组织规范来组织，组织单位即墨块模块
    mod nation{
        mod government {fn govern(){}}  //government模块中的函数都是私有/private的，不能访问
        pub mod congress {pub fn legislate(){}} //pub表示公共权限public
        mod court {fn judicial(){super::congress::legislate();}}
    }
/*  crate::nation::government::govern(); 绝对路径 */
    nation::congress::legislate();  //相对路径
    
    //模块中定义的结构体，本身是私有的，字段也默认私有；使用需要用pub声明
    mod back_of_house{
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String
        }
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches")
                }
            }
        }
    }
    pub fn eat_at_res() {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        println!("I'd like {} toast please", meal.toast);
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    }
    eat_at_res();


    //枚举项可以内含字段，但不具备类似性质：
    mod SomeModule {
        pub enum Person {King {name: String}, Queen}
    }
    let person = SomeModule::Person::King {name: String::from("Blue")};
    match person {
        SomeModule::Person::King { name } => {println!("{}", name);}
        _ => {}
    }

    println!("main");
    println!("{}", another_rs_file::message());

    use crate::another_rs_file::message as msg; //将墨块标识符引入当前作用域
    println!("{}", msg());
    

    use std::f64::consts::PI;  //引用标准库
    println!("{}", (PI/2.5).sin());


    //##错误处理
    //可恢复错误：如文件读取失败，表为Result<T,E>；逻辑错误：panic!
    //panic!("erro");//$env:RUST_BACKTRACE=1
    /*enum Result<T, E> {
        Ok(T),
        Err(E)
    }*/

    use std::fs::File;
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {println!("opened");}
        Err(err) => {println!("failed");}
    }
    
    //用if-let简化
    let f = File::open("hello.txt");
    if let Ok(file) = f{println!("opened");}
    else {println!("failed");}

    //使可恢复错误按逻辑错误处理：Result is Err时panic!()
    /*
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("failed"); //向panic!()发送错误信息
*/

    //可恢复错误的传递
    fn fun(i: i32) -> Result<i32, bool>{
        if i>=0 {Ok(i)}
        else {Err(false)}
    }
    fn g(i: i32) -> Result<i32, bool> { //传递了fun()可能出现的错误
        let t = fun(i);
        return match t {
            Ok(i) => Ok(i),
            Err(b) => Err(b)
        };
    }
    //可以更简单地写作：
    fn gun(i: i32) -> Result<i32, bool> {
        let t = fun(i)?;  //?用于取出Result中非异常的值；如有异常就将异常Result返回，因此仅用于fun -> Result<T, E>，其中type(E)=typeof(E, Result(E)?)
        Ok(t)  //由于t!=Err，此处t: i32
    }

    let r = fun(9999);
    if let Ok(v) = r {println!("Ok: f(-1)={}", v);}
    else {println!("Err");}

    let r = gun(9999);
    if let Ok(v) = r {println!("Ok: g(9999)={}", v);}
    else {println!("Err");}

    //用.kind()实现try
    use std::io;
    use std::io::Read;
    use std::fs::File as F;

    fn read_text(path: &str) -> Result<String, io::Error> {
        let mut f = F::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let str_f = read_text("hello.txt");
    match str_f {
        Ok(s) => println!("{}", s),
        Err(e) =>  match e.kind() {
                            io::ErrorKind::NotFound => {println!("no such file");},
                            _ => {println!("can't read file");}
                        }
    }


    //##泛型generic：编写灵活、可重用的函数、结构体、枚举类
    //c++ by template; c doesn't support
    //#在函数中定义泛型
    fn max(array: &[i32]) -> i32 { //可以处理i32，不能处理f64；泛型可以使其处理任何类型
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i] > array[max_index] {max_index = i; }
            i += 1;
        } array[max_index]
    }
    let a = [2, 4, 6, 3, 1];
    println!("max={}", max(&a));

    /*fn _max<T> (array: &[T]) -> T { //这里仅描述泛型的格式，实际编译会出现问题
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i] > array[max_index] //泛型T不支持比大小
            {max_index = i; }
            i += 1;
        } array[max_index]
    } */

    //结构体和枚举类也可以是泛型
    struct Point<T> { //点坐标结构体，T表示描述点坐标的数字类型
        x: T,
        y: T
    }
    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 1.0, y: 2.0}; //自动类型机制
    //let p3 = Point{x: 1, y: 2.0}; //类型不匹配。x: 1时已经将T设为i32，不允许出现f64

    struct _Point<T, U> { //使用两个泛型标识符，就可以支持不同数据类型的坐标
        x: T,
        y: U
    }

    //枚举类也可以是泛型
    enum Option_<T> {
        Some(T),
        None
    }
    enum Result_<T, E> {
        Ok(T),
        Err(E)
    }

    //结构体与枚举类都可以定义方法，方法也应实现泛型的机制
    impl<T> Point<T> { //impl后面必须有<T>，表示Point<T>的方法是泛型的
        fn x(&self) -> &T {&self.x}
    }
    impl Point<f64> { //也可以为其中的一种泛型添加方法
        fn x_(&self) -> f64 {self.x}
    }
    impl<T, U> _Point<T, U> { //impl块本身的泛型没有阻碍其内部方法具有泛型的能力
        fn mixup<V, W>(self, other: _Point<V, W>) -> _Point<T, W> { //将x(T, U)与y(V, W)融合为x(T, W)
            _Point {
                x: self.x,
                y: other.y
            }
        }
    }
    let p = Point{x: 1, y: 2};
    println!("p.x={}", p.x());


    //##特性trait：类似于java interface，都是行为规范，标识哪些类有哪些方法
    trait Descriptive { //规定实现者必须有.describe(&self) -> String
        fn describe(&self) -> String{
            String::from("[Object]") //默认特性
        }
    }

    struct Person {
        name: String,
        age: u8
    }
    impl Descriptive for Person { // impl <trait> for <struct>，实现trait；同一个类可以实现多个trait，每个impl块实现一个
        fn describe(&self) -> String { //可试着将块中内容去掉
            format!("{} is {} years old", self.name, self.age)
        }
    }
    

    //默认特性
    //java接口只能规范方法，但trait可以定义方法作为默认方法。对象可以重新定义方法，也可以不重新定义方法、使用默认的方法
    let p = Person{name: String::from("Blue"), age: 20};
    println!("{}", p.describe()); //impl Descriptive for Person块内容去掉后，将径直输出[Object]


    //特性作为参数
    fn output(object: impl Descriptive) { //任何实现了Descriptive的对象都可以作为参数；函数内无法使用其他属性与方法
        println!("output: {}", object.describe());
    }
    fn output_ <T: Descriptive> (object: T) { //等效语法（语法糖）
        println!("output_: {}", object.describe());
    }
    fn output_two<T: Descriptive>(arg1: T, arg2: T) { //多个参数
        println!("output_two: {}, {}", arg1.describe(), arg2.describe());
    }
    //涉及多个特性可用+链接，如fn notify<T: Descriptive + Clone>(arg: T)；或者fn notify(item: impl Descriptive + Clone)

    //复杂的实现可用where简化：
    //fn fun<T:D+C, U:C+D>(t:T, u:U) -> i32
    //简化为fn fun<T, U>(t:T, u:U) -> i32
    //  where T:D+C, U:C+D

    trait Comparable {
        fn compare(&self, object: &Self) -> i8; //Self是实现trait的类型，Self关键字就代表了当前类型（不是实例）本身
    }
    fn max_<T: Comparable>(array: &[T]) -> &T {
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i].compare(&array[max_index]) > 0 {max_index = i;}
            i += 1;
        }
        &array[max_index]
    }
    impl Comparable for f64 {
        fn compare(&self, other: &f64) -> i8 {
            if self > other {1}
            else if &self == &other {0}
            else {-1}
        }
    }
    let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("max_={}", max_(&arr));

    //特性作为返回值
    fn person() -> impl Descriptive { //只接受实现了该特性的对象做返回值；同一个函数中所有可能的返回值类型必须一样
        Person {
            name: String::from("Blue"),
            age: 20
        }
    }
    //结构体A、B都实现Trait，下面的函数就是错的：
    //fn fun(bool bl) -> impl Descriptive {
    //    if bl {return A{};} else {return B{};}  }

    //有条件实现：区分所属泛型已实现的方法，决定接下来该实现的方法
    /* 
    struct A<T> {}
    impl<T: B+C> A<T> {
        fn d(&self) {}
    } //T实现了B和C，才实现此impl块
    */


    //##生命周期lifetime：rust的引用机制，用于避免悬垂引用（空指针）
    /*
    {
    let r;
    {let x = 5; r = &x; //r指向x，但x的生命周期在这个块结束时就结束了，r指向空指针.引用必须在值的生命周期以内才有效
    } println!("{}", r);
    }
    
    fn longer(s1: &str, s2: &str) -> &str { //返回值的生命周期与参数的生命周期有关
        if s1.len() > s2.len() {s1} else {s2}
    }
    //返回值引用可能会返回过期的引用：
    let result;
    {
        let s1 = "long";
        {
            let s2 = "pioneer";
            result = longer(s1, s2);
        }
    } println!("{} is longer", result); //result被使用时，s1、s2都已经失效。
    //对函数来说，函数不能获知参数的生命周期，为保证传递值正常，必须要用所有权原则消除风险。所以longer()不能通过编译
    */


    //生命周期注释：&'a str，表示str的生命周期是'a，即引用的生命周期是'a
    /*
    &i32//常规引用
    &'a i32//带生命周期注释的引用
    &'a mut i32//可变型带生命周期注释的引用
    */
    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str { //用泛型声明规范生命周期的名称，之后return的生命周期将同两参数的一致
        if s1.len() > s2.len() {s1} else {s2}
    }
    let result;
    {
        let s1 = "long";
        {
            let s2 = "pioneer";
            result = longer(s1, s2);
            println!("{} is longer", result);
        }
    } println!("{} is longer", result);


    //结构体中使用字符串切片引用
    struct Str<'a> {s: &'a str}
    let s = Str{s: "string_slice"};
    println!("s.s={}", s.s);
    impl<'a> Str<'a> {
        fn get(&self) -> &str {self.s} //返回值并没有生命周期注释，但是加上也无妨
    }
    println!("s.get()={}", s.get());

    //静态生命周期：用双引号包括的字符串常量所代表的精确数据类型都是 &'static str
    let s: &'static str = "hello"; //'static 所表示的生命周期从程序运行开始到程序运行结束（如同c语言中的全局变量）

    //泛型+特性+生命周期
    use std::fmt::Display;
    fn longer_with_an_announcement<'a, T>(s1: &'a str, s2: &'a str, ann: T) -> &'a str
    where T: Display { //泛型T必须实现Display特性
        println!("Announcement! {}", ann);
        if s1.len() > s2.len() {s1} else {s2}
    }
    let s1 = "long";
    let s2 = "pioneer";
    let ann = "longer";
    println!("{} is longer", longer_with_an_announcement(s1, s2, ann));



    //##I/O
    //java、c中环境参数以主函数的参数（常常是一个字符串数组）传递给程序；rust中主函数是无参函数，环境参数通过标准库std::env::args()获取
    let args = std::env::args();
    println!("{:?}", args); //inner数组包含唯一字符串，表示当前程序所在的位置
    for arg in args {println!("{}", arg);}

    //std::io提供了标准输入（可认为是命令行输入）的相关功能
    use std::io::stdin;
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("failed to read line.");
    println!("input: {}", str_buf);
    //std::io::Stdio包含read_line读取方法，返回Result枚举类，用于传递读取中出现的错误，所以常用expect()或unwrap()来处理错误
    //尚无法处理中文输入，因为中文字符占用多个字节，而read_line()只读取一个字节
    //尚无直接从cmd读取数字/格式化数据的方法。可以读取一行字符串并使用字符串识别函数

    
    use std::fs;
    let text = fs::read_to_string("hello.txt").unwrap(); //读取一般文件
    println!("{}", text);

    let content = fs::read("hello.txt").unwrap(); //读取二进制文件
    println!("{:?}", content);

    //文件流读取方式
    use std::io::prelude::*;
    let mut buffer = [0u8; 5]; //u8是无符号8位整数，即字节；读取的长度即等于缓冲区长度
    let mut file = fs::File::open("hello.txt").unwrap(); //注意open()是只读，没有配套的.close()
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    fs::write("hi.txt", "hello, world!").unwrap(); //一次性写入。将直接删除原内容，不存在则创建
    let mut file = File::create("hi.txt").unwrap(); //流式写入；注意打开的文件要存放在mut中才能使用File的方法
    file.write_all(b"hello, world!").unwrap(); //b""表示字节字符串，即&[u8]

    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().append(true).open("hi.txt").unwrap(); //应OpenOptions实现用特定方法打开文件
    file.write(b"append?");
    let mut file = OpenOptions::new().read(true).write(true).open("hi.txt").unwrap(); //读写模式
    file.write(b"cover?");


    //##集合与字符串
    //#向量vector：线性地存放类型相同的多值的单数据结构，是**线性表**。使用方式类似列表list
    let mut vector: Vec<i32> = Vec::new(); //类型为i32的空向量
    let mut vector = vec![1, 2, 4, 8]; //通过数组创建向量
    vector.push(16); //向线性表追加和栈的push本质上是一样的，所以只有vec.push()
    println!("{:?}", vector);
    let mut v2: Vec<i32> = vec![16, 32, 64];
    vector.append(&mut v2); //将向量拼接到其他向量的尾部
    println!("{:?}", vector);

    println!("{}", match vector.get(1) { //向量长度无法逻辑推断，.get()不能保证取到值，返回Option枚举类
        Some(value) => value.to_string(),
        None => "None".to_string()
    });
    println!("&vec[2]={}", &vector[2]); //直接访问，若超出范围则panic!
    println!("vec[2]={}", vector[2]); //同上，是简写

    for i in &vector {println!("{}", i);} //遍历向量
    for i in &mut vector {*i += 50; println!("i+50={}", i);} //遍历中更改变量值


    //#字符串
    let string = String::new(); //新建字符串
    let one = 1.to_string(); //整数转字符串
    let flt = 1.3.to_string(); //浮点数转字符串
    let slice = "slice".to_string(); //字符串切片转字符串
    let hello = String::from("你好"); //字符串字面量转字符串；包含UTF-8的字符串
    
    let mut s = String::from("run");
    s.push_str("ning"); //追加字符串切片
    s.push('!'); //追加字符

    let s1 = String::from("pioneer");
    let s2 = s1 + " " //可以包含字符串切片
        + &String::from("neu"); //+运算符重载，将s1的所有权转移给s2

    let s = format!("{}-{}", s2, "hello"); //format!宏，类似于println!
    println!("{}, {}", s2, s);
    println!("len({})={}", s, s.len()); //字符串长度
    println!("cap({})={}", s, s.capacity()); //字符串容量
    
    let s = "pioneer东北大学"; //中文是UTF-8编码，每个字符3字节；而rust支持UTF-8字符对象
    println!("len({})={}", s, s.len());
    println!("len({})={}", s, s.chars().count()); //取字符集合，再统计字符数；速度很慢
    for c in s.chars() {println!("{}", c);} //遍历字符
    let a = s.chars().nth(2); //注意.nth()是从迭代器中取值，不是字符串的方法；也不要在遍历中使用，因为UTF-8字符长度不一定一致
    println!("{:?}", a);
    println!("sub={}", &s[0..5]); //切片，注意是字节位置，而不是字符位置，有可能截断UTF-8字符


    //#映射表map：键值散列映射表hash map
    use std::collections::HashMap;
    let mut map = HashMap::new(); //无须声明类型，因为HashMap::new()返回的是HashMap<K, V>，编译器可以推断出类型
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    println!("map={:?}", map);
    println!("{}", map.get("color").unwrap());
    for (key, value) in &map {println!("{}: {}", key, value);} //遍历映射表
    map.insert("color", "blue"); //.insert()会直接覆盖原值
    map.entry("color").or_insert("red"); //如果键不存在，则插入；存在则返回值
    println!("{:?}", map);

    map.insert("1", "a");
    if let Some(x) = map.get_mut(&"1") {*x="b";} //已经确定有某键的情形，直接修改对应值

    for p in map.iter()
        {println!("{:?}", p);} //iter()返回迭代器，iter_mut()返回可变迭代器；迭代元素是键值对元组




    //##面向对象：数据的封装与继承，能基于数据调用方法
    //#封装：对外显示的策略。rust通过模块机制实现最外层封装，内部封装通过结构体、枚举类、trait实现
    use another_rs_file::ClassName;
    let obj = ClassName::new(1024);
    obj.public_method();

    //继承extend是多态polymorphism思想的实现，多态即可以处理多种类型数据的代码
    //rust通过trait实现多态。特性无法实现属性的继承，只能实现类似于"接口"的功能，所以继承需要在子类中定义父类实例，没有与继承有关的语法糖
    


    //##并发concurrent：程序的不同部分独立执行；并行parallel指的是同时执行。并发往往造成并行
    //#线程thread：操作系统调度的最小单位，是程序执行的基本单位。程序往往在进程process内执行，当中线程共享进程的内存空间
    //有OS的环境，进程由OS交替调度、执行，线程在进程内由程序调度。线程并发很可能遭遇并行
    //java、c#采用运行时runtime软件协调资源，降低执行效率；c(++)在底层支持多线程，但语言本身及编译器不能侦查、回避并行错误
    use std::time::Duration;
    fn spawn_fun() {
        for i in 0..10 {
            println!("spawn thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    thread::spawn(spawn_fun); //创建线程；参数是无参函数，返回值是JoinHandle
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1)); //随着主线程结束，spawn线程也结束了
    }

    //可用闭包传递函数作为参数
    thread::spawn(|| {
        for i in 0..5 {
            println!("closure thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print again {}", i);
        thread::sleep(Duration::from_millis(1));
    }


    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print ...... {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); //可以使子线程运行结束后再停止运行程序


    let s = "Hello";
    let handle = thread::spawn(move || { //move将s的所有权转移给闭包
        println!("{}", s);
    });
    handle.join().unwrap();

    //#通道channel：线程间传递消息
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //子线程获得主线程的发送者transmitter（tx），调用其.send()方法发送消息
    });
    let received = rx.recv().unwrap(); //主线程通过对应的接收者receiver（rx）接收消息
    println!("received: {}", received);



    //##宏macro：编译时生成代码
    //#声明式宏declarative macro
    macro_rules! my_macro { //macro_rules宏，可以匹配代码结构、根据匹配的模式生成相应代码
        ($arg:expr) => {
            //生成的代码；用$arg代替匹配到的表达式
        };
    }

    macro_rules! greet {
        //模式匹配
        ($name:expr) => { //$后是宏规则，分号分隔
            println!("Hello, {}!", $name);
        };
    }
    greet!("pioneer");

    macro_rules! _vec {
        () => {Vec::new()}; //基本情况：空
        ($($element:expr),+ $(,)?) => { //递归情况：有元素；$(,)?用于处理末尾的逗号
            {
                let mut temp_vec = Vec::new();
                $(temp_vec.push($element);)+
                temp_vec
            }
        };
    }
    let my_vec = _vec![1, 2, 3];
    println!("{:?}", my_vec);
    

    //#过程宏procedural macro：能在编译时自定义代码生成过程以操作抽象语法树/AST，接近函数
    //分为派生宏derive macros（实现trait，如Copy、Debug）
    //以及属性宏attribute macros（在声明上附加额外元数据，如#[derive(Debug)]）
    //通常需要proc_macro库，如ToketStream、TokenTree、Group、Ident、Punct、Literal等



    //##智能指针smart pointer：封装了对动态分配内存的所有权、生命周期管理
    //原始指针+引用计数、所有权转移、生命周期管理……
//  Box<T>  在堆上分配内存，不可变，所有权转移
//  Rc<T>   多线程共享所有权；不可变，引用计数
//  Arc<T>  多线程共享所有权、线程安全的共享所有权；不可变，原子引用计数
//  RefCell<T>  内部可变
//  Mutex<T>    互斥锁
//  RwLock<T>   读写锁，读写访问数据
//  Weak<T>     弱引用，避免循环引用


    //Box<T>：堆上分配内存，不可变，所有权转移。可在堆上创建大小已知的数据
    let b = Box::new(5); //Box::new()返回指向堆上的指针
    println!("b={}", b);

    //Rc<T>：引用计数，允许多个所有者共享数据；所有者数=0时释放数据。适用于单线程下数据共享
    use std::rc::Rc;
    let five = Rc::new(5);
    let five1 = Rc::clone(&five); //clone()只增加引用计数，不复制数据
    let five2 = Rc::clone(&five);
    println!("five={}, five1={}, five2={}", five, five1, five2);

    //Arc<T>：用原子操作更新引用计数，可在多线程环境中共享数据
    use std::sync::Arc;
    let data = Arc::new(5);
    let data_clone = Arc::clone(&data);
    println!("data={}, data_clone={}", data, data_clone);

    //RefCell<T>：运行时检查借用规则，能在不可变引用下修改数据；只能用于单线程环境
    use std::cell::RefCell;
    let x = RefCell::new(5);
    let mut borrowed = x.borrow_mut(); //返回RefMut<T>，可变引用
    *borrowed += 1;
    println!("borrowed={}", borrowed);

    //Mutex<T>：互斥锁，确保任一时刻只有一个线程能访问Mutex()内的数据
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); //Mutex::lock()返回MutexGuard<T>，互斥锁
        *num += 1;
    }
    println!("m={}", m.lock().unwrap());

    //RwLock<T>：读写锁，允许多个只读访问或一个可变访问。写入时是排他的，读取时是共享的
    use std::sync::RwLock;
    let rw = RwLock::new(5);
    {
        let num = rw.read().unwrap(); //RwLock::read()返回RwLockReadGuard<T>，不可变引用
        println!("num={}", num);
    }
    {
        let mut num = rw.write().unwrap(); //RwLock::write()返回RwLockWriteGuard<T>，可变引用
        *num += 1;
    }
    println!("rw={}", rw.read().unwrap());

    //Weak<T>：弱引用
    use std::rc::Weak;
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five); //downgrade()返回Weak<T>
    let strong_five = weak_five.upgrade(); //upgrade()返回Option<Rc<T>>，转换为强引用
    println!("weak_five={:?}", weak_five);
    println!("strong_five={:?}", strong_five);


    //智能指针的生命周期管理
    //智能指针销毁时，会自动释放内存，以避免内存泄漏、空指针；还可以在创建时指定特定析构函数，实现自定义资源管理
    //下面用Rc<T>实现引用计数、演示多个所有者共享数据
    #[derive(Debug)]
    struct Data {value: i32,}
    let data = Rc::new(Data{value: 5});
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);
    println!("value={}", data.value);
    println!("data={:?}", data);
    println!("ref_count={}", Rc::strong_count(&data));
    println!("clone1: {:?}", data_clone1);
    println!("clone2: {:?}", data_clone2);


    //##异步async/await
    //长时间I/O中不被阻塞、执行其他任务
    //Future：表示异步操作。是可能未完成的计算，将来会返回值/错误
    //aync：定义异步函数，返回Future。await：暂停当前Future的执行，等待另一个Future的完成
    /*
    use tokio;
    async fn async_task() -> u32 {
        tokio::time::delay_for(Duration::from_secs(1)).await; //等待1秒，模拟异步操作
        42 //返回结果
    }
    async fn execute_async() {
        let r = async_task().await; //等待异步操作完成
        println!("async_task()={}", r);
    }
    #[tokio::main]
    async fn main() {
        println!("Start executing async task...");
        // 调用异步任务执行函数，并等待其完成
        execute_async().await;
        println!("Async task completed!");
    }*/
    

    /*
    use std::error::Error;
    use tokio::runtime::Runtime;
    use reqwest::get;

    // 异步函数，用于执行 HTTP GET 请求并返回响应结果
    async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
        let response = get(url).await?;  // 使用 reqwest 发起异步 HTTP GET 请求
        let body = response.text().await?;
        Ok(body)
    }

    // 异步任务执行函数
    async fn execute_async_task() -> Result<(), Box<dyn Error>> {
        let url = "https://jsonplaceholder.typicode.com/posts/1";  // 发起异步 HTTP 请求
        let result = fetch_url(url).await?;
        println!("Response: {}", result);
        Ok(())
    }

    fn main() {
        let rt = Runtime::new().unwrap(); // 创建异步运行时
        let result = rt.block_on(execute_async_task()); // 在异步运行时中执行异步任务
        match result { // 处理异步任务执行结果
            Ok(_) => println!("Async task executed successfully!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    */

    async fn hello_() -> String  //async定义异步函数，返回(impl) Future。可以包含await表达式，等待其他异步操作完成
        {"hello".to_string()}
    async fn add_(a: i32, b: i32) -> i32
        {a+b}//异步函数返回值类型通常是impl Future<Output=T>，T是异步操作结果类型
    async fn print_hello() { 
        let r = hello_().await; //await等待异步操作完成
        println!("{}", r);
    }

    async { //异步块，可在同步代码中使用异步操作
        let r1 = hello_().await;
        let r2 = add_(1, 2).await;
        println!("results: {}, {}", r1, r2);
    };

    /* 异步任务执行 可用tokio::main、async_std::task::block_on、futures::executor::block_on等，它们接受一异步函数/块，在当前线程/执行环境中执行
    use async_std::task;
    fn main(){
        task::block_on(print_hello());
    }*/
    
    /*
    async fn my_async_fun() -> Result<(), MyError> {
        some_async_fun().await?; //?的作用：若some_async_fun()返回错误，错误会被传播到调用者
    }*/

    /*
    trait MyAsyncTrait {async fn async_method(&self) -> Result<(), MyError>;} //为trait定义异步方法，之后可为不同类型的对象定义异步操作
    impl MyAsyncTrait for MyType {
        async fn async_method(&self) -> Result<(), MyError> {
            some_async_fun().await?; //异步逻辑
        }
    }*/

    //异步代码通常是异步运行时(tokio、async_std)，提供了调度和执行异步任务的机制(异步上下文)
    /*
    #[tokio::main]
    async fn main() {some_async_fun().await;}
    */

    //异步宏：tokio::spawn等，可在异步运行时启动新的异步任务
    /*
    #[tokio::main]
    async fn main() {
        let handle = tokio::spawn(async {//异步逻辑});
        handle.await.unwrap();
    }*/

    //异步I/O：tokio::fs::File、async_std::fs::File
    /*
    use tokio::fs::File;
    use tokio::io::{self, AsyncReadExt};
    #[tokio::main]
    async fn main() -> io::Result<()> {
        let mut file = File::open("file.txt").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        println!("{}", contents);
        Ok(())
    }*/

    //异步通道：tokio::sync::mpsc，允许异步任务之间传递消息
    /*
    use tokio::sync::mpsc;
    use tokio::spawn;

    #[tokio::main]
    async fn main(){
        let (tx, mut rx) = mpsc::channel(32);
        let child = spawn(async move {
            let r = "Hello, world!".to_string();
            tx.send(r).await.unwrap();
        });
        let r = rx.recv().await.unwrap();
        println!("received: {}", r);
        child.await.unwrap();
    }*/



    //##unsafe：允许绕过编译器的安全检查
    //unsafe块：块内可用unsafe代码
    //unsafe函数：允许定义unsafe函数，但是调用时需要在unsafe块内
    //unsafe trait：允许定义unsafe trait，但是实现时需要在unsafe块内
    //unsafe trait impl：允许在unsafe块内实现unsafe trait

    unsafe fn unsafe_fun() {println!("unsafe function");}
    unsafe {
        unsafe_fun();
    }

}

fn takes_ownership(string: String) {println!("{}", string);} //函数结束，string释放

fn makes_copy(int: i32) {println!("{}", int);} //函数结束，而int是基本类型，无需释放

fn gives_ownership() -> String {
    let str = String::from("hello");
    return str; //str被当做返回值，移动出函数
}

fn takes_gives_back(str: String) -> String {str /*str被当作返回值，移动出函数*/}

fn cal_len(s: &String) -> usize {s.len()}