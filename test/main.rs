fn main() {
    let mut a = 1;
    println!("a = {}", a);
    a = 2;
    println!("a = {}", a);
    let a = 'c';
    println!("a = {}", a);

    let b = true;
    println!("b = {}", b);

    let c: i8 = 99;
    println!("c = {}", c);
    let d: f32 = 9.9;
    println!("d = {}", d);

    let e: usize = usize::max_value();
    println!("e = {}", e);
    let f: isize = isize::max_value();
    println!("f = {}", f);

    let arr: [char; 5] = ['a','b','d',' ','h']; 
    for a in &arr {
        println!("arrN = {}", a);
    }
    println!("arr4 = {}", arr[4]);

    let tup: (u16, bool, f64) = (1000, false, 15.667);
    let (x, y, z) = tup;
    println!("tup = {:?}", tup);
    println!("tup2 = {}", tup.2);
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    let g = int_plus(2, 9);
    println!("g = {}", g);

    const K: &str = "hello, world";
    println!("const K = {}", K);
    println!("Hello, word!");

    another_function();
    let i = 3;
    another_function2(i);
    println!("i = {}", i);

    let a = 1;
    let mut b = 2;
    let c = b = 5;
    println!("c = {:?}", c); // ()
    println!("a = {}", a);

    let c = {
        println!("fill c");
        b + a + 10
    };
    println!("c = {}", c);

}

fn int_plus(a: i32, b: i32) -> i32 {
    a + b
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(i: i32) {
    println!("i = {}", i);
}
