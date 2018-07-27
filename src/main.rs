// ------------------------------------------------------------------------------------------------

// Colby Jeffries
// main.rs

// This is the main driver for the game Fives. This game is essentially a knock off of 2048.
// I am building this game with the intent of learning Rust and Piston. I have completed the
// Piston Sudoku tutorial, and will be using that as a reference for my work, along with the
// Rust/Cargo books.

// ------------------------------------------------------------------------------------------------

// External Dependencies
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

// Imports from External Dependencies
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};

// Imports from Local Files
pub use gameboard::Gameboard;
pub use gameboard_graphics::{GameboardGraphics, GameboardGraphicsSettings};

// Specify locations of local modules.
mod gameboard;
mod gameboard_graphics;

// ------------------------------------------------------------------------------------------------

// Main Driver
fn main() {
    // OpenGL settings object. Use version 3.2.
    let opengl_ver = OpenGL::V3_2;
    // Create windowing settings object. Title of the window is "Fives", and it is a 600x600px
    // square.
    // Srgb is set to false, as otherwise the application crashes.
    // Indicate which OpenGL version to use.
    // Allow the window to be exited by pressing Escape.
    let windowing_settings = WindowSettings::new("Fives", [600, 700])
        .srgb(false)
        .opengl(opengl_ver)
        .exit_on_esc(true);

    // Create a window, with failure error message.
    let mut window: GlutinWindow = windowing_settings.build().expect("Could not create window!");
    // Create events object. Lazy flag is set so that the event state is only updated if user
    // input is recieved.
    let mut events = Events::new(EventSettings::new().lazy(true));
    // Create a OpenGL shaders and buffers object.
    let mut gl = GlGraphics::new(opengl_ver);

    // Gameboard object.
    let mut gameboard = Gameboard::new();
    gameboard.new_game();
    // Gameboard graphics settings object.
    let gameboard_graphics_settings = GameboardGraphicsSettings::new();
    // Gameboard graphics object.
    let gameboard_graphics = GameboardGraphics::new(gameboard_graphics_settings);

    // Load the font.
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font.");

    // Event loop.
    while let Some(e) = events.next(&mut window) {
        // Pass event to the gameboard.
        gameboard.event(&e);
        // If the event is a render event.
        if let Some(args) = e.render_args() {
            // Create a graphics::Context and a graphics back end.
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                // Clear the viewport to white.
                clear([1.0; 4], g);

                // Render the gameboard.
                gameboard_graphics.draw(&gameboard, glyphs, &c, g);
            });
        }
    }

    // Debug
    println!("End of driver reached!");
}

// ------------------------------------------------------------------------------------------------
