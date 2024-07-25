extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    create_window();
}

fn create_window() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(900, 900, "Axolotl", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // MAke the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
