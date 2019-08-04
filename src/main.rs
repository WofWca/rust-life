use life::game::step_toroidal;
use life::graphics::{update_term, init_term};
use life::parse_life;

use std::{thread, time};

fn main() {
    let mut step_num: u32 = 1;
    let filename = std::env::args().nth(1).unwrap();
    println!("File: {}", filename);
    let file_content = std::fs::read_to_string(filename).unwrap();
    println!("Content: {}", file_content);
    let mut cells = parse_life(file_content).unwrap();
    let mut cells_next = cells.clone(); // TODO can we not initialize this?

    init_term(&cells);
    loop {
        update_term(&cells, &step_num);
        thread::sleep(time::Duration::from_millis(30));
        step_toroidal(&cells, &mut cells_next);
        std::mem::swap(&mut cells, &mut cells_next);
        step_num += 1;
    }
}
