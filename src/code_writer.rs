
use crate::parser::CommandType;

pub struct CodeWriter{
    label_id: usize,
}

impl CodeWriter {
    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: usize) -> Vec<String> {
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
                let end_label  = format!("EQ_END_{}", id);

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
                let end_label  = format!("GT_END_{}", id);

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
                let end_label  = format!("LT_END_{}", id);

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

        
}
