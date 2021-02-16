// Made by Connor Slade (Sigma#8214 on Discord) (https://github.com/Basicprogrammer10 on Github)
use std::env;
use std::process::exit;
use std::process::Command;

mod help;
//use help;

fn main() {
    let args: Vec<String> = env::args().collect();

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", color_print("                                         MASM32-Compile-Script                                        ", 45));
    if cfg!(unix) {
        println!("        {}", color_print("                               THIS IS FOR WINDOWS ONLY                               ", 41));
        exit(0);
    }

    parse_args(args);

    return;
}

fn compile_script(file_location: &str) {
    println!("        {}        ", color_print("                                      COMPILING                                       ", 44));
    println!("{}[35m", color_print("[*] Starting Compile", 36));
    let output = Command::new("C:\\masm32\\bin\\ml")
        .arg("/c")
        .arg("/Zd")
        .arg("/coff")
        .arg(file_location)
        .status()
        .expect("failed to execute process");

    if output.success() {
        println!("{}", color_print("[*] Compile Complete", 32));
    }else{
        println!("{}", color_print("[*] Compile Failed", 31));
        exit(0);
    }

    let new_file_loc: &str = &file_location[0..file_location.len() - 4];
    println!("{}[35m", color_print("[*] Starting Linking", 36));
    let output = Command::new("C:\\masm32\\bin\\link")
        .arg("/SUBSYSTEM:CONSOLE")
        .arg(format!("{}.obj", new_file_loc))
        .status()
        .expect("failed to execute process");

    if output.success() {
        println!("{}", color_print("[*] Compile Linking", 32));
    }else{
        println!("{}", color_print("[*] Linking Failed", 31));
        exit(0);
    }
}

fn run_script(file_location: &str){

}

fn parse_args(args: Vec<String>) {
    let args_len = args.len();

    if args_len <= 1 {
        help::show_help();
        exit(0);
    } else if args_len == 2 {
        compile_script(&*args[1]);
    }else if args_len == 3 {
        compile_script(&*args[1]);
        run_script(&*args[1]);
    }
}

fn color_print(text: &str, color_index: i32) -> String {
    let output = ["[", &color_index.to_string()[..], "m", text, "[0m"].join("");
    return output;
}
//TODO:Add Compiling / Linking
//TODO:Less Bad Arg Parsing???