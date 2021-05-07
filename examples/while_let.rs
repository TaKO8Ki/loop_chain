fn main() {
    let mut foo = (0..10).collect::<Vec<u8>>();
    loop_chain::loop_chain! {
        while let Some(v) = foo.pop();
        for x in 0..10;
        then {
            println!("v: {}, x: {}", v, x);
        }
    }
}
