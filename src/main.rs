use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Sposób użycia: {} FROM TO FILE", args[0]);
        eprintln!("Sposób użycia: {} even_odd_python -20000 20000", args[0]);
        std::process::exit(1);
    }

    let firstNumber = &args[2].parse::<i64>();
    let lastNumber = &args[3].parse::<i64>();
    let fileName = &args[1].parse::<String>();

    match firstNumber {
        Ok(_) => println!("Correctly stated value FROM"),
        Err(_) => {
            eprintln!("Uncorrectly stated value FROM");
            std::process::exit(1);
        },
    }

    match lastNumber {
        Ok(_) => println!("Correctly stated value TO"),
        Err(_) => {
            eprintln!("Uncorrectly stated value TO");
            std::process::exit(1);
        },
    }
    
    match fileName {
        Ok(_) => println!("Correctly stated value NAME"),
        Err(_) => {
            eprintln!("Uncorrectly stated value NAME");
            std::process::exit(1);
        },
    }
}
