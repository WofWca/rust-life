extern crate termion;

use std::{thread, time};
// Width and height of the cells array
const NUM_COLS: usize = 11;
const NUM_ROWS: usize = 12;
// Choose types that can fit the NUM_COLS and the NUM_ROWS accordingly.
type RowInd = u8;
type ColInd = u8;
const LAST_COL: ColInd = (NUM_COLS - 1) as ColInd;
const LAST_ROW: RowInd = (NUM_ROWS - 1) as RowInd;

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellState {
    Dead,
    Alive,
}
use CellState::Alive;
use CellState::Dead;

type CellsRow = Vec<CellState>;
type Cells = Vec<CellsRow>; // All rows must have the same length

fn step_toroidal(cells: &Cells, cells_next: &mut Cells) {
    // Let's divide the field into 9 sections.
    // 1 2 2 2 2 3
    // 4 5 5 5 5 6
    // 4 5 5 5 5 6
    // 4 5 5 5 5 6
    // 7 8 8 8 8 9

    // Used for edge cells.
    // TODO create a function for each map section so there are fewer checks?
    fn get_num_neighbors_edge(cells: &Cells, row_i: RowInd, col_i: ColInd) -> u8 {
        fn is_edge(row_i: RowInd, col_i: ColInd) -> bool {
            row_i == 0 || row_i == LAST_ROW || col_i == 0 || col_i == LAST_COL
        }
        debug_assert!(is_edge(row_i, col_i), "`get_num_neighbors_edge` must only be used for edge cells");
        let mut num_neighbors: u8 = 0;
        fn get_neighbor_state_toroidal(cells: &Cells, neighbor_of: (RowInd, ColInd), shift: (i8, i8)) -> CellState {
            debug_assert!(
                match shift.0 { -1 | 0 | 1 => true, _ => false }
                && match shift.1 { -1 | 0 | 1 => true, _ => false }
            );
            let (shift_row, shift_col) = shift;
            let (neighbor_of_row, neighbor_of_col) = neighbor_of;
            let translated_row: RowInd = if neighbor_of_row == 0 && shift_row == -1 {
                LAST_ROW
            } else if neighbor_of_row == LAST_ROW && shift_row == 1 {
                0
            } else {
                match shift_row {
                    -1 => neighbor_of_row - 1,
                    0 => neighbor_of_row,
                    1 => neighbor_of_row + 1,
                    _ => panic!()
                }
            };
            // TODO DRY
            let translated_col: ColInd = if neighbor_of_col == 0 && shift_col == -1 {
                LAST_COL
            } else if neighbor_of_col == LAST_COL && shift_col == 1 {
                0
            } else {
                match shift_col {
                    -1 => neighbor_of_col - 1,
                    0 => neighbor_of_col,
                    1 => neighbor_of_col + 1,
                    _ => panic!()
                }
            };
            return cells[translated_row as usize][translated_col as usize];
        }
        for (shift_row, shift_col) in [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ].iter() {
            if get_neighbor_state_toroidal(cells, (row_i, col_i), (*shift_row, *shift_col)) == Alive {
                num_neighbors += 1
            };
        }
        return num_neighbors;
    }
    fn get_num_neighbors_middle(cells: &Cells, row_i: RowInd, col_i: ColInd) -> u8 {
        let mut num_neighbors: u8 = 0;
        for (neighbor_row_i, neighbor_col_i) in [
            (row_i - 1, col_i - 1), (row_i - 1, col_i), (row_i - 1, col_i + 1),
            (row_i, col_i - 1), (row_i, col_i + 1),
            (row_i + 1, col_i - 1), (row_i + 1, col_i), (row_i + 1, col_i + 1),
        ].iter() {
            if cells[*neighbor_row_i as usize][*neighbor_col_i as usize] == Alive { num_neighbors += 1 };
        }
        return num_neighbors;
    }
    fn cell_next_state(curr_state: CellState, num_neighbors: u8) -> CellState {
        match num_neighbors {
            0...1 => Dead,
            2 => curr_state,
            3 => Alive,
            _ => Dead
        }
    }
    // Middle
    for row_i in 1..=(LAST_ROW - 1) {
        for col_i in 1..=(LAST_COL - 1) {
            let num_neighbors = get_num_neighbors_middle(cells, row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
    // Edges
    for &row_i in [0, LAST_ROW].iter() {
        for col_i in 0..=LAST_COL {
            let num_neighbors = get_num_neighbors_edge(cells, row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
    for row_i in 1..=(LAST_ROW - 1) {
        for &col_i in [0, LAST_COL].iter() {
            let num_neighbors = get_num_neighbors_edge(cells, row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
}

fn draw(cells: &Cells, step_num: &u32) {
    print!("{}Step: {}", termion::cursor::Goto(1,1), step_num);
    for (row_i, row) in cells.iter().enumerate() {
        print!("{}", termion::cursor::Goto(2, (row_i as u16) + 3));
        for cell in row.iter() {
            print!("{}", if *cell == Alive { '█' } else { ' ' });
        }
    }

    use std::io::Write;
    std::io::stdout().flush().unwrap();
}

fn init_board() {
    print!("{}{}", termion::clear::All, termion::cursor::Hide); // TODO show it again when we exit (use `HideCursor` instead?)
    fn draw_border() {
        // The following vars describe the border, not the cells field itself
        let first_row = 2;
        let first_col = 1;
        let last_row = NUM_ROWS as u16 + first_row + 1;
        let last_col = NUM_COLS as u16 + first_col + 1;
        print!("{}{}", termion::cursor::Goto(first_col, first_row), '╔');
        print!("{}{}", termion::cursor::Goto(first_col + 1, first_row), "═".repeat(NUM_COLS));
        print!("{}{}", termion::cursor::Goto(last_col, first_row), '╗');
        for row_i in (first_row + 1)..(last_row) {
            print!("{}{}", termion::cursor::Goto(first_col, row_i), '║');
            print!("{}{}", termion::cursor::Goto(last_col, row_i), '║');
        }
        print!("{}{}", termion::cursor::Goto(first_col, last_row), '╚');
        print!("{}{}", termion::cursor::Goto(first_col + 1, last_row), "═".repeat(NUM_COLS));
        print!("{}{}", termion::cursor::Goto(last_col, last_row), '╝');
    }
    draw_border();
}

fn parse_life(input: String) -> Result<Cells, String> {
    // TODO different error type?
    // TODO `input: &mut impl std::io::Read`?
    // TODO return Result instead of panicking. Result<Option<Cells>>?
    let mut lines = input.lines();
    // Skip the description. TODO parse it instead of skipping?
    let first_cells_line; // TODO get rid of this var somehow?
    loop {
        let line = lines.next();
        let line = match line {
            Some(line) => line,
            None => return Err(String::from("Invalid input: unexpected end of the string"))
        };
        match line.chars().nth(0) {
            Some(character) => {
                if character != '!' {
                    first_cells_line = line;
                    break;
                }
            },
            None => return Err(String::from("Invalid input: unexpected empty line"))
        }
    }
    fn parse_line(line: &str) -> Result<CellsRow, String> {
        let mut row: CellsRow = vec![];
        // TODO err if empty?
        for character in line.chars() {
            match character {
                '.' => row.push(Dead),
                'O' => row.push(Alive),
                _ => return Err(format!("Invalid cell value. Expected '.' or 'O', got '{}'", character)),
            }
        }
        Ok(row)
    }
    let row = match parse_line(first_cells_line) {
        Ok(cells) => cells,
        Err(err) => return Err(err),
    };
    let mut longest_row_len = row.len();
    let mut cells: Cells = vec![row];
    for line in lines {
        // If it's an empty line
        if let None = line.chars().next() {
            break;
        }

        let row = match parse_line(line) {
            Ok(cells) => cells,
            Err(err) => return Err(err),
        };
        if longest_row_len < row.len() { longest_row_len = row.len() }
        cells.push(row);
    }
    // Pad short rows
    for row in &mut cells {
        row.resize(longest_row_len, Dead);
    }
    // TODO allocate the required amount right away
    Ok(cells)
}
#[test]
fn all_rows_of_same_len() {
    let test_input = String::from(
        "!Name: Test Structure\n\
        !\n\
        .O\n\
        O.\n\
        O..");
    println!("{}", test_input);
    let cells = parse_life(test_input).unwrap();
    dbg!(&cells);
    let first_row_len = cells.len();
    for row in cells {
        assert_eq!(row.len(), first_row_len);
    }
}

fn main() {
    let mut step_num: u32 = 1;
    let filename = std::env::args().nth(1).unwrap();
    println!("File: {}", filename);
    let file_content = std::fs::read_to_string(filename).unwrap();
    println!("Content: {}", file_content);
    let mut cells = parse_life(file_content).unwrap();
    let mut cells_next: Cells = vec![vec![Dead; NUM_COLS]; NUM_ROWS]; // TODO can we not initialize this?

    init_board();
    loop {
        draw(&cells, &step_num);
        thread::sleep(time::Duration::from_millis(30));
        step_toroidal(&cells, &mut cells_next);
        std::mem::swap(&mut cells, &mut cells_next);
        step_num += 1;
    }
}
