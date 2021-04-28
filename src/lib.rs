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
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     if y.len() == 2;
//!     if let Some(z) = y;
//!     then {
//!         do_stuff_with(z);
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     if y.len() == 2 {
//!         if let Some(z) = y {
//!             do_stuff_with(z);
//!         }
//!     }
//! }
//! ```
//!
//! ## Fallback values with `else`
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     if let Some(z) = y;
//!     then {
//!         do_stuff_with(z)
//!     } else {
//!         do_something_else()
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     if let Some(z) = y {
//!         do_stuff_with(z)
//!     } else {
//!         do_something_else()
//!     }
//! } else {
//!     do_something_else()
//! }
//! ```
//!
//! ## Intermediate variables with `let`
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     let z = y.some().complicated().expression();
//!     if z == 42;
//!     then {
//!        do_stuff_with(y);
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     let z = y.some().complicated().expression();
//!     if z == 42 {
//!         do_stuff_with(y);
//!     }
//! }
//! ```
//!
//! ## Type ascription
//!
//! ```rust,ignore
//! let mut x = some_generic_computation();
//! if_chain! {
//!     if x > 7;
//!     let y: u32 = another_generic_computation();
//!     then { x += y }
//!     else { x += 1 }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! let mut x = some_generic_computation();
//! if x > 7 {
//!     let y: u32 = another_generic_computation();
//!     x += y
//! } else {
//!     x += 1
//! }
//! ```
//!
//! ## Multiple patterns
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Foo(y) | Bar(y) | Baz(y) = x;
//!     let Bubbles(z) | Buttercup(z) | Blossom(z) = y;
//!     then { do_stuff_with(z) }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! match x {
//!     Foo(y) | Bar(y) | Baz(y) => match y {
//!         Bubbles(z) | Buttercup(z) | Blossom(z) => do_stuff_with(z)
//!     },
//!     _ => {}
//! }
//! ```
//!
//! Note that if you use a plain `let`, then `if_chain!` assumes that the
//! pattern is *irrefutable* (always matches) and doesn't add a fallback branch.

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
