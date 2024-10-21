mod listup_exe;
mod inputing;

// @todo system procedure
fn main() {
    // 1. list runtimed .exe programs up as CLI 
    listup_exe::init_listup();

    // 2. select it (more show details up )
    // https://docs.rs/procfs/latest/procfs/process/index.html#:~:text=If%20you%20have%20a%20process,running%20processes%20using%20all_processes()%20.
    let input_string= inputing::init_inputing();
    listup_exe::init_selecting(input_string);

    // 3. .exe -> memory
    // https://www.codeproject.com/Articles/13323/Intercepting-WinAPI-calls

    // 3-b. a program dll runtime(windows System Procedures call while runtime)
    // https://drmemory.org/page_drstrace.html

    // 4. decoding UTF-8
    
    // 5. print some data(some seleting data) with GUI

    // 5-b. the lastest: runtime
    // https://github.com/emilk/egui
    // let app;
    // app.bulild()
}


