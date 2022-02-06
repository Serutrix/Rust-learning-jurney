fn main() {
    let x = 10;

    {
        let y = 5;

        println!("{} {}", x, y)

    }
    // print("{} {}", x, y) it wouldnt work because of y




    print_numbers_to(10000);
    println!("{}", is_even(50));

}

fn print_numbers_to(num: u32){
    for n in 1..num{
        println!("{}", n);
    }
}
//function with return type bool
fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}