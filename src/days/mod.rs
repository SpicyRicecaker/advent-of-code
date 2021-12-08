// This macro basically takes in a module name as a parameter, and auto brings them all into scope as a public module
macro_rules! mod_days {
    ($($x:ident),*) => {
        $(
            pub mod $x;
        )*
    };
}

mod_days!(one, one_two, two, two_two, three, three_three, four, four_two, five, five_two, six, six_two, seven, seven_two);