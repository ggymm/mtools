use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoopBuilder;
use tao::window::WindowBuilder;

use wry::WebViewBuilder;

#[derive(Clone, Debug)]
enum AppEvent {
    DragWindow(),
}

fn main() {
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();

    // window
    let window = WindowBuilder::new()
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0))
        .with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window.");

    // webview
    let _webview = WebViewBuilder::new()
        .with_url("http://localhost:5173")
        .with_ipc_handler(move |msg| {
            // 打印
            println!("{:?}", msg);
            match event_loop_proxy.send_event(AppEvent::DragWindow()) {
                Ok(_) => {
                    println!("发送成功");
                }
                Err(_) => {
                    println!("发送失败");
                }
            }
        })
        .build(&window)
        .expect("Failed to create webview.");

    // window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // 处理系统窗口事件
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } if window_id == window.id() => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }

            // 处理自定义IPC事件
            Event::UserEvent(AppEvent::DragWindow()) => {
                // println!("移动窗口到({}, {})", x, y);
                // let (phy_x, phy_y) = to_physical_checked(&window, x, y);
                match window.drag_window() {
                    Ok(_) => {
                        println!("移动窗口成功");
                    }
                    Err(_) => {
                        println!("移动窗口失败");
                    }
                }
            }
            // 其它事件
            _ => (),
        }
    });
}