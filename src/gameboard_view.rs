//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::gameboard_controller::GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 100.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
      use graphics::{Line, Rectangle};

      let ref settings = self.settings;
      let board_rect = [
          settings.position[0], settings.position[1],
          settings.size, settings.size,
      ];

      // Draw board background.
      Rectangle::new(settings.background_color)
          .draw(board_rect, &c.draw_state, c.transform, g);

      // Declare the format for cell and section lines.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        // Generate and draw the lines for the Sudoku Grid.
        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            let hline = [settings.position[0], y, x2, y];

            // Draw Section Lines instead of Cell Lines
            if (i % 3) == 0 {
                section_edge.draw(vline, &c.draw_state, c.transform, g);
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            }
            // Draw the regular cell Lines
            else {
                cell_edge.draw(vline, &c.draw_state, c.transform, g);
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }

      // Draw board edge.
      Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
          .draw(board_rect, &c.draw_state, c.transform, g);
  }
}
