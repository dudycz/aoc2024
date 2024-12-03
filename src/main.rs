mod day01;
mod day02;
mod day03;

macro_rules! solve_and_print {
    ($($day:expr, $file:expr, $solve_fn:expr);*) => {
        $(
            match $solve_fn($file) {
                Ok(result) => println!("{} result: {:?}", $day, result),
                Err(e) => eprintln!("Failed to solve {}: {:?}", $day, e),
            }
        )*
    };
}

fn main() {
    solve_and_print!(
        "Day 01", "inputs/day01.txt", day01::solve;
        "Day 02", "inputs/day02.txt", day02::solve;
        "Day 03", "inputs/day03.txt", day03::solve
    );
}
