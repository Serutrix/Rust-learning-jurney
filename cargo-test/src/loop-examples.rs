fn main() {
    //first cargo project
    println!("Hello, world!");

    let plushies = vec!["filu", "kittykat", "lovely cat", "nyancat"];

    let mut i = 0;
    let numbers =  30..51;
    loop {
        println!("the loop is working!{}", i);
        i += 1;
        if i > 100 {
            println!("leaving loop");
            break;
        }

    }
    while i > 0{
        i -= 1;
        println!("while loop i= {}", i);

    }

    for i in 1..44 {
        println!("number: {}", i);

    }
    for o in numbers{
        println!("{}", o);
    }
    //with .iter() you can use vectores as interators for for loops
    //without .iter() it will move the ovner ship to the for loop, what isnt the goal here
    for p in plushies.iter(){
        println!("plushies in collection: {}", p)
        //p has always the last value inside it.

    }
    //.enumerate gives the index number to index
    for (index,p) in plushies.iter().enumerate(){
        println!("plushies in collection: {}, it is at the position: {}", p, index)
        //p has always the last value inside it.

    }

    //without .iter() you couldnt excess the values here  anymore

}
