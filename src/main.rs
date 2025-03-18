use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    // window
    let window_builder = WindowBuilder::new()
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(800.0, 60.0));
    let window = window_builder.build(&event_loop).unwrap();

    // webview
    let webview_builder = WebViewBuilder::new()
        .with_url("http://localhost:5173")
        .with_ipc_handler(|msg| {
            // 打印
            println!("{:?}", msg);
        })
        .with_devtools(true);
    let _webview = webview_builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
