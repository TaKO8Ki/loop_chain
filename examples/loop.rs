fn main() {
    let mut foo = 0;
    loop_chain::loop_chain! {
        loop;
        foo += 1;
        if foo > 3 {
            break
        };
        for x in 0..10;
        then {
            println!("foo: {}, x: {}", foo, x);
        }
    }
}
