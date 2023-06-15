use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let writing_error: String = "ERROR! -> Some think is wrong with writing the file".to_string();
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
    writeln!(&file, "while True:").expect(&writing_error);
    writeln!(&file, "   try:").expect(&writing_error);
    writeln!(&file, "      input_number = int(input('Enter a number to check: '))").expect(&writing_error);
    writeln!(&file, "   except:").expect(&writing_error);
    writeln!(&file, "      print('ERROR! -> WRONG INPUT. ENTER AGAIN')").expect(&writing_error);

    for i in first_number..=last_number {
        if i == first_number {
            writeln!(&file, "   if input_number == {}:", &i)
                .expect(&writing_error);

            if i % 2 == 0 {
                writeln!(&file, "       print('Even')")
                    .expect(&writing_error);
            } else {
                writeln!(&file, "       print('Odd')")
                    .expect(&writing_error);
            }

            writeln!(&file, "")
                .expect(&writing_error);
        } else {
            writeln!(&file, "   elif input_number == {}:", &i)
                .expect(&writing_error);

            if i % 2 == 0 {
                writeln!(&file, "       print('Even')")
                    .expect(&writing_error);
            } else {
                writeln!(&file, "       print('Odd')")
                    .expect(&writing_error);
            }
        }
    }

    writeln!(&file, "   else:")
        .expect(&writing_error);
    writeln!(&file, "       print('This number is out of range')")
        .expect(&writing_error);

    println!("THE CREATION OF A {}.py WAS SUCCESSFUL", &args[1]);
    println!("You should find {}.py in peog directory", &args[1]);
    println!("ENJOY!!!");
}