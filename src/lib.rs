//! This crate provides a single macro called `loop_chain!`.
//!
//! `loop_chain!` enable you to write long chains of nested Loop
//! expressions such as `for`, `while`, `while let` and `loop`
//! without the associated rightward drift.
//!
//! # Examples
//!
//! ## Quick start
//!
//! ```rust
//! fn main() {
//!     loop_chain::loop_chain! {
//!         for width in 0..10;
//!         for height in 0..10;
//!         then {
//!             println!("width: {}, height: {}", width, height);
//!         }
//!     }
//! }
//! ```
//!
//! the generated code will be the following:
//!
//! ```rust
//! fn main() {
//!     for width in 0..10 {
//!         for height in 0..10 {
//!             println!("width: {}, height: {}", width, height);
//!         }
//!     }
//! }
//! ```
//!
//! ## While expression
//!
//! ```rust
//! fn main() {
//!     let mut foo = 0;
//!     loop_chain::loop_chain! {
//!         while foo < 3;
//!         foo += 1;
//!         for x in 0..10;
//!         then {
//!             println!("foo: {}, x: {}", foo, x);
//!         }
//!     }
//! }
//! ```
//!
//! ## While let expression
//!
//! ```rust
//! fn main() {
//!     let mut foo = (0..10).collect::<Vec<u8>>();
//!     loop_chain::loop_chain! {
//!         while let Some(v) = foo.pop();
//!         for x in 0..10;
//!         then {
//!             println!("v: {}, x: {}", v, x);
//!         }
//!     }
//! }
//! ```
//!
//! ## Loop expression
//!
//! ```rust
//! fn main() {
//!     let mut foo = 0;
//!     loop_chain::loop_chain! {
//!         loop;
//!         foo += 1;
//!         if foo > 3 {
//!             break
//!         };
//!         for x in 0..10;
//!         then {
//!             println!("foo: {}, x: {}", foo, x);
//!         }
//!     }
//! }
//! ```

#[macro_export(local_inner_macros)]
macro_rules! loop_chain {
    ($($tt:tt)*) => {
        __loop_chain! { @init () $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __loop_chain {
    (@init ($($tt:tt)*) then { $($then:tt)* }) => {
        __loop_chain! { @expand {} $($tt)* then { $($then)* } }
    };

    (@init ($($tt:tt)*) $head:tt $($tail:tt)*) => {
        __loop_chain! { @init ($($tt)* $head) $($tail)* }
    };

    (@expand {} for $val:tt in $expr:expr; $($tt:tt)+) => {
        for $val in $expr {
            __loop_chain! { @expand {} $($tt)+ }
        }
    };

    (@expand { $($other:tt)+ } for $val:tt in $expr:expr; $($tt:tt)+) => {
        for $val in $expr {
            $($other)+;
            __loop_chain! { @expand { $($other)+ } $($tt)+ }
        }
    };

    (@expand {} while let $pat:pat = $expr:expr; $($tt:tt)+) => {
        while let $pat = $expr {
            __loop_chain! { @expand {} $($tt)+ }
        }
    };

    (@expand { $($other:tt)+ } while let $pat:pat = $expr:expr; $($tt:tt)+) => {
        while let $pat = $expr {
            $($other)+;
            __loop_chain! { @expand { $($other)+ } $($tt)+ }
        }
    };

    (@expand {} while $expr:expr; $($tt:tt)+) => {
        while $expr {
            __loop_chain! { @expand {} $($tt)+ }
        }
    };

    (@expand { $($other:tt)+ } while $expr:expr; $($tt:tt)+) => {
        while $expr {
            $($other)+;
            __loop_chain! { @expand { $($other)+ } $($tt)+ }
        }
    };

    (@expand {} loop; $($tt:tt)+) => {
        loop {
            __loop_chain! { @expand {} $($tt)+ }
        }
    };

    (@expand { $($other:tt)* } then { $($then:tt)* }) => {
        $($then)*
    };

    (@expand { $($other:tt)* } $stmt:stmt; $($tt:tt)+) => {
        {
            $stmt
            __loop_chain! { @expand { $($other)* } $($tt)+ }
        }
    };
}

#[cfg(test)]
mod test {
    use super::loop_chain;
    #[test]
    fn test_for() {
        let mut success = false;
        loop_chain! {
            for _ in vec![1, 2];
            then {
                success = true
            }
        }
        assert!(success)
    }

    #[test]
    fn test_while() {
        let mut success = false;
        let mut x = 1;
        loop_chain! {
            while x < 2;
            then {
                x += 1;
                success = true
            }
        }
        assert!(success)
    }

    #[test]
    fn test_while_let() {
        let mut success = false;
        let mut vec = vec![1, 2];
        loop_chain! {
            while let Some(_) = vec.pop();
            then {
                success = true
            }
        }
        assert!(success)
    }

    #[test]
    fn test_loop() {
        let mut success = false;
        let mut x = 1;
        loop_chain! {
            loop;
            then {
                if x > 2 {
                    break
                }
                success = true;
                x += 1;
            }
        }
        assert!(success)
    }

    #[test]
    fn test_nested_loops() {
        let mut success = false;
        let mut x = 1;
        let mut y = 1;
        loop_chain! {
            for _ in 0..10;
            while y < 5;
            y += 1;
            loop;
            then {
                if x > 2 {
                    break
                }
                success = true;
                x += 1;
            }
        }
        assert!(success)
    }

    #[test]
    fn test_empty() {
        let success;
        loop_chain! {
            then { success = true }
        }
        assert!(success)
    }
}
