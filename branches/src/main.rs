fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("Divisible by 4!")
    } else if number % 3 == 0 {
        println!("Divisible by 3!");
    } else {
        println!("The number is not divisible by 4 and 3!");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
