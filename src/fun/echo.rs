#[macro_export]
/// Print space-separated values with newline
macro_rules! echo {
    ($($arg:tt)*) => ({
        echon!($($arg)*)
        println!("")
    });
}

#[macro_export]
/// Print space-separated values without newline
macro_rules! echon {
    ($head:expr $(, $tail:expr)*) => ({
        print!("{}", $head);
        $(print!(" {}", $tail);)*
    });
}
