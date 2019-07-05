use cef::{create_browser_sync, BrowserSettings, Settings, WindowInfo};
use std::sync::Arc;
use std::time::Duration;

struct MyApp {}
impl cef::App for MyApp {}

fn main() {
    let args = std::env::args().collect();

    let app = Arc::new(MyApp {});

    if cef::execute_process(&args, &app) > 0 {
        // It was a worker process, so stop here
        return;
    }

    let mut settings = Settings::default();
    //    settings.log_severity = cef_log_severity_t_LOGSEVERITY_VERBOSE;
    settings.remote_debugging_port = Some(9876);

    cef::initialize(&args, settings, &app);

    println!("ready");

    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(2));

        let mut window_info = WindowInfo::default();
        window_info.width = 1280;
        window_info.height = 720;

        let mut browser_settings = BrowserSettings::default();
        browser_settings.windowless_frame_rate = 30; // TODO - not necessary here?

        create_browser_sync(window_info, "http://google.com", browser_settings);
        // TODO

        println!("waiting");
        std::thread::sleep(Duration::from_secs(600));

        println!("quit");
        cef::quit_message_loop();
        // TODO - this doesnt appear to be stopping the loop..
    });

    cef::run_message_loop();

    println!("shutting fown");

    cef::shutdown();
}
