fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("THe value of the x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let space = spaces.len();
    println!("There are {space} spaces present.");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
