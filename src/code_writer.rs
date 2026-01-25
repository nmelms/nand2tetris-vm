use crate::parser::CommandType;

pub struct CodeWriter {
    pub label_id: usize,
    pub file_name: String,
}

impl CodeWriter {
    pub fn write_push_pop(
        &mut self,
        command: CommandType,
        segment: &str,
        index: usize,
    ) -> Vec<String> {
        match command {
            CommandType::PUSH => {
                println!("pushing");
                self.write_push(segment, index)
            }
            CommandType::POP => {
                println!("popping");
                self.write_pop(segment, index)
            }
            _ => vec!["nothing".to_string()],
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) -> Vec<String> {
        println!("write arth: {}", command);
        let mut instructions: Vec<String> = vec![];
        match command {
            "add" => {
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=D+M".to_string());
                instructions.push("M=D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M-1".to_string());
            }
            "sub" => {
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("M=M-D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M-1".to_string());
            }
            "eq" => {
                let id = self.label_id;
                self.label_id += 1;

                let true_label = format!("EQ_TRUE_{}", id);
                let end_label = format!("EQ_END_{}", id);

                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=M-D".to_string());
                instructions.push(format!("@{}", true_label));
                instructions.push("D;JEQ".to_string());

                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=0".to_string());
                instructions.push(format!("@{}", end_label));
                instructions.push("0;JMP".to_string());

                instructions.push(format!("({})", true_label));
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=-1".to_string());

                instructions.push(format!("({})", end_label));
            }
            "neg" => {
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=-M".to_string());
            }
            "gt" => {
                let id = self.label_id;
                self.label_id += 1;

                let true_label = format!("GT_TRUE_{}", id);
                let end_label = format!("GT_END_{}", id);

                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=M-D".to_string());
                instructions.push(format!("@{}", true_label));
                instructions.push("D;JGT".to_string());

                // false
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=0".to_string());
                instructions.push(format!("@{}", end_label));
                instructions.push("0;JMP".to_string());

                // true
                instructions.push(format!("({})", true_label));
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=-1".to_string());

                // end
                instructions.push(format!("({})", end_label));
            }
            "lt" => {
                let id = self.label_id;
                self.label_id += 1;

                let true_label = format!("LT_TRUE_{}", id);
                let end_label = format!("LT_END_{}", id);

                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=M-D".to_string());
                instructions.push(format!("@{}", true_label));
                instructions.push("D;JLT".to_string());

                // false
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=0".to_string());
                instructions.push(format!("@{}", end_label));
                instructions.push("0;JMP".to_string());

                // true
                instructions.push(format!("({})", true_label));
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=-1".to_string());

                // end
                instructions.push(format!("({})", end_label));
            }
            "or" => {
                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=D|M".to_string());
                instructions.push("M=D".to_string());
            }
            "and" => {
                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("A=A-1".to_string());
                instructions.push("D=D&M".to_string());
                instructions.push("M=D".to_string());
            }
            "not" => {
                instructions.push("@SP".to_string());
                instructions.push("A=M-1".to_string());
                instructions.push("M=!M".to_string());
            }

            _ => println!("invalid command. No match in write arithmetic"),
        }

        instructions
    }

    pub fn write_push(&mut self, segment: &str, index: usize) -> Vec<String> {
        // segment
        // index
        // stack
        let mut instructions: Vec<String> = vec![];

        match segment {
            "local" => {
                instructions.push("@LCL".to_string());
                instructions.push("D=M".to_string());
            }
            "argument" => {
                instructions.push("@ARG".to_string());
                instructions.push("D=M".to_string());
            }
            "this" => {
                instructions.push("@THIS".to_string());
                instructions.push("D=M".to_string());
            }
            "that" => {
                instructions.push("@THAT".to_string());
                instructions.push("D=M".to_string());
            }
            "temp" => {
                let mut temp_idx: usize = 5;
                temp_idx += index;
                instructions.push(format!("@{}", temp_idx));
                instructions.push("D=M".to_string());

                // push push instrucions
                instructions.push("@SP".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M+1".to_string());
                return instructions;
            }
            "constant" => {
                instructions.push(format!("@{}", index));
                instructions.push("D=A".to_string());

                // push push instrucions
                instructions.push("@SP".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M+1".to_string());
                return instructions;
            }
            "pointer" => {
                if index == 0 {
                    instructions.push("@THIS".to_string());
                } else if index == 1 {
                    instructions.push("@THAT".to_string());
                } else {
                    panic!("pointer index must be 0 or 1");
                }
                instructions.push("D=M".to_string());
                instructions.push("@SP".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M+1".to_string());
                return instructions;
            }
            "static" => {
                instructions.push(format!("@{}.{}", self.file_name, index));
                instructions.push("D=M".to_string());

                // push push instrucions
                instructions.push("@SP".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());
                instructions.push("@SP".to_string());
                instructions.push("M=M+1".to_string());
                return instructions;
            }

            _ => panic!("Invalid Segment: {} Segment did not match", segment),
        };

        // push index
        instructions.push(format!("@{}", index));
        instructions.push("A=D+A".to_string());
        instructions.push("D=M".to_string());

        // push push instrucions
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());

        instructions
    }

    pub fn write_pop(&mut self, segment: &str, index: usize) -> Vec<String> {
        // get address
        //
        let mut instructions: Vec<String> = vec![];

        match segment {
            "local" => {
                instructions.push("@LCL".to_string());
                instructions.push("D=M".to_string());
            }
            "argument" => {
                instructions.push("@ARG".to_string());
                instructions.push("D=M".to_string());
            }
            "this" => {
                instructions.push("@THIS".to_string());
                instructions.push("D=M".to_string());
            }
            "that" => {
                instructions.push("@THAT".to_string());
                instructions.push("D=M".to_string());
            }
            "temp" => {
                let temp_idx = 5 + index;
                instructions.push(format!("@{}", temp_idx));
                instructions.push("D=A".to_string());

                // Save address in R13
                instructions.push("@R13".to_string());
                instructions.push("M=D".to_string());

                // pop stack
                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push("@R13".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());

                return instructions;
            }

            "R13" => {
                instructions.push("@R13".to_string());
                instructions.push("D=M".to_string());
            }
            "R14" => {
                instructions.push("@R14".to_string());
                instructions.push("D=M".to_string());
            }
            "R15" => {
                instructions.push("@R15".to_string());
                instructions.push("D=M".to_string());
            }
            "pointer" => {
                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                if index == 0 {
                    instructions.push("@THIS".to_string());
                } else if index == 1 {
                    instructions.push("@THAT".to_string());
                } else {
                    panic!("pointer index must be 0 or 1");
                }
                instructions.push("M=D".to_string());
                return instructions;
            }
            "static" => {
                instructions.push("@SP".to_string());
                instructions.push("AM=M-1".to_string());
                instructions.push("D=M".to_string());
                instructions.push(format!("@{}.{}", self.file_name, index));
                instructions.push("M=D".to_string());
                return instructions;
            }

            _ => println!("Invalid Segment. Segment did not match"),
        };

        // Save address in R13
        instructions.push(format!("@{}", index));
        instructions.push("D=D+A".to_string());
        instructions.push("@R13".to_string());
        instructions.push("M=D".to_string());

        instructions.push("@SP".to_string());
        instructions.push("AM=M-1".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@R13".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());

        instructions
    }

    pub fn write_label(&self, label: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];

        instructions.push(format!("({})", label));
        return instructions;
    }

    pub fn write_goto(&self, label: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];

        instructions.push(format!("@{}", label));
        instructions.push("0;JMP".to_string());
        return instructions;
    }

    pub fn write_if(&self, label: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];

        // instructions.push(format!("@{}", label));
        instructions.push("@SP".to_string());
        instructions.push("AM=M-1".to_string());
        instructions.push("D=M".to_string());
        instructions.push(format!("@{}", label));
        instructions.push("D;JNE".to_string());

        return instructions;
    }

    pub fn write_funciton(&mut self, fn_name: &str, n_vars: usize) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];

        //push fn name
        instructions.push(format!("({})", fn_name));

        //loop nvars and push 0 to stack
        for _i in 0..n_vars {
            let mut str = self.write_push("constant", 0);
            instructions.append(&mut str);
        }

        instructions
    }

    pub fn write_call(&mut self, fn_name: &str, n_vars: usize) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];
        let id = self.label_id;
        self.label_id += 1;
        // might need to udate this to use the calle

        let label = format!("{}$ret.{}", fn_name, id);
        //genereate label and push to stack
        instructions.push(format!("@{}", label));
        instructions.push("D=A".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());

        // instructions.push(format!("({}$ret.{})", fn_name, self.label_id ));
        //push LCL
        instructions.push("@LCL".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());
        //push arg
        instructions.push("@ARG".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());
        //push THIS
        instructions.push("@THIS".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());
        //push THAT
        instructions.push("@THAT".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());
        //REposition ARG
        instructions.push("@SP".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@5".to_string());
        instructions.push("D=D-A".to_string());
        instructions.push(format!("@{}", n_vars));
        instructions.push("D=D-A".to_string());
        instructions.push("@ARG".to_string());
        instructions.push("M=D".to_string());
        //REposition LCL
        instructions.push("@SP".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@LCL".to_string());
        instructions.push("M=D".to_string());

        //goto fn
        instructions.push(format!("@{}", fn_name));
        instructions.push("0;JMP".to_string());

        instructions.push(format!("({})", label));

        instructions
    }
}
