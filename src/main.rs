use std::{thread, time};
// Width and height of the cells array
const NUM_COLS: usize = 31;
const NUM_ROWS: usize = 11;
// Choose types that can fit the NUM_COLS and the NUM_ROWS accordingly.
type RowInd = u8;
type ColInd = u8;
const LAST_COL: ColInd = (NUM_COLS - 1) as ColInd;
const LAST_ROW: RowInd = (NUM_ROWS - 1) as RowInd;
type CellsType = [[bool; NUM_COLS]; NUM_ROWS]; // So access items with arr[row_i][col_i].

fn step_toroidal(cells: CellsType, cells_next: &mut CellsType) {
    // Let's divide the field into 9 sections.
    // 1 2 2 2 2 3
    // 4 5 5 5 5 6
    // 4 5 5 5 5 6
    // 4 5 5 5 5 6
    // 7 8 8 8 8 9

    // Used for edge cells.
    // TODO create a function for each map section so there are fewer checks?
    fn get_num_neighbors_edge(cells: CellsType, row_i: RowInd, col_i: ColInd) -> u8 {
        fn is_edge(row_i: RowInd, col_i: ColInd) -> bool {
            row_i == 0 || row_i == LAST_ROW || col_i == 0 || col_i == LAST_COL
        }
        debug_assert!(is_edge(row_i, col_i), "`get_num_neighbors_edge` must only be used for edge cells");
        let mut num_neighbors: u8 = 0;
        fn is_neighbor_alive_toroidal(cells: CellsType, neighbor_of: (RowInd, ColInd), shift: (i8, i8)) -> bool {
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
            if is_neighbor_alive_toroidal(cells, (row_i, col_i), (*shift_row, *shift_col)) { num_neighbors += 1 };
        }
        return num_neighbors;
    }
    fn get_num_neighbors_middle(cells: CellsType, row_i: RowInd, col_i: ColInd) -> u8 {
        let mut num_neighbors: u8 = 0;
        for (neighbor_row_i, neighbor_col_i) in [
            (row_i - 1, col_i - 1), (row_i - 1, col_i), (row_i - 1, col_i + 1),
            (row_i, col_i - 1), (row_i, col_i + 1),
            (row_i + 1, col_i - 1), (row_i + 1, col_i), (row_i + 1, col_i + 1),
        ].iter() {
            if cells[*neighbor_row_i as usize][*neighbor_col_i as usize] { num_neighbors += 1 };
        }
        return num_neighbors;
    }
    dbg!(get_num_neighbors_middle(cells, 1, 2));
}

fn draw(cells: &CellsType, step_num: u8) {
    println!("Step: {}", step_num);
    for row in cells.iter() {
        for cell in row.iter() {
            print!("{}", if *cell { 'x' } else { '_' });
        }
        print!("\n");
    }
} 

fn main() {
    let mut cells: CellsType = [[false; NUM_COLS]; NUM_ROWS];
    let mut cells_next: CellsType = cells; // TODO can we not initialize this?
    let mut step_num = 1;
    // Glider
    cells[0][1] = true;
    cells[1][2] = true;
    cells[2][0] = true;
    cells[2][1] = true;
    cells[2][2] = true;
    loop {
        draw(&cells, step_num);
        thread::sleep(time::Duration::from_millis(500));
        step_toroidal(cells, &mut cells_next);
        step_num += 1;
    }
}
