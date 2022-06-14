use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    time::Instant,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Jack VM Translator by Iquiji requires:\n\nvm_translator FilePath/FolderPath!");
        return Ok(());
    };

    let path = Path::new(&args[1]);

    // Check if path is folder then do folder mode!
    if path.is_dir() {
        let dir = path.read_dir()?;

        let base_path = path;
        let mut all_files_to_process = vec![];

        for file_path in dir {
            let file_path = file_path?.path();
            if file_path.extension().unwrap().to_str().unwrap() == "vm" {
                // only handle .vm files
                let temp = file_path.file_name().unwrap().to_str().unwrap();
                all_files_to_process.push(temp.to_string());
            }
        }

        // convert all files to asm
        let final_assembler_code = make_bootstrap_code()
            + &all_files_to_process
                .iter()
                .map(|file_name| {
                    file_to_asm_code(&base_path.join(file_name), &Instant::now()).unwrap()
                })
                .collect::<Vec<String>>()
                .join("\n\n//NEW FILE!\n\n");

        let out_file_name = path
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
            + ".asm";
        let out_file_path = base_path.join(out_file_name);

        let mut file = File::create(out_file_path)?;
        file.write_all(final_assembler_code.as_bytes())?;
    } else {
        let in_file_path = path;
        let out_file_path = path.with_extension("asm");

        let assembler_code = file_to_asm_code(in_file_path, &start_start)?;

        let mut file = File::create(out_file_path)?;
        file.write_all(assembler_code.as_bytes())?;
    }

    let duration = start_start.elapsed();
    println!("- Flush Assembler Code to File!: {:?}", duration);

    println!(
        "\nJack VM Translator Total Time Used: {:?}",
        start_start.elapsed()
    );

    Ok(())
}

fn make_bootstrap_code() -> String {
    // * Bootstrap code (should be written in assembly)
    // SP = 256
    // **call** Sys.init
    let out_buf: Vec<&str> = vec![
        // Start of by setting LCL ARG THIS THAT to -1
        "D=-1",
        "@LCL",
        "M=D",
        "@ARG",
        "M=D",
        "@THIS",
        "M=D",
        "@THAT",
        "M=D",
        // Set SP to 256
        "@256",
        "D=A",
        "@SP",
        "M=D",
        "",
        "@bootstrap",
        "D=A",
        "@SP",
        "A=M",
        "M=D", // *SP = D
        "",
        "@SP", // Inc SP
        "M=M+1",
        "\n",
        "@LCL",
        "D=M",
        "@SP",
        "A=M",
        "M=D", // *SP = D
        "",
        "@SP", // Inc SP
        "M=M+1",
        "\n",
        "@ARG",
        "D=M",
        "@SP",
        "A=M",
        "M=D", // *SP = D
        "",
        "@SP", // Inc SP
        "M=M+1",
        "\n",
        "@THIS",
        "D=M",
        "@SP",
        "A=M",
        "M=D", // *SP = D
        "",
        "@SP", // Inc SP
        "M=M+1",
        "\n",
        "@THAT",
        "D=M",
        "@SP",
        "A=M",
        "M=D", // *SP = D
        "",
        "@SP", // Inc SP
        "M=M+1",
        "\n",
        // ARG = SP -5
        "@SP",
        "D=M",
        "@5",
        "D=D-A",
        "@ARG",
        "M=D",
        // Set LCL = SP
        "@SP",
        "D=M",
        "@LCL",
        "M=D",
        // Finnaly jump to Sys.init
        "@Sys.init",
        "0; JMP",
        "(bootstrap)",
        "// End of Bootstrap\n\n\n",
    ];

    out_buf.join("\n")
}

fn file_to_asm_code(
    in_file_path: &Path,
    start: &Instant,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("+ Proccesing File: {:?}", in_file_path);

    let asm_file_string = fs::read_to_string(in_file_path)?;

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

    // println!("Instruction list:\n {:?}\n", vm_instr_list);

    let duration = start.elapsed();
    println!("- Read in vm file!: {:?}", duration);
    let start = Instant::now();

    let parsed_instructions: Vec<(usize, VMCommand)> = vm_instr_list
        .iter()
        .map(|str_instr| (str_instr.0, VMCommand::from_string(&str_instr.1)))
        .collect();

    // println!("Instruction list:\n {:?}\n", parsed_instructions);

    let duration = start.elapsed();
    println!("- Parse into Representation!: {:?}", duration);

    let core_name = Path::new(&in_file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let mut assembler_code: Vec<String> = vec![];
    let mut comp_label_counter = 0;
    let mut current_function = CurrentVMFunction {
        active_flag: false,
        name: "".to_string(),
        return_label_count: 0,
    };
    for instr in parsed_instructions {
        let instr_asm =
            instr
                .1
                .clone()
                .to_asm(core_name, &mut comp_label_counter, &mut current_function);
        assembler_code.push(instr_asm);
    }

    let assembler_code: String = assembler_code.join("\n");

    Ok(assembler_code)
}

#[derive(Debug, Clone)]
struct CurrentVMFunction {
    active_flag: bool,
    name: String,
    return_label_count: usize,
}

#[derive(Debug, Clone)]
enum VMCommandType {
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

#[derive(Debug, Clone)]
struct VMCommand {
    original: String,
    c_type: VMCommandType,
    arg1: String,
    arg2: u16,
}
impl VMCommand {
    fn from_string(string_instr: &str) -> Self {
        let split_input_string: Vec<&str> = string_instr.split_whitespace().collect();

        match split_input_string[0] {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Arithmetic,
                arg1: split_input_string[0].to_owned(),
                arg2: 6502,
            },
            "push" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Push,
                arg1: split_input_string[1].to_owned(),
                arg2: split_input_string[2].parse().unwrap(),
            },
            "pop" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Pop,
                arg1: split_input_string[1].to_owned(),
                arg2: split_input_string[2].parse().unwrap(),
            },
            "label" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Label,
                arg1: split_input_string[1].to_owned(),
                arg2: 6502,
            },
            "goto" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Goto,
                arg1: split_input_string[1].to_owned(),
                arg2: 6502,
            },
            "if-goto" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::IfGoto,
                arg1: split_input_string[1].to_owned(),
                arg2: 6502,
            },
            "function" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Function,
                arg1: split_input_string[1].to_owned(),
                arg2: split_input_string[2].parse().unwrap(),
            },
            "call" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Call,
                arg1: split_input_string[1].to_owned(),
                arg2: split_input_string[2].parse().unwrap(),
            },
            "return" => VMCommand {
                original: string_instr.to_owned(),
                c_type: VMCommandType::Return,
                arg1: "".to_owned(),
                arg2: 6502,
            },
            _ => unreachable!("Invalid Command in vm translator: {:?}", split_input_string),
        }
    }
    fn to_asm(
        &self,
        file_core_name: &str,
        comp_label_counter: &mut usize,
        current_function_def: &mut CurrentVMFunction,
    ) -> String {
        let mut buffer_string = format!("// {}\n", self.original);
        match self.c_type {
            VMCommandType::Arithmetic => {
                match self.arg1.as_str() {
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
                    }
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
                    }
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
                    }
                    "eq" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n", comp_label_counter);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n", comp_label_counter);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n", comp_label_counter);

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
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n", comp_label_counter);
                        buffer_string += "D; JEQ\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;

                        *comp_label_counter += 1;
                    }
                    "gt" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n", comp_label_counter);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n", comp_label_counter);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n", comp_label_counter);

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
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n", comp_label_counter);
                        buffer_string += "D; JGT\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;

                        *comp_label_counter += 1;
                    }
                    "lt" => {
                        let label_true = &format!("(_COMP_LABEL_{}_TRUE)\n", comp_label_counter);
                        let label_false = &format!("(_COMP_LABEL_{}_FALSE)\n", comp_label_counter);
                        let label_end = &format!("(_COMP_LABEL_{}_END)\n", comp_label_counter);

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
                        buffer_string += &format!("@_COMP_LABEL_{}_TRUE\n", comp_label_counter);
                        buffer_string += "D; JLT\n";
                        buffer_string += &format!("@_COMP_LABEL_{}_FALSE\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_true;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=-1\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += &format!("@_COMP_LABEL_{}_END\n", comp_label_counter);
                        buffer_string += "0; JMP\n";

                        buffer_string += label_false;

                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=0\n"; // Push Onto The Stack

                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n"; // Fix Stack Location Back

                        buffer_string += label_end;

                        *comp_label_counter += 1;
                    }
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
                    }
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
                    }
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
                    }
                    _ => unreachable!(),
                }
            }
            VMCommandType::Push => {
                match self.arg1.as_str() {
                    "local" | "argument" | "this" | "that" => {
                        // *this + arg2 => R13
                        // [R13] => [*this + arg2]
                        // *SP = [*this + arg2]
                        // *SP++

                        // Calculate Offset from specified Memory Region
                        buffer_string += &match self.arg1.as_str() {
                            "local" => "@LCL\n".to_owned(),
                            "argument" => "@ARG\n".to_owned(),
                            "this" => "@THIS\n".to_owned(),
                            "that" => "@THAT\n".to_owned(),
                            _ => unreachable!(),
                        };
                        // add offset
                        buffer_string += "D=M\n";
                        buffer_string += &format!("@{}\n", self.arg2);
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
                        buffer_string += &format!("@{}.{}\n", file_core_name, self.arg2);
                        buffer_string += "D=M\n"; // Get Data At static

                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                                                  // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    }
                    "temp" => {
                        buffer_string += "@5\n";
                        // add offset
                        buffer_string += "D=A\n";
                        buffer_string += &format!("@{}\n", self.arg2);
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
                    "constant" => {
                        // D = const
                        // *SP = D
                        // *SP = *SP + 1
                        buffer_string += &format!("@{}\n", self.arg2);
                        buffer_string += "D=A\n";
                        // Push To Stack
                        buffer_string += "@SP\n";
                        buffer_string += "A=M\n";
                        buffer_string += "M=D\n"; // D = *SP
                                                  // Inc SP
                        buffer_string += "@SP\n";
                        buffer_string += "M=M+1\n";
                    }
                    "pointer" => {
                        match self.arg2 {
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
                            }
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
                            }
                            _ => unreachable!("INVALID pointer number"),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            VMCommandType::Pop => {
                match self.arg1.as_str() {
                    "local" | "argument" | "this" | "that" => {
                        // Calculate Offset from specified Memory Region
                        buffer_string += &match self.arg1.as_str() {
                            "local" => "@LCL\n".to_owned(),
                            "argument" => "@ARG\n".to_owned(),
                            "this" => "@THIS\n".to_owned(),
                            "that" => "@THAT\n".to_owned(),
                            _ => unreachable!(),
                        };
                        // add offset
                        buffer_string += "D=M\n";
                        buffer_string += &format!("@{}\nD=D+A\n", self.arg2);
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
                        buffer_string += &format!("@{}.{}\n", file_core_name, self.arg2);
                        buffer_string += "M=D\n";
                    }
                    "temp" => {
                        buffer_string += "@5\n";
                        // add offset
                        buffer_string += "D=A\n";
                        buffer_string += &format!("@{}\nD=D+A\n", self.arg2);
                        buffer_string += "@R13\nM=D\n"; // save temp in R13
                                                        // Get Data From Stack
                        buffer_string += "@SP\n";
                        buffer_string += "M=M-1\n";
                        buffer_string += "A=M\n";
                        buffer_string += "D=M\n"; // D = *SP
                                                  // Set Data At Location Specified
                        buffer_string += "@R13\nA=M\nM=D\n";
                    }
                    "pointer" => {
                        match self.arg2 {
                            0 => {
                                // Get Data From Stack
                                buffer_string += "@SP\n";
                                buffer_string += "M=M-1\n";
                                buffer_string += "A=M\n";
                                buffer_string += "D=M\n"; // D = *SP
                                                          // Set Data At Location Specified
                                buffer_string += "@THIS\nM=D\n";
                            }
                            1 => {
                                // Get Data From Stack
                                buffer_string += "@SP\n";
                                buffer_string += "M=M-1\n";
                                buffer_string += "A=M\n";
                                buffer_string += "D=M\n"; // D = *SP
                                                          // Set Data At Location Specified
                                buffer_string += "@THAT\nM=D\n";
                            }
                            _ => unreachable!("INVALID pointer number"),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            VMCommandType::Label => {
                if !current_function_def.active_flag {
                    buffer_string += &format!("({})\n", self.arg1);
                } else {
                    buffer_string += &format!("({}${})\n", current_function_def.name, self.arg1);
                }
            }
            VMCommandType::Goto => {
                if !current_function_def.active_flag {
                    buffer_string += &format!("@{}\n", self.arg1);
                } else {
                    buffer_string += &format!("@{}${}\n", current_function_def.name, self.arg1);
                }
                buffer_string += "0; JMP\n";
            }
            VMCommandType::IfGoto => {
                // Get Data From Stack
                buffer_string += "@SP\n";
                buffer_string += "M=M-1\n";
                buffer_string += "A=M\n";
                buffer_string += "D=M\n"; // D = *SP
                if !current_function_def.active_flag {
                    buffer_string += &format!("@{}\n", self.arg1);
                } else {
                    buffer_string += &format!("@{}${}\n", current_function_def.name, self.arg1);
                }
                buffer_string += "D; JNE\n";
            }
            VMCommandType::Function => {
                *current_function_def = CurrentVMFunction {
                    active_flag: true,
                    name: self.arg1.clone(),
                    return_label_count: 0,
                };
                buffer_string += &format!("({})\n", current_function_def.name.clone());
                for _ in 0..self.arg2 {
                    // load 0 to push as local variable
                    buffer_string += "@0\n";
                    buffer_string += "D=A\n";
                    // Push To Stack
                    buffer_string += "@SP\n";
                    buffer_string += "A=M\n";
                    buffer_string += "M=D\n"; // D = *SP
                                              // Inc SP
                    buffer_string += "@SP\n";
                    buffer_string += "M=M+1\n";
                }
            }
            VMCommandType::Return => {
                // endFrame in @R13
                // retAddr in @R14

                // first save Local so we can get the return value in a second
                buffer_string += "@LCL\n";
                buffer_string += "D=M\n";
                buffer_string += "@R13\n";
                buffer_string += "M=D\n";

                // second get the return address to jump to at the end of the return statement
                // its in LCL-5
                // save in @R14
                buffer_string += "@R13\n";
                buffer_string += "D=M\n";
                buffer_string += "@5\n";
                buffer_string += "A=D-A\n";
                buffer_string += "D=M\n";
                buffer_string += "@R14\n";
                buffer_string += "M=D\n";

                // set the return value in Argument 0, top of the Stack for caller
                // *ARG = pop()
                // Get Data From Stack
                buffer_string += "@SP\n";
                buffer_string += "M=M-1\n";
                buffer_string += "A=M\n";
                buffer_string += "D=M\n"; // D = *SP
                                          // Set Data At ARG
                buffer_string += "@ARG\nA=M\nM=D\n";

                // Reset SP to ARG+1
                buffer_string += "@ARG\n";
                buffer_string += "D=M\n";
                buffer_string += "@SP\n";
                buffer_string += "M=D+1\n";

                // Restore Stack Frame of Caller
                for to_save in ["@THAT\n", "@THIS\n", "@ARG\n", "@LCL\n"]
                    .iter()
                    .enumerate()
                {
                    // *(endframe - n) // Get Value for Restoration
                    buffer_string += "@R13\n";
                    buffer_string += "A=M\n";
                    buffer_string += "D=A\n";
                    buffer_string += &format!("@{}\n", to_save.0 + 1);
                    buffer_string += "A=D-A\n";
                    buffer_string += "D=M\n";

                    // Restore
                    buffer_string += to_save.1;
                    // get value
                    buffer_string += "M=D\n";
                }

                // Go back to retAddr
                buffer_string += "@R14\n";
                buffer_string += "A=M\n";
                buffer_string += "0 ; JMP // Return from function\n";
            }
            VMCommandType::Call => {
                // Push Return address
                buffer_string += &format!(
                    "@{}$ret.{}\n",
                    current_function_def.name.clone(),
                    current_function_def.return_label_count
                );
                buffer_string += "D=A\n";
                // Push To Stack
                buffer_string += "@SP\n";
                buffer_string += "A=M\n";
                buffer_string += "M=D\n"; // D = *SP
                                          // Inc SP
                buffer_string += "@SP\n";
                buffer_string += "M=M+1\n";

                for to_save in ["@LCL\n", "@ARG\n", "@THIS\n", "@THAT\n"] {
                    buffer_string += to_save;
                    // get value
                    buffer_string += "D=M\n";
                    // Push To Stack
                    buffer_string += "@SP\n";
                    buffer_string += "A=M\n";
                    buffer_string += "M=D\n"; // D = *SP
                                              // Inc SP
                    buffer_string += "@SP\n";
                    buffer_string += "M=M+1\n";
                }

                // set new ARG Pointer to SP-5-nArgs
                buffer_string += "@SP\n";
                buffer_string += "D=M\n";
                // -5
                buffer_string += "@5\n";
                buffer_string += "D=D-A\n";
                // -nArgs
                buffer_string += &format!("@{}\n", self.arg2);
                buffer_string += "D=D-A\n";
                // set ARG
                buffer_string += "@ARG\n";
                buffer_string += "M=D\n";

                // set LCL to SP
                buffer_string += "@SP\n";
                buffer_string += "D=M\n";
                buffer_string += "@LCL\n";
                buffer_string += "M=D\n";

                // call futncion
                buffer_string += &format!("@{}\n", self.arg1);
                buffer_string += "0; JMP // Jump to Function\n";

                // return label
                buffer_string += &format!(
                    "({}$ret.{}) // return label\n",
                    current_function_def.name.clone(),
                    current_function_def.return_label_count
                );
                current_function_def.return_label_count += 1;
            }
        }
        if buffer_string.is_empty() {
            eprintln!("EMPTY!, {:?}", self);
        }
        buffer_string
    }
}
