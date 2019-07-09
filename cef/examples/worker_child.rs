use std::sync::Arc;

struct MyApp {}
impl cef::App for MyApp {
    type OutBrowserProcessHandler = ();
}

// This example will never be run directly, but is the child subprocess of the worker example
fn main() {
    let app = Arc::new(MyApp {});

    std::process::exit(cef::execute_process(&app))
}
