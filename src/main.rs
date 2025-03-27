use serde::Deserialize;
use std::str::FromStr;
use tao::dpi::{LogicalPosition, LogicalSize, PhysicalPosition};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::monitor;
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 60.0;

#[derive(Clone, Debug)]
enum AppEvent {
    DragWindow(),
    ToggleSetting(),
}

impl FromStr for AppEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<AppEventBody>(s) {
            Ok(body) => match body.event.as_str() {
                "main:drag:window" => Ok(AppEvent::DragWindow()),
                "main:toggle:setting" => Ok(AppEvent::ToggleSetting()),
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

    let monitor = event_loop.primary_monitor().unwrap();
    let position = calculate_position(&monitor);

    // window
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_position(position)
        .with_resizable(false)
        .with_visible(false)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_focused(true)
        .build(&event_loop)
        .expect("Failed to create window.");

    // webview
    let webview = WebViewBuilder::new()
        .with_ipc_handler(move |msg| {
            println!("{:?}", msg);

            // 解析 ipc msg
            // 交给 event loop 进行处理
            match AppEvent::from_str(msg.body()) {
                Ok(event) => event_loop_proxy.send_event(event).unwrap(),
                Err(_) => {}
            }
        })
        .with_focused(true)
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
                AppEvent::DragWindow() => {
                    window.drag_window().unwrap();
                }
                AppEvent::ToggleSetting() => {
                    if window.inner_size().height as f64 > HEIGHT {
                        window.set_inner_size(LogicalSize::new(WIDTH, HEIGHT))
                    } else {
                        window.set_inner_size(LogicalSize::new(WIDTH, 200.0))
                    }
                }
            },
            // 其它事件
            _ => (),
        }
    });
}

fn calculate_position(monitor: &monitor::MonitorHandle) -> PhysicalPosition<f64> {
    let scale = monitor.scale_factor();

    // 窗口大小
    let window_size: LogicalSize<f64> = LogicalSize::new(WIDTH, HEIGHT);
    let logical_size: LogicalSize<f64> = monitor.size().to_logical(scale);

    // 计算坐标
    let x = (logical_size.width - window_size.width) / 2.0;
    let y = (logical_size.height * 0.20).round();
    
    // 转换为物理坐标
    LogicalPosition::new(x, y).to_physical::<f64>(scale).cast()
}
