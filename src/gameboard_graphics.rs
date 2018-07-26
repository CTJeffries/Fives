// ------------------------------------------------------------------------------------------------

// Colby Jeffries
// gameboard_graphics.rs

// This file contains all of the logic to display the gameboard.

// ------------------------------------------------------------------------------------------------

// Imports from External Dependencies
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

// Imports from Local Files
use Gameboard;

// ------------------------------------------------------------------------------------------------

// Struct that stores the graphical settings for the gameboard.
pub struct GameboardGraphicsSettings {
    pub position: [f64; 2],
    pub length: f64,
    pub background_color: Color,
    pub border_color: Color,
    pub border_edge_radius: f64,
    pub board_edge_color: Color,
    pub board_edge_radius: f64,
    pub text_color: Color,
}

impl GameboardGraphicsSettings {
    // Initializer.
    pub fn new() -> GameboardGraphicsSettings {
        GameboardGraphicsSettings {
            position: [10.0; 2],
            length: 580.0,
            background_color: [0.7, 0.7, 0.7, 1.0],
            border_color: [0.1, 0.1, 0.1, 1.0],
            border_edge_radius: 4.0,
            board_edge_color: [0.2, 0.2, 0.2, 1.0],
            board_edge_radius: 2.0,
            text_color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

// ------------------------------------------------------------------------------------------------

// Stores visual information about the gameboard, and implements functions to draw it.
pub struct GameboardGraphics {
    pub settings: GameboardGraphicsSettings,
}

impl GameboardGraphics {
    // Initializer.
    pub fn new(settings: GameboardGraphicsSettings) -> GameboardGraphics {
        GameboardGraphics {
            settings: settings,
        }
    }

    pub fn draw<G: Graphics, C: CharacterCache<Texture = G::Texture>>(&self, board: &Gameboard,
                                                                      glyphs: &mut C, c:&Context,
                                                                      g: &mut G) {
        use graphics::{Line, Rectangle, text, Transformed};

        let ref settings = self.settings;

        // Draw the board background.
        let board_rect = [
            settings.position[0], settings.position[1], settings.length, settings.length,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw numbers.
        let cell_size = settings.length / 5.0;
        for j in 0..5 {
            for i in 0..5 {
                let pos = [
                    settings.position[0] + i as f64 * cell_size + 20.0,
                    settings.position[1] + j as f64 * cell_size + 30.0
                ];
                if let Some(number) = board.get_string([i, j]) {
                    text::Text::new(34).draw(&number, glyphs, &c.draw_state,
                                             c.transform.trans(pos[0], pos[1]), g);
                }
            }
        }

        // Draw grid.
        let border_edge = Line::new(settings.border_color, settings.border_edge_radius);
        for i in 0..5 {
            let x = settings.position[0] + ((i as f64) / 5.0) * settings.length;
            let y = settings.position[1] + ((i as f64) / 5.0) * settings.length;
            let x2 = settings.position[0] + settings.length;
            let y2 = settings.position[1] + settings.length;

            let vline = [x, settings.position[1], x, y2];
            border_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            border_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}

// ------------------------------------------------------------------------------------------------
