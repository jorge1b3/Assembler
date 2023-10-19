use std::fmt;
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