fn main() {
let melon = &[240, 159, 141, 137];
if let Ok(s) = str::from_utf8(melon) {
    println!("{}", s);
}

for c in "rust".chars() {
    println!("Give me a {}", c)
}

//let c = String::from("hello ");
//let f = String::from("there");
//let cd = c + f;
//println!("{c}{f}")

}
