use cef::Settings;
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

    let mut settings: Settings = Default::default();
    //    settings.log_severity = cef_log_severity_t_LOGSEVERITY_VERBOSE;
    settings.remote_debugging_port = Some(9876);

    cef::initialize(&args, settings, &app);

    println!("test");

    //    std::thread::sleep(Duration::from_secs(6000));

    cef::shutdown();
}
