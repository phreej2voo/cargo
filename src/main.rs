fn main() {
    // println!("Hello, world!");
    // 变量、基本类型、函数、注释和控制流
    // Rust是强类型语言 声明变量使用let关键字
    // mut 声明是可变变量 mutable
    // let a = 12;
    let mut a = 13;
    println!("a is {}, a again is {}", a, a);
    a = 26;
    println!("a is {0}, a again is {0}", a);
    
    // 元祖  tuple  是用一对()包括的一组数组
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;

    // 数组  使用一对[]包括的同类型数据
    let _a = [1, 2, 3, 4, 5];
    // let c: [i32; 5];

    println!("{}", add(2, 3));
}

// function add method
// 函数名称命名风格 小写字母下划线分割
fn add(a: i32, b: i32) -> i32{
    return a + b;
}
