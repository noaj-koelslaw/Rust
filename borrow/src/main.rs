use std::io;


fn main() {
    let mut name = String::from("");
    ask_name(&mut name);

    let mut n1: u32 = ask_number1();
   let mut n2: u32 = ask_number2();



   let finalnumber = add_numbers(n1,n2);

    println!("name is {name}");
    println!("number is {finalnumber}");
}

fn ask_name(name: &mut String) {
    println!("input name");
    io::stdin()
            .read_line(name)
            .expect("Failed to read line");
}

fn ask_number1() -> u32 {
    println!("input a number");
    let mut n1 = String::new();
    io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read line");
    let n1: u32 = n1.trim().parse().expect("REASON");

    n1
}

fn ask_number2() -> u32 {
    println!("input a number");
    let mut n2 = String::new();
    io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read line");
    let n2: u32 = n2.trim().parse().expect("REASON");

    n2
}

fn add_numbers(n1: u32,n2: u32) -> u32 {
    let finalnumber = n1 + n2;
    finalnumber
}



//
//write a sentence in main
//ask for name
//ask for two input numbers numbers
//add numbers
//print in main
//
//