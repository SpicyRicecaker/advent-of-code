use advent_of_code_2021::config;

// This macro basically takes in a module name as a parameter, and uses the run function of that module
macro_rules! get_run_func_from {
    ($x:ident) => {
        use advent_of_code_2021::days::$x::run;
        println!("------[{}]-----", stringify!($x));
    };
}

fn main() {
    let state = config();

    // Brings into scope the run function from that module
    get_run_func_from!(three_three);

    run(state);
}
