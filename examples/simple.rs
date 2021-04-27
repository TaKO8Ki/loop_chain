fn main() {
    loop_chain::loop_chain! {
        for x in vec![1, 2];
        println!("first loop");
        for _ in vec![1, 2];
        for _ in vec![1, 2];
        while false;
        then {
            println!("{}", x);
        }
    }
}
