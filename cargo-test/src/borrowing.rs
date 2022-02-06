fn main() {
    //make refferences uwu



    //you cannot borrow x as immutable when it is already borred as mutable (for example in print)

    let mut x = 10;
    // &x = &variable

    //refferences that CAN'T CHANGE
    let x_refference = &x;
    let siro = &x;

    //refference that can change
    //we can now change x with daddy
    {
        let daddy = &mut x;
        //to change make:
        *daddy += 1;
    }
    //we need the codeblock otherwise we couldnt make this now:

    println!("x is {} ", x);


    //sooo you can have multiple unmutable refferences to an variable
    //BUT NOT multiple mutable onces, so I need to close them again

}