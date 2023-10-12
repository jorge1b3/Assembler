use std::env;
use std::fs::File;
use std::io::{self,BufRead,BufReader};

enum Instruction {
    AInstruction(String), // -> Addressing instruction
    CInstruction(String, String, String), // -> Computation instruction
    LInstruction(String), // -> Label instruction
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.unwrap()
            .trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with("//"));

    let instructions = lines.map(|line| parser(line));

    for instruction in instructions {
        match instruction {
            Instruction::AInstruction(a_instruction) => println!("A instruction: {}", a_instruction),
            Instruction::CInstruction(dest, comp, jump) => println!("C instruction: {} {} {}", dest, comp, jump),
            Instruction::LInstruction(l_instruction) => println!("L instruction: {}", l_instruction),
        }
    }

    // for line in reader.lines() {
    //     let line = line?;
    //     let line = line.trim();

    //     if !line.is_empty() && !line.starts_with("//") {
    //         println!("{}", line);
    //     }
    // }
    
    // // let lines = reader_and_cleaner(file_name)?;
    // // let instructions: Vec<Instruction> = parser(lines);

    // //let dest = [("null", [0,0,0]), ("M", [0,0,1]), ("D", )];
    // for instruction in instructions {
    //     match instruction {
    //         Instruction::AInstruction(a_instruction) => println!("A instruction: {}", a_instruction),
    //         Instruction::CInstruction(dest, comp, jump) => println!("C instruction: {} {} {}", dest, comp, jump),
    //         Instruction::LInstruction(l_instruction) => println!("L instruction: {}", l_instruction),
    //     }
    // }
    Ok(())
}

// fn reader_and_cleaner(file_name: &str) -> io::Result<Vec<String>> {
//     let file = File::open(file_name)?;
//     let reader = BufReader::new(file);

//     let mut lines: Vec<String> = Vec::new();

//     for line in reader.lines() {
//         let line = line?;
//         let line = line.trim();

//         if !line.is_empty() && !line.starts_with("//") {
//             line.to_string()
//         }
//     }

//     Ok(lines)
// }

fn parser(line: String) -> Instruction {
    if line.starts_with("@") {
        Instruction::AInstruction(String::from(line))
    } else if line.starts_with("(") && line.ends_with(")") {
        Instruction::LInstruction(String::from(line))
    } else {
        let mut copy: &str = & line.clone();
        let mut jump = "";
        let dest = match copy.contains("=") {
            true => {
                let mut parts = copy.split("=");
                let temp = parts.next().unwrap();
                copy = parts.next().unwrap();
                temp
            },
            false => "",
        };
        let comp = match copy.contains(";") {
            true => {
                let mut parts = copy.split(";");
                copy = parts.next().unwrap();
                jump = parts.next().unwrap();
                copy
            },
            false => copy
        };
        Instruction::CInstruction(String::from(dest), String::from(comp), String::from(jump))
    }
}