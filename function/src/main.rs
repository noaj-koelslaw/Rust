use rand::Rng; // 0.8.5

fn main() {
    let mut rng = rand::rng();
    let num_elements = 1000;
    
    // Generate 1000 random numbers (e.g., 0-999)
    let numbers: Vec<String> = (0..num_elements)
        .map(|_| rng.random_range(0..1000).to_string())
        .collect();

    // Join them with commas
    let comma_separated_list = numbers.join(",");

    // Print a portion of the list
    println!("{}", &comma_separated_list[..]); // Print first 50 chars
}

