use glutin::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
};
use luminance_glutin::GlutinSurface;

fn main() {
    let (mut surface, event_loop) = GlutinSurface::new_gl33_from_builders(
        |_, win_builder| {
            win_builder
                .with_title("Zirconium")
                .with_inner_size(PhysicalSize::new(960, 540))
        },
        |_, ctx_builder| ctx_builder.with_double_buffer(Some(true))
    ).expect("Failed to create surface.");

    let mut back_buffer = surface.back_buffer().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_) => back_buffer = surface.back_buffer().unwrap(),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                surface.ctx.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                let render = surface
                    .new_pipeline_gate()
                    .pipeline(
                        &back_buffer,
                        &PipelineState::default().set_clear_color([0.33, 0.38, 1.0, 1.0]),
                        |_, mut shd_gate| {
                            Ok(())
                        }
                    ).assume();

                if render.is_ok() {
                    surface.swap_buffers();
                }
            },
            _ => (),
        };
    });
}