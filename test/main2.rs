#[allow(unused_variables)]
fn main() {
    //let a = "HELLO";
    let mut s = String::from("HELLO");
    s.push_str(", WORLD");
    println!("{}", s);

    let a = String::from("heell");
    let b = a.clone();
    println!("a = {}", a);

    let a = 2;
    let b = a;
    println!("a = {}, b = {}", a, b);
}
