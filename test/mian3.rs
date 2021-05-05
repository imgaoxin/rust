fn main() {
    let mut s = String::from("Hello");
    let s1 = &s;
    let s2 = &s;
    some_process(s1);
    println!("{}", s2);
    let s3 = &mut s;
    some_process2(s3);
    println!("{}", s3);
    let s4 = &mut s;
    //println!("{}", s3);
    println!("{}", s4);

    some_process3();
}

fn some_process(s: &String) {
    let mut ss = s.to_string();
    ss.push_str(", world");
    println!("{}", ss);
}

fn some_process2(s: &mut String) {
    s.push_str(", world");
    println!("{}", s);
}

fn some_process3() -> String {
    String::from("ni hao")
}
