fn fibu(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibu(n-1) + fibu(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fibu({n}) = {}", fibu(n));
}
