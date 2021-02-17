fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str 函数向 String 空间的尾部添加了一段字面量

    println!("{}", s); // 这里会输出完整的 hello, world!
}
