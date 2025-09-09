fn main() {
    let mut number = 1000;

    loop{
        
        if number % 3 == 0 {
            println!("number {number} is divisible by three");
        } else {
            println!("number {number} is not divisible")
        }
        number -= 1;
        if number < 0 {
            break
        }
    }

}
