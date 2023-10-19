enum Instruction {
  Addressing(String),
  Computing(String, String, String),
  Labeling(String),
}

trait string{
  fn to_string<T>(v:T) -> String;
}

impl string for Instruction {
  fn to_string(instruction: Instruction) {
    match instruction {
      Instruction::Computing(dest, cmp, jmp) => {
        let mut a = String::new();
        a.push_str(dest.clone());
        a.push_str(cmp.clone());
        a.push_str(jmp.clone());
        a
      }
      Instruction::Addressing(value) => value.clone(),
      Instruction::Labeling(value) => value.clone(),
    }
  }
}
