extern crate termion;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}
use CellState::Alive;
use CellState::Dead;

type CellsRow = Vec<CellState>;
pub type Cells = Vec<CellsRow>; // All rows must have the same length

pub trait TableLike {
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn last_row(&self) -> usize {
        self.num_rows() - 1
    }
    fn last_col(&self) -> usize {
        self.num_cols() - 1
    }
}

impl TableLike for Cells {
    // TODO cache the len values?
    fn num_rows(&self) -> usize { self.len() }
    fn num_cols(&self) -> usize { self[0].len() }
}

pub fn step_toroidal(cells: &Cells, cells_next: &mut Cells) {
    // TODO don't panic if the board is one cells wide/high
    let (last_row, last_col) = (cells.last_row(), cells.last_col());
    // TODO inline instead of closures?
    let get_num_neighbors_middle = |row_i, col_i| -> u8 {
        let mut num_neighbors: u8 = 0;
        for (neighbor_row_i, neighbor_col_i) in [
            (row_i - 1, col_i - 1), (row_i - 1, col_i), (row_i - 1, col_i + 1),
            (row_i, col_i - 1), (row_i, col_i + 1),
            (row_i + 1, col_i - 1), (row_i + 1, col_i), (row_i + 1, col_i + 1),
        ].iter() {
            if cells[*neighbor_row_i as usize][*neighbor_col_i as usize] == Alive { num_neighbors += 1 };
        }
        return num_neighbors;
    };
    // TODO create a function for each map section so there are fewer checks?
    let get_num_neighbors_edge = |row_i, col_i| -> u8 {
        let is_edge = || -> bool {
            row_i == 0 || row_i == last_row || col_i == 0 || col_i == last_col
        };
        debug_assert!(is_edge(), "`get_num_neighbors_edge` must only be used for edge cells");
        let mut num_neighbors: u8 = 0;
        let get_neighbor_state_toroidal = |neighbor_of: (usize, usize), shift: (i8, i8)| -> CellState {
            debug_assert!(
                match shift.0 { -1 | 0 | 1 => true, _ => false }
                && match shift.1 { -1 | 0 | 1 => true, _ => false }
            );
            let (shift_row, shift_col) = shift;
            let (neighbor_of_row, neighbor_of_col) = neighbor_of;
            let translated_row: usize = if neighbor_of_row == 0 && shift_row == -1 {
                last_row
            } else if neighbor_of_row == last_row && shift_row == 1 {
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
            let translated_col: usize = if neighbor_of_col == 0 && shift_col == -1 {
                last_col
            } else if neighbor_of_col == last_col && shift_col == 1 {
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
        };
        for (shift_row, shift_col) in [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ].iter() {
            if get_neighbor_state_toroidal((row_i, col_i), (*shift_row, *shift_col)) == Alive {
                num_neighbors += 1
            };
        }
        return num_neighbors;
    };
    fn cell_next_state(curr_state: CellState, num_neighbors: u8) -> CellState {
        match num_neighbors {
            0...1 => Dead,
            2 => curr_state,
            3 => Alive,
            _ => Dead
        }
    }

    // Middle
    for row_i in 1..=(last_row - 1) {
        for col_i in 1..=(last_col - 1) {
            let num_neighbors = get_num_neighbors_middle(row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
    // Edges
    for &row_i in [0, last_row].iter() {
        for col_i in 0..=last_col {
            let num_neighbors = get_num_neighbors_edge(row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
    for row_i in 1..=(last_row - 1) {
        for &col_i in [0, last_col].iter() {
            let num_neighbors = get_num_neighbors_edge(row_i, col_i);
            cells_next[row_i as usize][col_i as usize] = cell_next_state(cells[row_i as usize][col_i as usize], num_neighbors);
        }
    }
}

pub fn update_term(cells: &Cells, step_num: &u32) {
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

pub fn init_term(cells: &Cells) {
    let (num_rows, num_cols) = (cells.num_rows(), cells.num_cols());
    print!("{}{}", termion::clear::All, termion::cursor::Hide); // TODO show it again when we exit (use `HideCursor` instead?)
    let draw_border = || {
        // The following vars describe the border, not the cells field itself
        let first_row: u16 = 2;
        let first_col: u16 = 1;
        let last_row: u16 = num_rows as u16 + first_row + 1;
        let last_col: u16 = num_cols as u16 + first_col + 1;
        print!("{}{}", termion::cursor::Goto(first_col, first_row), '╔');
        print!("{}{}", termion::cursor::Goto(first_col + 1, first_row), "═".repeat(num_cols));
        print!("{}{}", termion::cursor::Goto(last_col, first_row), '╗');
        for row_i in (first_row + 1)..(last_row) {
            print!("{}{}", termion::cursor::Goto(first_col, row_i), '║');
            print!("{}{}", termion::cursor::Goto(last_col, row_i), '║');
        }
        print!("{}{}", termion::cursor::Goto(first_col, last_row), '╚');
        print!("{}{}", termion::cursor::Goto(first_col + 1, last_row), "═".repeat(num_cols));
        print!("{}{}", termion::cursor::Goto(last_col, last_row), '╝');
    };
    draw_border();
}

pub fn parse_life(input: String) -> Result<Cells, String> {
    // TODO different error type?
    // TODO `input: &mut impl std::io::Read`?
    // TODO return Result instead of panicking. Result<Option<Cells>>?
    let mut lines = input.lines();
    // Skip the description. TODO parse it instead of skipping?
    let first_cells_line; // TODO get rid of this var somehow?
    loop {
        let line = match lines.next() {
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
    fn parse_row(line: &str) -> Result<CellsRow, String> {
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
    let row = parse_row(first_cells_line)?;
    let mut longest_row_len = row.len();
    let mut cells: Cells = vec![row];
    for line in lines {
        // If it's an empty line
        if let None = line.chars().next() {
            break;
        }

        let row = parse_row(line)?;
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
