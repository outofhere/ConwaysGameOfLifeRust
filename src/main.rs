extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::color;

const SIZEX: usize = 10;
const SIZEY: usize = 10;

#[derive(Clone)]
enum CellState {
    Alive,
    Dead,
}

struct Game {
    size_x: usize,
    size_y: usize,
    gl: GlGraphics,
    seed: Vec<CellState>,
    field: Vec<CellState>,
}


impl Game {
    fn constructor(&mut self) {
        self.seed = vec![CellState::Dead; self.size_x * self.size_y];
        self.field = vec![CellState::Dead; self.size_x * self.size_y];
        println!("Seed size: {}", self.seed.len());
        println!("Field size: {}", self.field.len());
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(color::WHITE, gl);
        });
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                match self.seed[x + y*self.size_x] {
                    CellState::Alive => {
                        let mut cell = Cell{pos_x: (x * 20) as u32, pos_y: (y * 20) as u32};
                        cell.render(&mut self.gl, args);
                        },
                    CellState::Dead => {},
                }
            };
        }
    }

    fn populate(&mut self, vec: Vec<Cell>) {
        for item in vec {
            if item.pos_x >= self.size_x as u32 || item.pos_y >= self.size_y  as u32 {
                println!("[-] Invalid cell, will be ignored: {} {}", item.pos_x, item.pos_y);
            } else {
                println!("[+] Added: {} {}", item.pos_x, item.pos_y);
                self.seed[item.pos_x as usize + self.size_x * item.pos_y as usize] = CellState::Alive;
            }
        }
    }

    fn play(&mut self){
        let mut count_near_cells = 0;
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                let cells: Vec<(i32, i32)> = vec![
                    (x as i32 - 1, y as i32 - 1),
                    (x as i32 - 1, y as i32),
                    (x as i32 - 1, y as i32 + 1),
                    (x as i32, y as i32 - 1),
                    (x as i32, y as i32 + 1),
                    (x as i32 + 1, y as i32 - 1),
                    (x as i32 + 1, y as i32),
                    (x as i32 + 1, y as i32 + 1)
                ];
                for (x1, y1) in cells {
                    if x1 < 0 || y1 < 0 { continue };
                    if x1 as usize >= self.size_x || y1 as usize >= self.size_y { continue };
                    match self.seed[(x1 as usize) + (y1 as usize) * self.size_x] {
                        CellState::Alive => count_near_cells+= 1,
                        CellState::Dead => {},
                    }
                }
                if count_near_cells < 2 {
                    self.field[x + y * self.size_x] = CellState::Dead;
                } else if count_near_cells == 2 {
                    match self.seed[x + y * self.size_x] {
                        CellState::Alive =>  {
                            self.field[x + y * self.size_x] = CellState::Alive;
                            },
                        CellState::Dead => {
                            self.field[x + y * self.size_x] = CellState::Dead;
                            },
                        }
                    } else if count_near_cells == 3 {
                        self.field[x + y * self.size_x] = CellState::Alive;
                        } else if count_near_cells > 3 {
                            self.field[x + y * self.size_x] = CellState::Dead;
                            } else {
                                println!("Impossible situation {} {}", x, y);
                                }
                count_near_cells = 0;
            }
        }
        self.seed = self.field.clone();
    }
}
#[derive(Debug)]
struct Cell {
    pos_x: u32,
    pos_y: u32,
}

impl Cell {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(
            self.pos_x as f64,
            self.pos_y as f64,
            20_f64,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(color::BLACK, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Convay",
        [500; 2]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    /*
    let vec: Vec<Cell> = vec![
        Cell{
            pos_x: 1,
            pos_y: 1,
        },
        Cell{
            pos_x: 2,
            pos_y: 1,
        },
        Cell{
            pos_x: 1,
            pos_y: 2,
        },
        Cell{
            pos_x: 4,
            pos_y: 4,
        },
        Cell{
            pos_x: 4,
            pos_y: 3,
        },
        Cell{
            pos_x: 3,
            pos_y: 4,
        },
    ];
    */
    let vec: Vec<Cell> = vec![
        Cell{
            pos_x: 3,
            pos_y: 2,
        },
        Cell{
            pos_x: 3,
            pos_y: 3,
        },
        Cell{
            pos_x: 3,
            pos_y: 4,
        },
        Cell{
            pos_x: 4,
            pos_y: 1,
        },
        Cell{
            pos_x: 4,
            pos_y: 2,
        },
        Cell{
            pos_x: 4,
            pos_y: 3,
        },
    ];


    let mut game = Game {
        size_x: SIZEX,
        size_y: SIZEY,
        gl: GlGraphics::new(opengl),
        seed: Vec::new(),
        field: Vec::new(),
    };

    game.constructor();
    game.populate(vec);

    let mut events = Events::new(EventSettings::new()).ups(1);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_u) = e.update_args() {
            game.play();
        }
    }
}
