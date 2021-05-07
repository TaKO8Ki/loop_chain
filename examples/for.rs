fn main() {
    loop_chain::loop_chain! {
        for width in 0..10;
        for height in 0..10;
        then {
            println!("width: {}, height: {}", width, height);
        }
    }
}
