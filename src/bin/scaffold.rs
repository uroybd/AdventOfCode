use chrono::prelude::*;
use clap::Parser;
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const MODULE_TEMPLATE: &str = r###"// Advent of Code {{ year }} - Day {{ day }}


pub fn solution_day_{{ day }}_01(file_path: String) -> anyhow::Result<usize> {
    None
}

pub fn solution_day_{{ day }}_02(file_path: String) -> anyhow::Result<usize> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_{{ day }}_01() {
        let file_path: String = String::from("inputs/{{ year }}/day{{ day }}e.txt");
        let result = solution_day_{{ day }}_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_{{ day }}_02() {
        let file_path: String = String::from("inputs/{{ year }}/day{{ day }}e.txt");
        let result = solution_day_{{ day }}_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_day_{{ day }}_01() {
        let file_path: String = String::from("inputs/{{ year }}/day{{ day }}.txt");
        let result = solution_day_{{ day }}_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_{{ day }}_02() {
        let file_path: String = String::from("inputs/{{ year }}/day{{ day }}.txt");
        let result = solution_day_{{ day }}_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
"###;

fn is_year(s: &str) -> Result<u16, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let current_year = chrono::Utc::now().year() as u16;
    let year: u16 = s.parse()?;
    if year > 2014 && year <= current_year {
        Ok(year)
    } else {
        Err(format!("{} is not in the range 2015-{}", year, current_year).into())
    }
}

fn is_day(s: &str) -> Result<u16, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let day: u16 = s.parse()?;
    if day > 0 && day <= 25 {
        Ok(day)
    } else {
        Err(format!("{} is not in the range 1-25", day).into())
    }
}

#[derive(Parser, Debug, Serialize)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = is_year)]
    year: u16,
    #[arg(value_parser = is_day)]
    day: u16,
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let cli = Cli::parse();

    let day_padded = format!("{:02}", cli.day);

    let input_path = format!("inputs/{}/day{}.txt", cli.year, day_padded);
    let example_path = format!("inputs/{}/day{}e.txt", cli.year, day_padded);
    let module_path = format!("src/solutions/year{}/day{}.rs", cli.year, day_padded);

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    let context = tera::Context::from_serialize(cli).expect("Invalid context");
    let content =
        tera::Tera::one_off(&MODULE_TEMPLATE, &context, false).expect("Failed to compile template");

    match file.write_all(content.as_bytes()) {
        Ok(_) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }
}
