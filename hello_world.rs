fn main() {
    let mut a = String::from("Hello, World");
    a.push_str("!");
    println!("{}", &a[..]);
    // broken
    // println!("{}", &a[..-1]);
    // println!("{}", &a[..301]);
}
