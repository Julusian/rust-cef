use cef::Settings;
use cef_sys::cef_log_severity_t_LOGSEVERITY_VERBOSE;
use std::time::Duration;

fn main() {
    let args = std::env::args().collect();

    if cef::execute_process(&args) > 0 {
        // It was a worker process, so stop here
        return;
    }

    let mut settings: Settings = Default::default();
    //    settings.log_severity = cef_log_severity_t_LOGSEVERITY_VERBOSE;
    settings.remote_debugging_port = Some(9876);
    cef::initialize(&args, settings);

    println!("test");

    //    std::thread::sleep(Duration::from_secs(6000));

    cef::shutdown();
}
