use serde::Deserialize;
use std::str::FromStr;
use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoopBuilder;
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

#[derive(Clone, Debug)]
enum AppEvent {
    ShowWindow(), // show-window
    DragWindow(), // drag-window
}

impl FromStr for AppEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<AppEventBody>(s) {
            Ok(body) => match body.event.as_str() {
                "show-window" => Ok(AppEvent::ShowWindow()),
                "drag-window" => Ok(AppEvent::DragWindow()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AppEventBody {
    event: String,
    message: Option<String>,
}

fn main() {
    let url = "http://localhost:5173";

    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();

    // window
    let window = WindowBuilder::new()
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0))
        .with_visible(false)
        .with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window.");

    // webview
    let webview = WebViewBuilder::new()
        .with_ipc_handler(move |msg| {
            println!("ipc msg: {}", msg.body());

            // 解析 ipc msg
            // 交给 event loop 进行处理
            match AppEvent::from_str(msg.body()) {
                Ok(event) => event_loop_proxy.send_event(event).unwrap(),
                Err(_) => {}
            }
        })
        .build(&window)
        .expect("Failed to create webview.");

    // 加载页面
    window.set_visible(true);
    webview.load_url(url).unwrap_or_else(|err| {
        panic!("Failed to load url {}: {}", url, err);
    });

    // 启动事件循环
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        println!("event: {:?}", event);

        match event {
            // 处理窗口事件
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } if window_id == window.id() => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }

            // 处理自定义事件
            Event::UserEvent(event) => match event {
                AppEvent::ShowWindow() => {
                    window.set_visible(true);
                }
                AppEvent::DragWindow() => {
                    window.drag_window().unwrap();
                }
            },
            // 其它事件
            _ => (),
        }
    });
}
