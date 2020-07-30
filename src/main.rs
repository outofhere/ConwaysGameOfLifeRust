extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
mod game;

const SIZEX: usize = 20;
const SIZEY: usize = 20;


fn main() -> Result<(), Error> {
    let matches = App::new("Coway's Game of life implementation")
        .version("0.1.0")
        .author("Igor Chervatyuk <ichervatyuk@gmail.com>")
        .about("Simple tool that reads points' coordinates from seed file and runs Conway's Game of life simulation. Sample seed file goes with the program.")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("Input file with seed. Should be split by colon as follows:\n x1:y1\nx2:y2"))
        .get_matches();

    // Vector for initial seed read from seed file.
    let mut game_seed_vector: Vec<game::Cell> = Vec::new();

    // Passing file to parse
    let seed_file = matches.value_of("file").unwrap_or("seed.txt");
    println!("File passed as seed: {}", seed_file);
    let seed = File::open(seed_file)?;
    let buffered = BufReader::new(seed);
    for line in buffered.lines() {
        let string = line.unwrap();
        if string.starts_with("//") {
            println!("Comment says: {}", string);
        } else {
            let splt = string.split(":");
            let splt: Vec<&str> = splt.collect();
            game_seed_vector.push(game::Cell{
                pos_x: splt[0].parse::<u32>().unwrap(),
                pos_y: splt[1].parse::<u32>().unwrap(),
            });
        }
    }

    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Convay",
        [500; 2]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = game::Game {
        size_x: SIZEX,
        size_y: SIZEY,
        gl: GlGraphics::new(opengl),
        seed: Vec::new(),
        field: Vec::new(),
    };

    game.constructor();
    game.populate(game_seed_vector);

    let mut events = Events::new(EventSettings::new()).ups(1);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_u) = e.update_args() {
            game.play();
        }
    }
    Ok(())
}
