fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // This causes an error "'if' and 'else' have incompatible types".
    //let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
