use std::{time::Instant, env, fs::{self, File}, io::Write, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Jack VM Translator by Iquiji requires:\n\nHack_Assembler InFilePath OutFilePath !");
        return Ok(());
    };

    let in_file_path = args[1].clone();
    let out_file_path = args[2].clone();

    let asm_file_string = fs::read_to_string(in_file_path.clone())?;

    let vm_instr_list: Vec<(usize, String)> = asm_file_string
        .lines()
        .filter(|line| !(line.starts_with("//") || line.is_empty()))
        .map(|line| {
            if line.contains("//") {
                line.split_at(line.find("//").unwrap()).0.to_owned()
            } else {
                line.to_string()
            }
        })
        .map(|line| line.trim().to_owned())
        .enumerate()
        .collect();

    println!("Instruction list:\n {:?}\n", vm_instr_list);

    let duration = start_start.elapsed();
    println!("\n- Read in vm file!: {:?}", duration);
    let start = Instant::now();

    let parsed_instructions: Vec<(usize,VMCommand)> = vm_instr_list.iter().map(|str_instr| (str_instr.0,VMCommand::from_string(&str_instr.1))).collect();

    println!("Instruction list:\n {:?}\n", parsed_instructions);

    let duration = start.elapsed();
    println!("\n- Parse into Representation!: {:?}", duration);
    let start = Instant::now();

    let core_name = Path::new(&in_file_path).file_stem().unwrap().to_str().unwrap();
    
    let assembler_code: String = parsed_instructions
        .iter()
        .map(|instr| instr.1.clone().to_asm(core_name,instr.0))
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = File::create(out_file_path)?;
    file.write_all(assembler_code.as_bytes())?;

    let duration = start.elapsed();
    println!("- Flush Assembler Code to File!: {:?}", duration);

    println!("\nJack VM Translator Total Time Used: {:?}", start_start.elapsed());
    
    Ok(())
}

#[derive(Debug,Clone)]
enum VMCommandType{
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    IfGoto,
    Function,
    Return,
    Call,
}

#[derive(Debug,Clone)]
struct VMCommand{
    original: String,
    c_type: VMCommandType,
    arg1: String,
    arg2: u16,
}
impl VMCommand{
    fn from_string(string_instr: &str) -> Self{
        let split_input_string: Vec<&str> = string_instr.split_whitespace().collect();

        match split_input_string[0]{
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Arithmetic,
                    arg1: split_input_string[0].to_owned(),
                    arg2: 6502,
                }
            },
            "push" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Push,
                    arg1: split_input_string[1].to_owned(),
                    arg2: split_input_string[2].parse().unwrap(),
                }
            },
            "pop" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Pop,
                    arg1: split_input_string[1].to_owned(),
                    arg2: split_input_string[2].parse().unwrap(),
                }
            },
            "label" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Label,
                    arg1: split_input_string[1].to_owned(),
                    arg2: 6502,
                }
            },
            "goto" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Goto,
                    arg1: split_input_string[1].to_owned(),
                    arg2: 6502,
                }
            },
            "if-goto" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::IfGoto,
                    arg1: split_input_string[1].to_owned(),
                    arg2: 6502,
                }
            },
            "function" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Function,
                    arg1: split_input_string[1].to_owned(),
                    arg2: split_input_string[2].parse().unwrap(),
                }
            },
            "call" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Call,
                    arg1: split_input_string[1].to_owned(),
                    arg2: split_input_string[2].parse().unwrap(),
                }
            },
            "return" => {
                VMCommand{
                    original: string_instr.to_owned(),
                    c_type: VMCommandType::Return,
                    arg1: "".to_owned(),
                    arg2: 6502,
                }
            },
            _ => unreachable!("Invalid Command in vm translator: {:?}",split_input_string)
        }
    }
    fn to_asm(&self,file_core_name: &str,instr_number: usize) -> String{
        let mut buffer_string = format!("// {}\n",self.original);
        match self.c_type{
            VMCommandType::Arithmetic => {
                match self.arg1.as_str(){
                    "add" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=D+A\n"; // Now perform OP

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    "sub" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=A-D\n"; // Now perform OP
                        
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    "neg" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "D=-D\n"; // Now perform OP

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    "eq" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n",instr_number);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n",instr_number);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n",instr_number);

                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=A-D\n"; // Now perform OP

                        // jump if true
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n",instr_number);
                        buffer_string += "D; JEQ\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n",instr_number);;
                        buffer_string += "0; JMP\n";
                        
                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n",instr_number);;
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;
                    },
                    "gt" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n",instr_number);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n",instr_number);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n",instr_number);

                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=A-D\n"; // Now perform OP

                        // jump if true
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n",instr_number);
                        buffer_string += "D; JGT\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n",instr_number);;
                        buffer_string += "0; JMP\n";
                        
                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n",instr_number);;
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;
                    },
                    "lt" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n",instr_number);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n",instr_number);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n",instr_number);

                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=A-D\n"; // Now perform OP

                        // jump if true
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n",instr_number);
                        buffer_string += "D; JLT\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n",instr_number);;
                        buffer_string += "0; JMP\n";
                        
                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n",instr_number);;
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;
                    },
                    "and" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=D&A\n"; // Now perform OP

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    "or" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "A=M\n"; // A = *SP
                        
                        buffer_string += "D=D|A\n"; // Now perform OP

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    "not" => {
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP

                        buffer_string += "D=!D\n"; // Now perform OP

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // Push Onto The Stack

                        buffer_string += "@SP\n"; 
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back
                    },
                    _ => unreachable!(),
                }
            },
            VMCommandType::Push => {
                match self.arg1.as_str(){
                    "local" | "argument" | "this" | "that" => {
                        // *this + arg2 => R13
                        // [R13] => [*this + arg2]
                        // *SP = [*this + arg2]
                        // *SP++

                        // Calculate Offset from specified Memory Region
                        buffer_string += &match self.arg1.as_str(){
                            "local" => "@LCL\n".to_owned(),
                            "argument" => "@ARG\n".to_owned(),
                            "this" => "@THIS\n".to_owned(),
                            "that" => "@THAT\n".to_owned(),
                            _ => unreachable!(),
                        };
                        // add offset
                        buffer_string += "D=M\n";
                        buffer_string += &format!("@{}\n",self.arg2);
                        buffer_string += "A=D+A\n";
                        buffer_string += "D=M\n";

                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                        // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    }
                    "static" => {
                        buffer_string += &format!("@{}.{}\n",file_core_name,self.arg2);
                        buffer_string += "D=M\n"; // Get Data At static
                        
                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                        // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    },
                    "temp" => {
                        buffer_string += "@5\n";
                        // add offset
                        buffer_string += "D=A\n";
                        buffer_string += &format!("@{}\n",self.arg2);
                        buffer_string += "A=D+A\n";
                        buffer_string += "D=M\n";

                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                        // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    },
                    "constant" => {
                        // D = const
                        // *SP = D
                        // *SP = *SP + 1
                        buffer_string += &format!("@{}\n",self.arg2);
                        buffer_string += "D=A\n";
                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                        // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    },
                    "pointer" => {
                        match self.arg2{
                            0 => {
                                buffer_string += "@THIS\n";
                                // add offset
                                buffer_string += "D=M\n";
                                // Push To Stack
                                buffer_string += "@SP\n";
                                buffer_string += "A=M\n";
                                buffer_string += "M=D\n"; // D = *SP
                                // Inc SP
                                buffer_string += "@SP\n";
                                buffer_string += "M=M+1\n";
                            },
                            1 => {
                                buffer_string += "@THAT\n";
                                // add offset
                                buffer_string += "D=M\n";
                                // Push To Stack
                                buffer_string += "@SP\n";
                                buffer_string += "A=M\n";
                                buffer_string += "M=D\n"; // D = *SP
                                // Inc SP
                                buffer_string += "@SP\n";
                                buffer_string += "M=M+1\n";
                            },
                            _ => unreachable!("INVALID pointer number"),
                        }
                    },
                    _ => unreachable!()
                }
            },
            VMCommandType::Pop => {
                match self.arg1.as_str(){
                    "local" | "argument" | "this" | "that" => {
                        // Calculate Offset from specified Memory Region
                        buffer_string += &match self.arg1.as_str(){
                            "local" => "@LCL\n".to_owned(),
                            "argument" => "@ARG\n".to_owned(),
                            "this" => "@THIS\n".to_owned(),
                            "that" => "@THAT\n".to_owned(),
                            _ => unreachable!(),
                        };
                        // add offset
                        buffer_string += "D=M\n";
                        buffer_string += &format!("@{}\nD=D+A\n",self.arg2);
                        buffer_string += "@R13\nM=D\n"; // save temp in R13
                        // Get Data From Stack
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP
                        // Set Data At Location Specified
                        buffer_string += "@R13\nA=M\nM=D\n";
                    }
                    "static" => {
                        // Get Data From Stack
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP
                        // Set Data At Location Specified
                        buffer_string += &format!("@{}.{}\n",file_core_name,self.arg2);
                        buffer_string += "M=D\n";
                    },
                    "temp" => {
                        buffer_string += "@5\n";
                        // add offset
                        buffer_string += "D=A\n";
                        buffer_string += &format!("@{}\nD=D+A\n",self.arg2);
                        buffer_string += "@R13\nM=D\n"; // save temp in R13
                        // Get Data From Stack
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP
                        // Set Data At Location Specified
                        buffer_string += "@R13\nA=M\nM=D\n";
                    },
                    "pointer" => {
                        match self.arg2{
                            0 => {
                                // Get Data From Stack
                                buffer_string += "@SP\n";
                                buffer_string += "M=M-1\n";
                                buffer_string += "A=M\n";
                                buffer_string += "D=M\n"; // D = *SP
                                // Set Data At Location Specified
                                buffer_string += "@THIS\nM=D\n";
                            },
                            1 => {
                                // Get Data From Stack
                                buffer_string += "@SP\n";
                                buffer_string += "M=M-1\n";
                                buffer_string += "A=M\n";
                                buffer_string += "D=M\n"; // D = *SP
                                // Set Data At Location Specified
                                buffer_string += "@THAT\nM=D\n";
                            },
                            _ => unreachable!("INVALID pointer number"),
                        }
                    },
                    _ => unreachable!()
                }
            },
            VMCommandType::Label => todo!(),
            VMCommandType::Goto => todo!(),
            VMCommandType::IfGoto => todo!(),
            VMCommandType::Function => todo!(),
            VMCommandType::Return => todo!(),
            VMCommandType::Call => todo!(),
        }
        if buffer_string.is_empty(){
            eprintln!("EMPTY!, {:?}",self);
        }
        buffer_string
    }
}