use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} FROM TO FILE", args[0]);
        eprintln!("Example: {} even_odd_python -20000 20000", args[0]);
        std::process::exit(1);
    }

    let first_number = match args[2].parse::<i64>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("ERROR! -> Invalid value for FROM");
            std::process::exit(1);
        }
    };

    let last_number = match args[3].parse::<i64>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("ERROR! -> Invalid value for TO");
            std::process::exit(1);
        }
    };

    let file_name = &args[1];

    let file = match File::create(format!("{}.py", file_name)) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("ERROR! -> Failed to create file");
            std::process::exit(1);
        }
    };

    // FIRST LINES FOR PYTHON CODE
    // INPUT AND CODE SECURITY
    writeln!(&file, "while True:").expect("Error writing to file");
    writeln!(&file, "   try:").expect("Error writing to file");
    writeln!(&file, "      input_number = int(input('Enter a number to check: '))").expect("Error writing to file");
    writeln!(&file, "   except ValueError:").expect("Error writing to file");
    writeln!(&file, "      print('\\tERROR! -> WRONG INPUT. ENTER AGAIN\\n')").expect("Error writing to file");
    writeln!(&file, "");  // empty line

    for i in first_number..=last_number {
        if i == first_number {
            writeln!(&file, "   if input_number == {}:", &i)
                .expect("Error writing to file");

            if i % 2 == 0 {
                writeln!(&file, "       print('Even')")
                    .expect("Error writing to file");
            } else {
                writeln!(&file, "       print('Odd')")
                    .expect("Error writing to file");
            }

            writeln!(&file, "")
                .expect("Error writing to file");
        } else {
            writeln!(&file, "   elif input_number == {}:", &i)
                .expect("Error writing to file");

            if i % 2 == 0 {
                writeln!(&file, "       print('Even')")
                    .expect("Error writing to file");
            } else {
                writeln!(&file, "       print('Odd')")
                    .expect("Error writing to file");
            }

            writeln!(&file, "") // empty line
                .expect("Error writing to file");
        }
    }

    writeln!(&file, "   else:")
        .expect("Error writing to file");
    writeln!(&file, "       print('This number is out of range')")
        .expect("Error writing to file");
}
