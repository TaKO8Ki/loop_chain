<div align="center">

 # loop_chain

 A Rust macro for writing nested loop expressions

 [![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/loop_chain/CI/main)](https://github.com/TaKO8Ki/loop_chain/actions) [![crates](https://img.shields.io/crates/v/loop_chain.svg?logo=rust)](https://crates.io/crates/loop_chain) [![docs](https://img.shields.io/badge/docs-loop_chain-8da0cb?labelColor=555555&logo=rust)](https://docs.rs/loop_chain)

 [Usage](#Usage) | [Examples](examples) | [Docs](https://docs.rs/loop_chain)

</div>

## Dependencies

```toml
[dependencies]
loop_chain = "0.1.1"
```

## Usage

### For expression

```rust
fn main() {
    loop_chain::loop_chain! {
        for width in 0..10;
        for height in 0..10;
        then {
            println!("width: {}, height: {}", width, height);
        }
    }
}
```

the generated code will be the following:

```rust
fn main() {
    for width in 0..10 {
        for height in 0..10 {
            println!("width: {}, height: {}", width, height);
        }
    }
}
```

###  While expression

```rust
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

```

the generated code will be the following:

```rust
fn main() {
    let mut foo = 0;
    while foo < 3 {
        for x in 0..10 {
            println!("foo: {}, x: {}", foo, x);
        }
    }
}
```

###  While let expression

```rust
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
```

the generated code will be the following:

```rust
fn main() {
    let mut foo = (0..10).collect::<Vec<u8>>();
    while let Some(v) = foo.pop() {
        for x in 0..10 {
            println!("v: {}, x: {}", v, x);
        }
    }
}
```

### Loop expression

```rust
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
```

the generated code will be the following:

```rust
fn main() {
    let mut foo = 0;
    loop {
        foo += 1;
        if foo > 3 {
            break
        };
        for x in 0..10 {
            println!("foo: {}, x: {}", foo, x);
        }
    }
}
```

## Reference

- [if_chain](https://github.com/lambda-fairy/if_chain)
