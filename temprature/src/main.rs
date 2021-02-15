use std::io;

fn main() {
    println!("请输入要转换的数值");

    //获取用户输入
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("!不接受符号和表情!");

    //将input转换为f64
    let input: f64 = input.trim().parse().expect("!请输入数字!");

    //打印结果
    conver_c(input);
    conver_f(input);
}

fn conver_c(f: f64) {
    let result = 5.0 * (f - 32.0) / 9.0;

    println!("摄氏温度为: {}", result);
}

fn conver_f(c: f64) {
    let result = 9.0 * c / 5.0 + 32.0;

    println!("华氏温度为: {}", result);
}
