//fn main() {
//    let mut x = 0;
//    let a = [1,2,3,4,5];
//    loop {
//        let mut number = a[x];
//        println!("{number}");
//        x += 1;
//        if x > 4 {
//            break
//        }
//    }
//}

fn main() {
let mut x = 0;
let a = [10,50,20,30,80,70,90,60,00,40];
    while x < 10 {
        let mut number: i32 = a[x];
        x += 1;
        testing1(number);
    }





}

fn testing1(x: i32) {
    println!("{x}");
}


