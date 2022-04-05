fn main() {
    /*
        i8 u8
        i16 u16
        i32 u32
        i64 u64
        i128 u128
        isafe usafe

        f32 f64

        utf-8 单引号 字符

        true false
    */
    let int = -0;
    println!("有符号整数 i32 {}", int);

    let int: u32 = 10;
    println!("默认整数 u32 {}", int);

    let x = 3.1;
    println!("默认的浮点数 f64 {}", x);

    let x: f32 = 2.3;
    println!("第二种浮点类型 f32 {}", x);

    let space = "";
    println!("sapce的类型是否有发生改变 {:?}", space);

    let space = space.len();
    println!("sapce的类型是否有发生改变 {:?}", space);

    let special_int = b'a'; // A 65 | a 97
    println!("整数类型的特殊表示方式 只限 u8 {}", special_int);

    let x = 1i8;
    let y = 127i8;
    let sum = x + y;
    // attempt to compute `1_i8 + i8::MAX`, which would overflow
    println!("The sum is overflow {}", sum);
}
