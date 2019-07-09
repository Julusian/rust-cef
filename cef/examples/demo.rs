#![feature(async_await)]
#![feature(type_alias_enum_variants)]

use cef::{
    create_browser_sync, Browser, BrowserSettings, CefRect, PaintElementType, RenderHandler,
    Settings, WindowInfo,
};
use std::sync::Arc;
use std::time::Duration;

struct MyApp {}
impl cef::App for MyApp {
    type OutBrowserProcessHandler = ();
}

struct MyRenderHandler {}
impl RenderHandler for MyRenderHandler {
    fn get_root_screen_rect(&self, _browser: &Browser) -> Option<CefRect> {
        None
    }

    fn get_view_rect(&self, _browser: &Browser) -> CefRect {
        CefRect {
            x: 0,
            y: 0,
            width: 800,
            height: 800,
        }
    }

    fn on_paint(
        &self,
        _browser: &Browser,
        _type_: PaintElementType,
        _dirty_rects: &[CefRect],
        _bytes: &[u8],
        _width: i32,
        _height: i32,
    ) {
        println!("Got frame");
    }
}

struct MyClient {
    render_handler: Arc<MyRenderHandler>,
}
impl cef::Client for MyClient {
    type OutAudioHandler = ();
    type OutDisplayHandler = ();
    type OutLifeSpanHandler = ();
    type OutRenderHandler = MyRenderHandler;

    fn get_render_handler(&self) -> Option<Arc<Self::OutRenderHandler>> {
        Some(self.render_handler.clone())
    }
}

fn main() {
    let app = Arc::new(MyApp {});

    {
        let result_code = cef::execute_process(&app);
        if result_code >= 0 {
            // It was a worker process, so stop here
            std::process::exit(result_code as i32);
        }
    }

    let mut settings = Settings::default();
    // settings.log_severity = cef_log_severity_t_LOGSEVERITY_VERBOSE;
    settings.remote_debugging_port = Some(9876);
    settings.windowless_rendering_enabled = true;
    settings.multi_threaded_message_loop = true;

    cef::initialize(settings, &app);

    println!("ready");

    //    std::thread::spawn(|| {
    //    std::thread::sleep(Duration::from_secs(2));

    //        let client2 = client.clone();
    cef::post_task(cef::ThreadId::TID_UI, move || {
        let mut window_info = WindowInfo::default();
        window_info.width = 1280;
        window_info.height = 720;
        window_info.windowless_rendering_enabled = true;

        let mut browser_settings = BrowserSettings::default();
        browser_settings.windowless_frame_rate = 30; // TODO - not necessary here?

        let client = Arc::new(MyClient {
            render_handler: Arc::new(MyRenderHandler {}),
        });

        // Open a window
        create_browser_sync(window_info, &client, "http://google.com", browser_settings);
    })
    .unwrap();

    println!("waiting");
    std::thread::sleep(Duration::from_secs(600));

    println!("quit");
    cef::quit_message_loop();
    // TODO - this doesnt appear to be stopping the loop..
    //    });

    //    cef::run_message_loop();

    println!("shutting fown");

    cef::shutdown();
}
