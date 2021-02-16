// Made by Connor Slade (Sigma#8214 on Discord) (https://github.com/Basicprogrammer10 on Github)
use std::env;
use std::process::Command;
use std::process::exit;
use std::time::{Instant};

mod help;

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

fn compile_script(file_location: &str, masm_install: &str) {
    println!("        {}        ", color_print("                                      COMPILING                                       ", 44));
    println!("{}[35m", color_print("[*] Starting Compile", 36));
    let process_start_time = Instant::now();
    let output = Command::new(format!("{}\\bin\\ml", masm_install))
        .arg("/c")
        .arg("/Zd")
        .arg("/coff")
        .arg(file_location)
        .status()
        .expect("failed to execute process");

    if output.success() {
        println!("{} {} {}", color_print("[*] Compile Complete [", 32), color_print(&*format!("{} ms", process_start_time.elapsed().as_millis()), 36), color_print("]", 32));
    } else {
        println!("{}", color_print("[*] Compile Failed", 31));
        exit(0);
    }

    let new_file_loc: &str = &file_location[0..file_location.len() - 4];
    println!("{}[35m", color_print("[*] Starting Linking", 36));
    let process_start_time = Instant::now();
    let output = Command::new(format!("{}\\bin\\link", masm_install))
        .arg("/SUBSYSTEM:CONSOLE")
        .arg(format!("{}.obj", new_file_loc))
        .status()
        .expect("failed to execute process");

    if output.success() {
        println!("{} {} {}", color_print("[*] Compile Linking  [", 32), color_print(&*format!("{} ms", process_start_time.elapsed().as_millis()), 36), color_print("]", 32));
    } else {
        println!("{}", color_print("[*] Linking Failed", 31));
        exit(0);
    }
}

fn run_script(file_location: &str) {
    let new_file_loc: &str = &file_location[0..file_location.len() - 4];
    println!("{}[35m", color_print("[*] Running", 36));
    let process_start_time = Instant::now();
    let output = Command::new(format!("{}.exe", new_file_loc))
        .output()
        .expect("failed to execute process");


    let output_code = output.status.to_string();
    let format_output_code: &str = &output_code[11..output_code.len()];

    println!("{}", color_print("\n\r[*] Finished", 32));
    println!("{}  {}", color_print("[*] Runtime: ", 33), color_print(&*format!("{} ms", process_start_time.elapsed().as_millis()), 36));
    println!("{} {}", color_print("[*] Exit Code:", 33), color_print(format_output_code, 36));
}

fn parse_args(args: Vec<String>) {
    let args_len = args.len();

    if args_len <= 1 {
        help::show_help();
        exit(0);
    } else if args_len == 2 {
        compile_script(&*args[1], "C:\\masm32");
    } else if args_len == 3 {
        compile_script(&*args[1], "C:\\masm32");
        if args[2].to_lowercase() == "true" {
            run_script(&*args[1]);
        }
    } else if args_len == 4 {
        compile_script(&*args[1], &*args[3]);
        if args[2].to_lowercase() == "true" {
            run_script(&*args[1]);
        }
    }
}

fn color_print(text: &str, color_index: i32) -> String {
    let output = ["[", &color_index.to_string()[..], "m", text, "[0m"].join("");
    return output;
}