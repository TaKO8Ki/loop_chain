fn main() {
    let mut foo = 0;
    loop_chain::loop_chain! {
        while foo < 3;
        foo += 1;
        for x in 0..10;
        then {
            println!("foo: {}, x: {}", foo, x);
        }
    }
}
