fn main() {
    let mut x = 10;

    {
        //would change the x obove
        //x = 15;

        let x = 15;
        //we make so called shadowing uwu
        //it only exists here

    }
    //when we make it here we can change the data type (aka just overwrite the whole x
    let x = "X is now a string";
    println!("{}", x);

    let x = true;
    println!("{}", x);

}