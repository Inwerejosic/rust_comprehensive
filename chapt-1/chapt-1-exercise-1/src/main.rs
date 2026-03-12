// fn main() {
//     let x:i16 = 10;
//     println!("x: {x}")
// }

fn interproduct(a: i32, b:i32, c:i32) -> i32 {
    return a * b + b * c * a;
}

fn main(){
    println!("result: {}", interproduct(120, 100, 248))
}