fn main() {
    let mut x = 0;
    let a = [1,2,3,4,5];
    loop {
        let mut number = a[x];
        println!("{number}");
        x += 1;
        if x > 4 {
            break
        }
    }
}
