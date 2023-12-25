
fn main() {

    let i: &str = "t";
    let u: String = String::from("Start");
    let array = [10,10,10];
    println!("{:?}", array);
    let array : [i64; 2]= [1, 10];
    println!("{:?}", array);
    let slice = &array[0..1];
    println!("{:?}", slice);
    let number = array.len();
    println!("{}", number);
	let _a = 10987;

    {

    let mut u = u.clone();

    u = String::from("Start Now");

    println!("{}", u);
    }
    {

    let mut i = i;

    i = "S";

    println!("{}", i);
    }

    if u == i  {
        println!("X is a String. i = {}. u = {} ", i, u);
    }
    else {
        println!("X is not a String i = {}. u = {}", i, u);
    }

    println!("Start");

    println!("Hello, world!");
}
