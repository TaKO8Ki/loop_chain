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
            $stmt;
            __loop_chain! { @expand { $($other)* } $($tt)+ }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_loop_chain() {}
}
