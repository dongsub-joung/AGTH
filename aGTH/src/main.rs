mod listup_exe;
mod inputing;

use eframe::egui;

// @todo system procedure
fn main() -> eframe::Result {
    // 1. list runtimed .exe programs up as CLI 
    listup_exe::init_listup();

    // 2. select it (more show details up )
    let input_string= inputing::init_inputing();    
    listup_exe::init_selecting(input_string);

    // 3-a. (Windows) cargo install watchexec-cli -> watchexec -r cargo run
    // 3.b. deno compile --reload ./main.ts (node js runtime) 
    // .exe -> memory
    // 3-b. a program dll runtime(windows System Procedures call while runtime)
    // https://drmemory.org/page_drstrace.html
    // let data_stream_bus= runtime 
    
    // 4. print some data(some seleting data) with GUI
    // node js http server -> print html (native web app)
}

// Some doc
// https://www.codeproject.com/Articles/13323/Intercepting-WinAPI-calls
// https://users.rust-lang.org/t/how-to-execute-any-string-as-source-code-in-runtime/55717/12
// https://doc.rust-lang.org/reference/runtime.html
// deno (https://www.telerik.com/blogs/how-to-compile-rust-into-webassembly-run-in-deno)