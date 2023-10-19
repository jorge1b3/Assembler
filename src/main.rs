use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

#[derive(Debug, Clone)]
enum Instruction {
    Addressing(String),
    Computing(String, String, String),
    Labeling(String),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Computing(dest, cmp, jmp) => {
                let mut a = String::new();
                a.push_str(dest.clone().as_str());
                a.push_str(cmp.clone().as_str());
                a.push_str(jmp.clone().as_str());
                write!(f, "{}", a)
            }
            Instruction::Addressing(value) => write!(f, "{}", value.clone()),
            Instruction::Labeling(value) => write!(f, "{}", value.clone()),
        }
    }
}

const VARIABLE_ADDRESS_START: u16 = 16;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    if !args[1].ends_with(".asm") {
        eprintln!("The file {} is not a supporter file", args[1]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    let file = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(parser)
        .collect();

    let mut num_instructions = 0;

    let label_table: HashMap<String, u16> = HashMap::from_iter(
        instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Labeling(_) => (instruction, num_instructions),
                _ => {
                    let temp = num_instructions;
                    num_instructions += 1;
                    (instruction, temp)
                }
            })
            .filter(|(instruction, _)| matches!(instruction, Instruction::Labeling(_)))
            .map(|(instruction, number)| (format!("{}", instruction), number))
            .collect::<Vec<(String, u16)>>(),
    );

    let mut symbol_table: HashMap<String, u16> = HashMap::new();

    let binary_instructions: Vec<String> = instructions
        .iter()
        .filter(|instruction| !matches!(instruction, Instruction::Labeling(_)))
        .map(|instruction: &Instruction| to_binary(instruction, &mut symbol_table, &label_table))
        .filter(|line| !line.is_empty())
        .collect();

    let output_file_name = file_name.replace("asm", "hack");
    let output_file = File::create(output_file_name)?;
    let mut writer = BufWriter::new(output_file);
    binary_instructions.iter().for_each(|line| {
        let _ = writer.write_all(line.as_bytes());
        let _ = writer.write_all("\n".as_bytes());
    });
    Ok(())
}

fn parser(line: String) -> Instruction {
    if line.starts_with('@') {
        Instruction::Addressing(String::from(line.trim_start_matches('@')))
    } else if line.starts_with('(') && line.ends_with(')') {
        Instruction::Labeling(String::from(
            line.trim_start_matches('(').trim_end_matches(')'),
        ))
    } else {
        let mut copy: &str = &line.clone();
        let mut jump = "";
        let dest = match copy.contains('=') {
            true => {
                let mut parts = copy.split('=');
                let temp = parts.next().unwrap();
                copy = parts.next().unwrap();
                temp
            }
            false => "",
        };
        let comp = match copy.contains(';') {
            true => {
                let mut parts = copy.split(';');
                copy = parts.next().unwrap();
                jump = parts.next().unwrap();
                copy
            }
            false => copy,
        };
        Instruction::Computing(String::from(dest), String::from(comp), String::from(jump))
    }
}

fn to_binary(
    instruction: &Instruction,
    symbol_table: &mut HashMap<String, u16>,
    label_table: &HashMap<String, u16>,
) -> String {
    let comp_table: HashMap<&str, &str> = HashMap::from([
        ("0", "0101010"),
        ("1", "0111111"),
        ("-1", "0111010"),
        ("D", "0001100"),
        ("A", "0110000"),
        ("!D", "0001101"),
        ("!A", "0110001"),
        ("-D", "0001111"),
        ("-A", "0110011"),
        ("D+1", "0011111"),
        ("A+1", "0110111"),
        ("D-1", "0001110"),
        ("A-1", "0110010"),
        ("D+A", "0000010"),
        ("D-A", "0010011"),
        ("A-D", "0000111"),
        ("D&A", "0000000"),
        ("D|A", "0010101"),
        ("M", "1110000"),
        ("!M", "1110001"),
        ("-M", "1110011"),
        ("M+1", "1110111"),
        ("M-1", "1110010"),
        ("D+M", "1000010"),
        ("D-M", "1010011"),
        ("M-D", "1000111"),
        ("D&M", "1000000"),
        ("D|M", "1010101"),
    ]);

    let dest_table: HashMap<&str, &str> = HashMap::from([
        ("", "000"),
        ("M", "001"),
        ("D", "010"),
        ("MD", "011"),
        ("A", "100"),
        ("AM", "101"),
        ("AD", "110"),
        ("AMD", "110"),
    ]);

    let jump_table: HashMap<&str, &str> = HashMap::from([
        ("", "000"),
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ]);

    let default_symbols_table: HashMap<&str, u16> = HashMap::from([
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
        ("R0", 0),
        ("R1", 1),
        ("R2", 2),
        ("R3", 3),
        ("R4", 4),
        ("R5", 5),
        ("R6", 6),
        ("R7", 7),
        ("R8", 8),
        ("R9", 9),
        ("R10", 10),
        ("R11", 11),
        ("R12", 12),
        ("R13", 13),
        ("R14", 14),
        ("R15", 15),
        ("SCREEN", 16384),
        ("KBD", 24576),
    ]);

    match instruction {
        Instruction::Addressing(name) => match name.parse::<u16>() {
            Ok(number) => {
                if number > 32768 {
                    let address = symbol_table.get(name.as_str()).cloned().unwrap_or_else(|| {
                        let new_address = VARIABLE_ADDRESS_START + symbol_table.len() as u16;
                        symbol_table.insert(name.to_string(), new_address);
                        new_address
                    });
                    format!("0{:015b}", address)
                } else {
                    format!("0{:015b}", number)
                }
            }
            _ => {
                let address = default_symbols_table
                    .get(name.as_str())
                    .or(label_table.get(name))
                    .or(symbol_table.get(name))
                    .cloned()
                    .unwrap_or_else(|| {
                        let new_address = VARIABLE_ADDRESS_START + symbol_table.len() as u16;
                        symbol_table.insert(name.to_string(), new_address);
                        new_address
                    });
                format!("0{:015b}", address)
            }
        },
        Instruction::Computing(dest, comp, jump) => {
            let comp_binary = comp_table.get(comp.as_str()).unwrap_or(&"0000000");
            let dest_binary = dest_table.get(dest.as_str()).unwrap_or(&"000");
            let jump_binary = jump_table.get(jump.as_str()).unwrap_or(&"000");
            format!("111{}{}{}", comp_binary, dest_binary, jump_binary)
        }
        _ => String::new(),
    }
}
