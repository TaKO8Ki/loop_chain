#[macro_export(local_inner_macros)]
macro_rules! for_chain {
    ($($tt:tt)*) => {
        __for_chain! { @init () $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __for_chain {
    (@init ($($tt:tt)*) then { $($then:tt)* }) => {
        __for_chain! { @expand {} $($tt)* then { $($then)* } }
    };

    (@init ($($tt:tt)*) $head:tt $($tail:tt)*) => {
        __for_chain! { @init ($($tt)* $head) $($tail)* }
    };

    (@expand {} for $val:tt in $expr:expr; $($tt:tt)+) => {
        for $val in $expr {
            __for_chain! { @expand {} $($tt)+ }
        }
    };

    (@expand { $($other:tt)+ } for $val:tt in $expr:expr; $($tt:tt)+) => {
        for $val in $expr {
            $($other)+;
            __for_chain! { @expand { $($other)+ } $($tt)+ }
        }
    };

    (@expand { $($other:tt)* } then { $($then:tt)* }) => {
        $($then)*
    };

    (@expand { $($other:tt)* } $stmt:stmt; $($tt:tt)+) => {
        {
            $stmt;
            __for_chain! { @expand { $($other)* } $($tt)+ }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_for_chain() {}
}
