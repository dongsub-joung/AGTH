mod listup_exe;
mod inputing;

use eframe::egui;

// @todo system procedure
fn main() -> eframe::Result {
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
    // let data_stream_bus= runtime 

    // 4. print some data(some seleting data) with GUI

    // 5. the lastest: runtime
    // https://github.com/emilk/egui
    // let app;
    // app.bulild()

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    runtime_channel: String,
    body_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            runtime_channel: "【Collab】Ronald McDonald Insanity 2023【15th Anniversary】".to_owned(),
            body_text: "王　政　復　古　の　大　号　令".to_owned(),
        }
    }
}

// https://doc.rust-lang.org/reference/runtime.html
// https://users.rust-lang.org/t/how-to-execute-any-string-as-source-code-in-runtime/55717/12
// deno (https://www.telerik.com/blogs/how-to-compile-rust-into-webassembly-run-in-deno)
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("My egui Application\n");
            ui.label(format!("runtime_channel: {}\n", self.runtime_channel));
            ui.heading(&self.body_text);
        });
    }
}

