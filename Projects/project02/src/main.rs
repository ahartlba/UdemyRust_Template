use colored::*; // farben im terminal
use regex::Regex; // regular expressions
use std::env; // für cmd-line befehle
use std::fs; // file systems

#[derive(Debug)]
struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String,
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv);
}

fn run(argc: usize, argv: Vec<String>) {
    // println!("Argc: {}", argc);
    // println!("Argv: {:?}", argv);

    let args = parse_args(argc, &argv);
    let data = read(&args);
    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(_) => std::process::exit(1),
    };
    write(&args, &replaced_data);

    println!("DONE!")
}

// HELP FUNCTIONS
fn parse_args(argc: usize, argv: &Vec<String>) -> Arguments {
    if argc != 5 {
        eprintln!("{} wrong number of arguments!", "Error".red());
        std::process::exit(1);
    }

    Arguments {
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4],
    }
}

fn read(args: &Arguments) -> String {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} Failed to read from file{}!",
                "Error".red(),
                &args.input_file
            );

            std::process::exit(1);
        }
    };
    return data;
}

fn replace(search_target: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(search_target)?;
    Ok(regex.replace_all(data, replacement).to_string())
}

fn write(args: &Arguments, data: &String) {
    match fs::write(&args.output_file, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} Failed to write to file {}!",
                "Error".red(),
                &args.input_file
            );
            std::process::exit(1);
        }
    }
}
