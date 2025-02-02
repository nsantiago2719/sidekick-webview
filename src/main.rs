use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::io::Read;
use tao::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::WindowBuilder,
};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};
use wry::WebViewBuilder;

fn main() {
    let item64: (String, bool) = get_content_from_clipboard();

    let _ = create_webview(item64);
}

fn create_webview(item_details: (String, bool)) -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sidekick Wrapper")
        .with_minimizable(false)
        .with_maximizable(false)
        .with_always_on_top(true)
        .with_focused(true)
        .with_inner_size(tao::dpi::LogicalSize::new(850.0, 1200.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(500.0, 700.0))
        .build(&event_loop)
        .unwrap();

    let (item64, waystone) = item_details;
    // Check if the item is waystone and use the proper URL for
    // map check else use the trade url
    let url: String = if waystone {
        format!("http://localhost:5000/map/xurl_{}", item64)
    } else {
        format!("http://localhost:5000/trade/xurl_{}", item64)
    };
    let builder = WebViewBuilder::new().with_url(&url);

    // Build only for linux
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // close program when Escape key is pressed
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: KeyCode::Escape,
                                state: ElementState::Pressed,
                                repeat: false,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        };
    });
}

fn get_content_from_clipboard() -> (String, bool) {
    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    let copied_text: String;
    match result {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            let _ = pipe.read_to_end(&mut contents);
            copied_text = String::from_utf8_lossy(&contents).to_string();
        }

        Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
            copied_text = "EMPTY".to_string();
            println!("INFO: Clipboard is empty or does not contain anything")
        }

        Err(err) => {
            copied_text = "ERROR".to_string();
            println!("ERROR: {err}")
        }
    }

    let waystone: bool = copied_text.contains("Waystone");
    return (URL_SAFE.encode(&copied_text), waystone);
}
