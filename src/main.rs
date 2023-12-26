fn concatenate_strings (a: &str, b:&str) -> String {
    let mut result = String::from(a);
    result.push_str(b);
    result
}

fn main() {
    let string1: String = String::from("Hello");
    let string2: String = String::from("World");

    let result = concatenate_strings(string1.as_str(), string2.as_str());
    println!("{}", result);
     
}