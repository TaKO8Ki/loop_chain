fn main() {
    for_chain::for_chain! {
        for x in vec![1, 2];
        println!("a");
        println!("b");
        for _ in vec![1, 2];
        println!("c");
        for _ in vec![1, 2];
        then {
            println!("{}", x);
        }
    }
}
