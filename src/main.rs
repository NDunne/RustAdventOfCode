use std::process::exit;

mod runner;
mod solver;
mod solutions;
mod verify;

use runner::get_solution;

fn run_day(year : &str, day_number : u8)
{
    match get_solution(year, day_number)
    {
        Ok(solution)  => println!("Day {:02}: {}", day_number, solution),
        Err(e) => eprintln!("Day {:02} : {}", day_number, e)
    }
}

fn main() 
{
    let first_arg: Option<String> = std::env::args().nth(1);
    let year_arg: Option<String> = std::env::args().nth(2);
    
    let year: &str = year_arg.as_deref().unwrap_or("2025");

    if let Some(arg) = first_arg
    {
        if regex::Regex::new(r"^day\d\d?$").unwrap().is_match(&arg) {
            let selected_day = arg[3..].parse().unwrap();
            match selected_day {
                1..=25 => run_day(year, selected_day),
                _ => eprintln!("Invalid day, must be 1-25"),
            }
        }
        else
        {
            eprintln!("Format day argument as 'dayXX'");
            exit(1);
        }
    }
    else
    {
        for i in 1..26
        {
            run_day(year, i);
        }
    }
}
