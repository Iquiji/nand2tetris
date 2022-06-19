mod parser_non_xml;
mod tokenizer;

use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
    time::Instant,
};

use crate::parser_non_xml::{Parser, CodeGenerator};
use crate::tokenizer::Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Jack Compiler by Iquiji requires:\n\njack_compiler FilePath/FolderPath!");
        return Ok(());
    };

    let path = Path::new(&args[1]);

    if path.is_dir() {
        let dir = path.read_dir()?;

        let base_path = path;
        let mut all_files_to_process = vec![];

        for file_path in dir {
            let file_path = file_path?.path();
            if file_path.is_dir(){
                continue;
            }
            if file_path.extension().unwrap().to_str().unwrap() == "jack" {
                // only handle .vm files
                let temp = file_path.file_name().unwrap().to_str().unwrap();
                all_files_to_process.push(temp.to_string());
            }
        }

        for file_name in all_files_to_process {
            println!(
                "\n+ Proccesing File: {:?}",
                &base_path.join(file_name.clone())
            );

            let out_file_path = base_path.join(&file_name).with_extension("T.xml");

            let read_in_file = read_to_string(&base_path.join(&file_name))?;

            let duration = start_start.elapsed();
            println!("- Read in vm file!: {:?}", duration);
            let start = Instant::now();

            let tokenizer = Tokenizer::from_string(read_in_file);

            let duration = start.elapsed();
            println!("- Tokenizing finished for file!: {:?}", duration);
            let start = Instant::now();

            let test_t_xml = tokenizer.to_xml_string();

            let mut file = File::create(out_file_path.clone())?;
            file.write_all(test_t_xml.as_bytes())?;

            let parsed_file = Parser::from_tokenizer(tokenizer).compileClass();

            let duration = start.elapsed();
            println!("- Parsing finished for file!: {:?}", duration);
            let start = Instant::now();

            let compiled_file = CodeGenerator::new().to_vm_code(parsed_file);

            let duration = start.elapsed();
            println!("- Compiling finished for file!: {:?}", duration);
            let start = Instant::now();

            let mut file = File::create(base_path.join(&file_name).with_extension("vm"))?;
            file.write_all(compiled_file.as_bytes())?;

            let duration = start.elapsed();
            println!("- Flush VM Code to File!: {:?}", duration);
        }
    } else {
        let in_file_path = path;
        let out_file_path = path.with_extension("T.xml");

        let read_in_file = read_to_string(in_file_path)?;

        let tokenizer = Tokenizer::from_string(read_in_file);

        let test_t_xml = tokenizer.to_xml_string();

        let mut file = File::create(out_file_path)?;
        file.write_all(test_t_xml.as_bytes())?;
    }

    println!(
        "\nJack Compiler Total Time Used: {:?}",
        start_start.elapsed()
    );

    Ok(())
}
