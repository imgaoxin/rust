fn main() {
    let mut s = 123;
    some_process(&mut s);
    println!("{}", s);

}

fn some_process(i: &mut i32) {
    *i += 1;
}
