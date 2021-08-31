fn main() {
    // Integer Types 8-bit:i8|u8 16-bit:i16|u16 ...32...64...128... arch:isize|usize
    // let guess: u32 = "42".parse().expect("Not a number!");

    // --> 字面量 literals Decimal->98_222 Hex->0xff Octal->0o77 Binary->0b1111_0000 Byte(u8 only)-> b'A'

    // --> 浮点类型 Floating-Point Types
    // let quotient: f32 = 10.0 / 3.0;
    // print!("{}",quotient);

    // --> 数值运算 Numeric Operations
    // addition
    // let sum = 5 + 10;
    // subtraction
    // let difference = 95.5 - 4.3;
    // multiplication
    // let product = 4 * 30;
    // division
    // let quotient = 56.7 / 32.2;
    // remainder
    // let remainder = 43 % 5;

    // --> 布尔类型 Boolean type
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // --> 字符类型 Character Type
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    // --> 复合类型 Compound Types

    // --> 元组 Tuple Type(特性:复合元素，长度固定)
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup; // a pattern with let to take tup
    // println!("The value of y is: {}", y);
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // --> 数组类型 Array Type(特性:同元素，长度固定，数据分配到堆栈), {向量 vector}
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; 包含 5 个元素为 3 的数组。arry named a will contail 5 elements that will all be set to the value 3 initally.
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];




    // --> 函数 Functions
    // Rust 编程使用蛇形大小写风格命名 funcation 和 variable, 蛇形下所有字母小写，下划线分割。 Rust code uses snake case as the conventional style for Functions and variable names. 
    // another_function(5, 6);
    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {}", y);
    // let x = five();
    // println!("The value of x is: {}", x);
    // let x = plus_one(5);
    // println!("The value of x is: {}", x);

    


    // --> if 表达式 if Expressions
    // let number = 3;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // --> if in a let Statement
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is: {}", number);

    // --> 循环重复 Repetition with Loops
    // loop {
    //     println!("again!");
    // }
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }
    // for number in (1..4).rev(){
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // Questions 
    //     1.华氏度、摄氏度转换。Convert temperatures between Fahrenheit and Celsius.
    //     2.生成第 n 个斐波那契数。Generate the nth Fibonacci number.
    //     3.Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.



    // --> demo
    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{}", s); // This will print `hello, world!`

    // --> 堆栈和堆,内容释放和复制
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); s1 已经被弃用,这里会编译报错
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2); // clone 会把 s1 堆中内存复制.

    // --> 引用和借用 References and Borrowing(使用 & 为借用对象,原变量不会被销毁，被借用后不能修改变量指向的内容)
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);
    
    // --> 可变引用 Mutable References (限制:特定范围内只能由一个可变引用)
    // fn main() {
    //     let mut s = String::from("hello");
    //     change(&mut s);
    // }
    // 可变引用的另一种使用方式，通过创建作用域来使得多个可变引用可以共存。
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    // let r2 = &mut s;
    // 不可变引用不可以有可变引用，不可变引用可以同时有多个多可变引用，因为多个不可变引用读取数据时不会影响到别人读取
    
    // 悬空指针(空指针) Dangling References, 编译时会报错, Rust 编译时会抛出悬空指针得错误。
    // let reference_to_nothing = dangle();
    
    // --> 切片类型 Slice Type
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // // s.clear(); // error!
    // println!("the first word is: {}", word);
    // let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    // let word = first_word(&my_string[..]);
    // println!("the first word is: {}", word);
    // let my_string_literal = "hello world";
    // first_word works on slices of string literals
    // let word = first_word(&my_string_literal[..]);
    // println!("the first word is: {}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);
    // println!("the first word is: {}", word);




    // --> 定义和实例化结构 Defining and Instantiating Structs
    // 
}

// &str 允许对结果进行切片(Slice)
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// &s 试图返回 s 得引用，这样做会被 Rust 编译抛出错误，这里正确做法是直接返回 String, s.
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn plus_one(x: i32) -> i32{
//     x + 1
// }

// fn five() -> i32 {
//     5
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }