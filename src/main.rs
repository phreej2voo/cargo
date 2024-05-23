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
    let (_x, _y, _z) = _tup;

    // 数组  使用一对[]包括的同类型数据
    let _a = [1, 2, 3, 4, 5];
    // let c: [i32; 5];

    println!("{}", add(2, 3));

    // 条件语句
    let mut number = if a > 0 {1} else {-1};
    println!("numebr is {}", number);


    // 循环
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("the value is: {}", i);
    }

    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    let s = ['p', 'h', 'r', 'e', 'e', 'j'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'j' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    let location = loop {
        let ch = s[i];
        if ch == 'j' {
            break i;
        }
        i += 1;
    };
    println!("\'j\'的索引为 {}", location);

    // 迭代器
    
}

// function add method
// 函数名称命名风格 小写字母下划线分割
fn add(a: i32, b: i32) -> i32{
    return a + b;
}

//Rust中的条件表达式必须是bool类型
