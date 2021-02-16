use crate::color_print;

pub fn show_help() {
    let file_name = std::env::
    current_exe().expect("Can't get the exec path").
        file_name().expect("Can't get the exec name").
        to_string_lossy().into_owned();

    println!("        {}        ",
             color_print("                                         HELP                                         ", 44));
    println!("{} {} {} {}",
             color_print("Usage:    ", 35),
             color_print(&*format!("{}", file_name), 36),
             color_print("[File.ASM]", 32),
             color_print("[AutoRun] [MASM Install]", 33));
    println!();
    println!("{} {}                               - Shows This Help Page",
             color_print("Example1: ", 35),
             color_print(&*format!("{}", file_name), 36));
    println!("{} {} {}                - Compiles with default MASM location",
             color_print("Example2: ", 35),
             color_print(&*format!("{}", file_name), 36),
             color_print("HelloWorld.asm", 32));
    println!("{} {} {} {}           - Auto run (true / false)",
             color_print("Example3: ", 35),
             color_print(&*format!("{}", file_name), 36),
             color_print("HelloWorld.asm", 32),
             color_print("true", 33));
    println!("{} {} {} {} - All Options",
             color_print("Example4: ", 35),
             color_print(&*format!("{}", file_name), 36),
             color_print("HelloWorld.asm", 32),
             color_print("true C:\\masm32", 33));
}
