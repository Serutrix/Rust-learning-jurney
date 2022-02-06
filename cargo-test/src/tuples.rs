fn main() {
//how to make tuples

    let tup1 = (20, 30, 40, 50, 60, "Rust", false, (1, 4, 5));
    let (a, b, c, d, e, f, g, h) = tup1;

    println!("{} {} {} {}", tup1.2, tup1.5, tup1.6, (tup1.7).1);

    println!("a is {}", a);
    println!("a is {}", b);
    println!("a is {}", c);

//I am col
}