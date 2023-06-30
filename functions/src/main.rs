

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of (width, height);
    println!("Area is {}", area);
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(width: i32, height: i32) -> i32 {
    width * height
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}


/*
fn main() {
    println!("Hello, world!");
    do_stuff(2.0, 12.5);
}

// -> is return variables
fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz !", qty, oz);
    qty * oz
}
*/
/*

function returns
 { return true; } is equal to  { true }

*/

