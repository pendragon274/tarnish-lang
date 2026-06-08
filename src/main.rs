use {
    std::{
        env,
        io,
        io::BufRead,
        process::ExitCode},
    tarnishlang::TarnishCompiler};

/*
enum InterpretationResult{
    Ok(String),
    Incomplete,
    Err(String)
}

use InterpretationResult::*;
use crate::tarnish_compiler::TarnishCompiler;

fn convert_input_name_to_ll(name: &str) -> String {
    let found_delimiter = name.find(".");
    match found_delimiter {
        None => String::from(name) + ".ll",
        Some(index) => String::from(&name[0..index]) + ".ll"
    }
}

fn convert_ll_name_to_o(name: &str) -> String {
    let found_delimiter = name.find(".");
    match found_delimiter {
        None => String::from(name) + ".o",
        Some(index) => String::from(&name[0..index]) + ".o"
    }
}

fn convert_o_name_to_exe(name: &str) -> String {
    let found_delimiter = name.find(".");
    match found_delimiter {
        None => String::from(name) + ".out",
        Some(index) => String::from(&name[0..index])
    }
}

fn interpret_macro(symbols: &mut SymbolTable, carry: &mut Vec<Token>) -> InterpretationResult{
    println!("Interpreting macro with symbols: {:?} and carry: {:?}", symbols, carry);
    *carry = Vec::new();
    Ok("".to_string())
}

fn execute_macro(symbols: &mut SymbolTable, carry: &mut Vec<Token>) -> InterpretationResult{
    println!("Executing macro with symbols: {:?} and carry: {:?}", symbols, carry);
    *carry = Vec::new();
    Ok("".to_string())
}

fn interpret_line(line: &str, symbols: &mut SymbolTable, carry: &mut Vec<Token>) -> InterpretationResult {
    let trimmed = line.trim();

    if trimmed.len() == 0{
        return if carry.len() == 0 {
            Ok(String::from(""))
        } else {
            Incomplete
        };
    }

    if trimmed.starts_with("~"){
        return if carry.len() == 0 {
            println!("Current line is llvm: {}", trimmed);
            Ok(String::from(&trimmed[1..trimmed.len()]))
        } else {
            carry.push(Token::from(trimmed));
            Incomplete
        }
    }

    carry.append(&mut Token::separate(trimmed));
    println!("Carry turned into after tokens separated: {:?}", &carry);

    if carry[0].eq(&Token::from("macro")){
        interpret_macro(symbols, carry)
    }else{
        execute_macro(symbols, carry)
    }
}*/

fn main() -> ExitCode {
    let mut file_names: Vec<String> = env::args().skip(1).collect::<Vec<String>>();
    if file_names.len() < 1 {
        println!("Please enter a filename:");
        let stdin = io::stdin();
        let mut done = false;
        while !done{
            let file = stdin.lock().lines().next().unwrap().unwrap();
            if file != String::from("") {
                file_names.push(file);
            }else {
                done = true;
            }
        }
    }

    let mut compiler = TarnishCompiler::new(file_names);
    match compiler.compile(){
        Ok(_) => ExitCode::SUCCESS,
        Err(e) =>{
            println!("\nCompilation error:\n\n{}", e);
            ExitCode::from(e.code())
        }
    }

    /*let file_ll_name = convert_input_name_to_ll(&file_name);
    let file_o_name = convert_ll_name_to_o(&file_ll_name);
    let file_exe_name = convert_o_name_to_exe(&file_o_name);
    let file_contents = fs::read_to_string(&file_name).expect("Something went wrong reading the file");

    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut carry: Vec<Token> = Vec::new();
    let mut interpreted_lines: Vec<String> = Vec::new();
    let mut symbol_table: SymbolTable = SymbolTable::new();
    for (index, line) in lines.iter().enumerate() {
        let interpreted_line_result = interpret_line(line, &mut symbol_table, &mut carry);
        match interpreted_line_result {
            Ok(new_line) => {
                interpreted_lines.push(new_line);
                continue;
            },
            Incomplete =>{
                continue;
            },
            Err(e) => {
                println!("Error in line number {} with line:\n{}\nError description: {}", &index, &line, e);
                return;
            }
        }
    }

    fs::write(&file_ll_name, interpreted_lines.join("\n")).expect("Unable to write file");

    println!("LLVM file generated. Compiling to binary.");

    let llc_output = Command::new("llc").args(["-filetype=obj", &file_ll_name, "-o", &file_o_name]).output().expect("Failed to compile llvm code: ");
    if llc_output.stderr.len() > 0 {
        println!("LLC Error: {}", String::from_utf8(llc_output.stderr).unwrap());
        return;
    }

    //Command::new("rm").args([&file_ll_name]).output().expect("Failed to remove intermediate ll file: ");
    let clang_output = Command::new("clang").args([&file_o_name, "-o", &file_exe_name]).output().expect("Failed to link: ");
    if clang_output.stderr.len() > 0 {
        println!("CLANG Error: {}", String::from_utf8(clang_output.stderr).unwrap());
        return;
    }

    Command::new("rm").args([&file_o_name]).output().expect("Failed to remove intermediate linker file: ");

    println!("Successfully compiled to {}!", &file_exe_name);

    println!("Executing compiled program:\n");
    let final_return = Command::new(String::from("./") + &file_exe_name).output().expect("Program execution failed: ");
    println!("{}", &final_return.stdout.iter().map(|b| *b as char).collect::<String>());
    println!("Process finished with exit code: {:?}", &final_return.status.code().unwrap());*/
}
