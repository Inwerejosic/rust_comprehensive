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

    // Rust Arrays

    // let mut a:[i8; 5] = [5, 4, 3, 2, 1];
    // a[0] = 7;
    // println!("a: {a:?}");

    let week_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let today = week_days[3];
    let tomorrow = week_days[4];
    let yesterday = week_days[2];
    println!("Today's date is {today:#}, yesterday was {yesterday:#} and tomorrow will be {tomorrow:?}")
}
