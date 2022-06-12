use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Hack Assembler by Iquiji requires:\n\nHack_Assembler InFilePath OutFilePath !");
        return Ok(());
    };

    let in_file_path = args[1].clone();
    let out_file_path = args[2].clone();

    let asm_file_string = fs::read_to_string(in_file_path)?;

    let instruction_list: Vec<(usize, String)> = asm_file_string
        .lines()
        .filter(|line| !(line.starts_with("//") || line.is_empty()))
        .map(|line| {
            if line.contains("//") {
                line.split_at(line.find("//").unwrap()).0.to_owned()
            } else {
                line.to_string()
            }
        })
        .map(|line| line.replace(' ', "").trim().to_owned())
        .enumerate()
        .collect();

    let duration = start_start.elapsed();
    println!("\n- Read in asm file!: {:?}", duration);
    let start = Instant::now();
    // println!("Instruction list:\n {:?}\n", instruction_list);

    // Step One: get All Symbols
    let mut line_number_reduce = -1;
    let mut line_number_reduce_list: Vec<(usize, i32)> = vec![(0,0)];
    let mut symbols_map: HashMap<String, usize> = instruction_list
        .iter()
        .filter(|instruction| instruction.1.starts_with('(') && instruction.1.ends_with(')'))
        .map(|instr| {
            line_number_reduce += 1;
            line_number_reduce_list.push((
                (instr.0 as i32 - line_number_reduce) as usize,
                line_number_reduce,
            ));
            (
                instr
                    .1
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .to_owned(),
                (instr.0 as i32 - line_number_reduce) as usize,
            )
        })
        .collect();

    let duration = start.elapsed();
    println!("- Get All Jump Symbols!: {:?}", duration);
    let start = Instant::now();

    // Add Screen and Keyboard constant symbols

    // SP 0 0x0000
    // LCL 1 0x0001
    // ARG 2 0x0002
    // THIS 3 0x0003
    // THAT 4 0x0004
    // R0-R15 0-15 0x0000-f
    // SCREEN 16384 0x4000
    // KBD 24576 0x6000
    symbols_map.insert("SP".to_owned(), 0);
    symbols_map.insert("LCL".to_owned(), 1);
    symbols_map.insert("ARG".to_owned(), 2);
    symbols_map.insert("THIS".to_owned(), 3);
    symbols_map.insert("THAT".to_owned(), 4);
    symbols_map.insert("R0".to_owned(), 0);
    symbols_map.insert("R1".to_owned(), 1);
    symbols_map.insert("R2".to_owned(), 2);
    symbols_map.insert("R3".to_owned(), 3);
    symbols_map.insert("R4".to_owned(), 4);
    symbols_map.insert("R5".to_owned(), 5);
    symbols_map.insert("R6".to_owned(), 6);
    symbols_map.insert("R7".to_owned(), 7);
    symbols_map.insert("R8".to_owned(), 8);
    symbols_map.insert("R9".to_owned(), 9);
    symbols_map.insert("R10".to_owned(), 10);
    symbols_map.insert("R11".to_owned(), 11);
    symbols_map.insert("R12".to_owned(), 12);
    symbols_map.insert("R13".to_owned(), 13);
    symbols_map.insert("R14".to_owned(), 14);
    symbols_map.insert("R15".to_owned(), 15);
    symbols_map.insert("SCREEN".to_owned(), 16384);
    symbols_map.insert("KBD".to_owned(), 24576);

    // Step Two Remove all Symbols from Instr List now
    let mut instruction_list: Vec<(usize, String)> = instruction_list
        .iter()
        .filter(|instruction| !(instruction.1.starts_with('(') && instruction.1.ends_with(')')))
        .cloned()
        .collect();

    let duration = start.elapsed();
    println!("- Remove All Jump Symbols!: {:?}", duration);
    let start = Instant::now();

    // Step Two: Fix all line numbers
    let mut current_reducer = 0;
    let mut current_reducer_idx = 0;
    for instr in instruction_list.iter_mut(){
        // println!("current_reducer_idx: {},line_number_reduce_list.len()-1: {},min: {}",current_reducer_idx,line_number_reduce_list.len()-1,(current_reducer_idx+1).min(line_number_reduce_list.len()-1));
        if instr.0 > line_number_reduce_list[(current_reducer_idx+1).min(line_number_reduce_list.len()-1)].0 && current_reducer_idx + 1 != line_number_reduce_list.len() {
            current_reducer_idx += 1;
            current_reducer = line_number_reduce_list[(current_reducer_idx).min(line_number_reduce_list.len()-1)].1;
        }
        instr.0 -= current_reducer as usize;
    }

    // for reduce_from in &line_number_reduce_list {
    //     &instruction_list
    //         .iter()
    //         .by_ref()
    //         .enumerate()
    //         .skip(reduce_from.0)
    //         .for_each(|instr| instruction_list[instr.0].0 -= 1);
    // }

    let duration = start.elapsed();
    println!(
        "- Fix Line Numbers after Taking out Jump Symbols!: {:?}",
        duration
    );
    let start = Instant::now();

    // Step Three: replace all symbols with their line counterparts
    let instruction_list_symbols_replaced: Vec<(usize, String)> = instruction_list
        .iter()
        .cloned()
        .map(|instr| {
            if instr.1.starts_with('@') {
                let at_part = instr.1.strip_prefix('@').unwrap();
                if let Some(replacement_line) = symbols_map.get(at_part) {
                    (instr.0, ("@".to_string() + &replacement_line.to_string()))
                } else {
                    (instr.0, instr.1.to_owned())
                }
            } else {
                (instr.0, instr.1)
            }
        })
        .collect();

    let duration = start.elapsed();
    println!("- Replace All Jump Symbols!: {:?}", duration);
    let start = Instant::now();

    // Step Four Replace all remaining @something with line numbers after the programm
    let mut free_location_data_segment = 15;
    let instruction_list_all_symbols_replaced: Vec<(usize, String)> =
        instruction_list_symbols_replaced
            .iter()
            .cloned()
            .map(|instr| {
                if instr.1.starts_with('@') {
                    let at_part = instr.1.strip_prefix('@').unwrap();
                    if let Some(already_alloced) = symbols_map.get(at_part) {
                        (instr.0, ("@".to_string() + &already_alloced.to_string()))
                    } else if at_part.parse::<usize>().is_err() {
                        free_location_data_segment += 1;
                        symbols_map.insert(at_part.to_string(), free_location_data_segment);
                        (
                            instr.0,
                            ("@".to_string() + &free_location_data_segment.to_string()),
                        )
                    } else {
                        (instr.0, instr.1)
                    }
                } else {
                    (instr.0, instr.1)
                }
            })
            .collect();

    let duration = start.elapsed();
    println!("- Replace All remaining Symbols!: {:?}", duration);
    let start = Instant::now();

    // Step Five convert all instructions to machine code and then flush to output file
    let machine_code_out: String = instruction_list_all_symbols_replaced
        .iter()
        .map(|instr| instruction_to_machina_code(&instr.1))
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = File::create(out_file_path)?;
    file.write_all(machine_code_out.as_bytes())?;

    let duration = start.elapsed();
    println!("- Flush Machine Code to File!: {:?}", duration);

    println!("\nHack Assembler Total Time Used: {:?}", start_start.elapsed());

    Ok(())
}
fn instruction_to_machina_code(instr: &str) -> String {
    let mut instr = instr;
    let mut machine_code_out = "".to_owned();
    if instr.starts_with('@') {
        // A-Instruction
        let a_instr_num = instr.strip_prefix('@').unwrap().parse::<usize>().unwrap();
        machine_code_out = format!("0{0:015b}", a_instr_num);
    } else {
        // C-Instruction
        // [X=]Y[;]
        // = may be after AMD

        // C level
        machine_code_out += "111";

        // Dest Part
        let mut dest_part = "".to_owned();

        if instr.contains('=') {
            let split = instr.split_once('=').unwrap();
            instr = split.1;
            let assign_part = split.0;

            dest_part += &format!(
                "{:01b}{:01b}{:01b}",
                assign_part.contains('A') as u8,
                assign_part.contains('D') as u8,
                assign_part.contains('M') as u8
            );
        } else {
            dest_part += "000";
        }

        // Jump Part

        let jump_part = if instr.contains(';') {
            let split = instr.split_once(';').unwrap();
            instr = split.0;
            let jump_part_to_match = split.1.trim();

            match jump_part_to_match {
                "JGT" => "001",
                "JEQ" => "010",
                "JGE" => "011",
                "JLT" => "100",
                "JNE" => "101",
                "JLE" => "110",
                "JMP" => "111",
                _ => panic!("Invalid Jump Segment"),
            }
        } else {
            "000"
        };

        // Computational Part and a Bit
        let comp_part_and_a_bit = match instr {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => panic!("invalid computational part"),
        };

        machine_code_out += comp_part_and_a_bit;
        machine_code_out += &dest_part;
        machine_code_out += jump_part;
    }

    machine_code_out
}
