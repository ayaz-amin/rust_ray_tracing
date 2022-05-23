use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod math;
mod camera;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 225;

fn hit_sphere(center: math::Vec3, radius: f32, camera: &camera::Camera, ray_dir: &math::Vec3) -> f32 {
    let oc = math::Vec3::new(
        camera.origin.x - center.x,
        camera.origin.y - center.y,
        camera.origin.z - center.z
        );

    let a = math::dot_prod(&ray_dir, &ray_dir);
    let b = math::dot_prod(&oc, &ray_dir);
    let c = math::dot_prod(&oc, &oc) - (radius * radius);
    let discriminant = (b * b) - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / a
    }
}

fn draw(frame: &mut [u8], camera: &camera::Camera) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

        let x = (i % WIDTH as usize) as f32;
        let y = (i / WIDTH as usize) as f32;
        let u = x / (WIDTH - 1) as f32;
        let v = y / (HEIGHT - 1) as f32;

        let dir = camera.direction(u, v);
        let dir_normalized = dir.normalized();

        let r: f32;
        let g: f32;
        let b: f32;

        let mut t = hit_sphere(math::Vec3::new(0.0, 0.0, -1.0), 0.5, &camera, &dir); 

        if t > 0.0 {
            let N = math::Vec3::new(
                camera.origin.x + t * dir.x,
                camera.origin.y + t * dir.y,
                camera.origin.z + t * dir.z - 1.0
                ).normalized();

            r = 127.5 * (N.x + 1.0);
            g = 127.5 * (N.y + 1.0);
            b = 127.5 * (N.z + 1.0);
        } else {
            t = 0.5 * (dir_normalized.y + 1.0);
            r = ((1.0 - t) * 0.5 + (t * 0.9)) * 255.0;
            g = ((1.0 - t) * 0.7 + (t * 0.9)) * 255.0;
            b = ((1.0 - t) * 0.8 + (t * 0.9)) * 255.0;
        }
        
        let rgba = [r as u8, g as u8, b as u8, 255];
        pixel.copy_from_slice(&rgba);
    }
}

fn main() {
    let camera = camera::Camera::new(WIDTH as f32, HEIGHT as f32);

    let event_loop = EventLoop::new();
    let window = {
        let min_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Test")
            .with_inner_size(min_size)
            .with_min_inner_size(min_size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {

        match event {
            
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                draw(pixels.get_frame(), &camera);
                if pixels.render().map_err(|e| {}).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },

            Event::WindowEvent {ref event, window_id} if window_id == window.id() =>
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape), 
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    _ => {}
                },

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}

        }
    });
}
