fn main() {
    loop_chain::loop_chain! {
        for x in 0..10;
        if x > 3 { println!("x: {} is larger than 3", x) };
        for y in 0..10;
        then {
            println!("x: {}, y: {}", x, y);
        }
    }
}
