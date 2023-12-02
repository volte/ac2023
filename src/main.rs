mod aoc;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: aoc <day> <stage>");
        std::process::exit(1);
    }

    let day = args[1].parse::<u8>().expect("Invalid day");
    let stage = args[2].parse::<u8>().expect("Invalid stage");
    let input = std::fs::read_to_string(format!("inputs/day{}.txt", day)).unwrap();

    println!("{}", aoc::solve_problem(day, stage, &input));
}
