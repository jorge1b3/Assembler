use std::env;
use std::fs::File;
use std::io::{self,BufRead,BufReader};
use std::collections::HashMap;

enum Instruction {
    AInstruction(String), // -> Addressing instruction
    CInstruction(String, String, String), // -> Computation instruction
    LInstruction(String), // -> Label instruction
}

const VAR_START: u16 = 16;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut label_table: HashMap<String, u16> = HashMap::new();

    let instructions_copy: Vec<Instruction> =  reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with("//")).map(|line| parser(line)).collect();
    // First pass to add labels to the symbol table
    let mut instruction_count = 0;
    for instruction in instructions_copy {
        match instruction {
            Instruction::LInstruction(value) => {
                label_table.insert(value.clone(), instruction_count);
                continue;
            },
            _ => instruction_count += 1,
        }
    }
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut symbol_table: HashMap<String, u16> = HashMap::new();

    // For each line, trim the line, remove empty lines and comments
    let lines = reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with("//"));

    let instructions = lines.map(|line| parser(line));

    
    let mut binary_instructions: Vec<String> = Vec::new();

    
    
    for instruction in instructions {
        match instruction {
            Instruction::LInstruction(_) => {
                continue;
            },
            _ => {},
        }
        let binary_instruction = to_binary(&instruction, &mut symbol_table, &mut label_table);
        if binary_instruction.is_empty() {
            continue;
        }
        binary_instructions.push(binary_instruction);
    }
    
    // Print the binary instructions
    for binary in &binary_instructions {
        if !binary.is_empty() {
            println!("{}", binary);
        }
    }

    Ok(())
}



fn parser(line: String) -> Instruction {
    if line.starts_with("@") {
        Instruction::AInstruction(String::from(line))
    } else if line.starts_with("(") && line.ends_with(")") {
        let variable_name = line.trim_start_matches("(").trim_end_matches(")");
        Instruction::LInstruction(String::from(variable_name))
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

fn to_binary(instruction: &Instruction, symbol_table: &mut HashMap<String, u16>, label_table: &mut HashMap<String, u16>) -> String {
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

    match instruction {
        Instruction::AInstruction(value) => {
            let value = value.trim_start_matches('@');
            if let Ok(num) = value.parse::<u16>() {
                // A-instruction with a numeric value
                format!("0{:015b}", num)
            } else {
                if label_table.contains_key(value){
                    format!("0{:015b}", label_table[value])
                }
                // A-instruction with a symbol
                else if symbol_table.contains_key(value) {
                    format!("0{:015b}", symbol_table[value])
                } else {
                    let address = VAR_START + symbol_table.len() as u16;
                    symbol_table.insert(value.to_string(), address);
                    format!("0{:015b}", address)
                }
            }
        }
        Instruction::CInstruction(dest, comp, jump) => {
            let comp_binary = match comp_table.get::<str>(comp.as_str()) {
                Some(comp_bin) => comp_bin,
                None => "0000000",
            };
            let dest_binary = dest_table.get::<str>(dest.as_str()).unwrap_or(&"000");
            let jump_binary = jump_table.get::<str>(jump.as_str()).unwrap_or(&"000");
            format!("111{}{}{}", comp_binary, dest_binary, jump_binary)
        }
        Instruction::LInstruction(_) => {
            "".to_string()
        }, // Labels have no binary representation
    }
}
