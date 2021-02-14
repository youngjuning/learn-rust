fn main() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let one = tup.1;
    println!("The value of y is: {}", y);
}
