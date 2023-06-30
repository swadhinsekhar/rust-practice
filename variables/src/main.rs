

const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;


fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles..", ready, missiles);
    missiles -= ready;
    println!("{} missiles left", missiles);
}

//fn test() {
    /*
    //declare & initialize variables
    let bunnies: i32 = 4;
    let mut bunnies: i32 = 32;
    let (bunnies, puppies)  = (4, 8);
    println!("{}", bunnies);
    */

    /*
    let mut x = 5; //x is mutable
    let x = x; //x is now immutbale
    */

    /*
    //scope
    let x = 5;
    {
        let x = 99;
        println!("{}", x) // prints "99"
    }
    println!("{}", x); //prints "5"
    */

    /*
    //shadow a variable
    let meme = "More cowbell!";
    let meme = make_image(meme);
    */

 //   println!("Hello, world!");
//}
