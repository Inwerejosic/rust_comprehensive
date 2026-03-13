fn main() {
//     let x = 10;
//     if x == 0 {
//         println!("Zero!");
//     } else if x < 100 {
//         println!("biggish");
//     } else {
//         println!("huge")
//     }

    // let x = 10;
    // let size = if x < 20 { "small" } else { "large" };
    // println!("number size: {}", size)

    let mut a:[i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}
