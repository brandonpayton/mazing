extern crate piston_window;

use piston_window::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::cmp::Ordering;

fn main() {
    let window_width = 1250;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("triangle maze", [window_width; 2 ])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(false);

    const ROWS: usize = 70;
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM: usize = 2;
    let mut walls = [[true; 3]; ROWS * (ROWS + 1) / 2];

    let get_wall_index = |row: usize, col: usize| { row * (row + 1) / 2 + col / 2 };
    let mut cell_stack = LinkedList::new();
    cell_stack.push_back((0 as usize, 0 as usize));
    let mut visited_cells = HashSet::new();
    visited_cells.insert((0 as usize, 0 as usize));

    //while there are cells on the stack
    // if there is an unvisted neighbor
        //  push it on the stack
        //  break down the wall between
    // else 
        // pop cell off the stack
    
    while cell_stack.len() > 0 {
        match cell_stack.back().unwrap() {
            (row, col) => {
                let mut unvisited_cells = Vec::new();
                let first_col = 0;
                let last_col = 2 * row;
                if *col > 0 {
                    let left_cell = (*row, *col - 1);
                    let left_wall_index = get_wall_index(*row, *col);
                    let left_wall_side = if col % 2 == 0 { LEFT } else { RIGHT };
                    if &first_col < col && ! visited_cells.contains(&left_cell) {
                        unvisited_cells.push((left_cell, (left_wall_index, left_wall_side)));
                    }
                }

                let right_cell = (*row, *col + 1);
                let right_wall_index = get_wall_index(*row, col + (col % 2));
                let right_wall_side = if col % 2 == 0 { RIGHT } else { LEFT };
                if &last_col > col && ! visited_cells.contains(&right_cell) {
                    unvisited_cells.push((right_cell, (right_wall_index, right_wall_side)));
                }

                let vertical_cell = if col % 2 == 0 { (*row + 1, *col + 1) } else { (*row - 1, *col - 1) };
                let vertical_wall_index = 
                    if col % 2 == 0 { get_wall_index(*row, *col) }
                    else { get_wall_index(row - 1, col - 1) };
                if
                    ! visited_cells.contains(&vertical_cell) && (
                        (col % 2 == 0 && vertical_cell.0 < ROWS) ||
                        (col % 2 == 1 && vertical_cell.0 > 0 && *col < 2 * (*row - 1))
                    )
                {
                    unvisited_cells.push((vertical_cell, (vertical_wall_index, BOTTOM)));
                }

                if unvisited_cells.len() > 0 {
                    let next_cell_index = rand::random::<usize>() % unvisited_cells.len();
                    let next_cell_info = unvisited_cells[next_cell_index];
                    match next_cell_info {
                        (next_cell, (wall_index, wall_selector)) => {
                            visited_cells.insert(next_cell);
                            walls[wall_index][wall_selector] = false;
                            cell_stack.push_back(next_cell);
                        }
                    }
                } else {
                    cell_stack.pop_back();
                }
            }
        }
    }

    let mut cell_stack = LinkedList::new();
    let mut visited_cells = HashSet::new();
    let mut backtracked_cells: HashSet<(usize, usize)> = HashSet::new();
    cell_stack.push_back( (0, 0) );
    visited_cells.insert((0 as usize, 0 as usize));

    let mut rng = rand::thread_rng();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g );

            let padding = 17.0; // padding between top of window and drawing
            let center = window_width as f64 / 2.0;
            let color = [ 0.0, 0.0, 0.0, 1.0 ];
            let red = [ 1.0, 0.0, 0.0, 1.0 ];
            let green = [ 0.0, 1.0, 0.0, 1.0 ];
            let blue = [ 0.0, 0.0, 1.0, 1.0 ];
            let orange = [ 1.0, 0.65, 0.0, 1.0 ];
            let white = [ 1.0, 1.0, 1.0, 1.0 ];
            let line_width = 1.0;
            let angle = std::f64::consts::PI / 3.0;
            let side_length = ( window_width as f64 - 2.0 * padding ) / ROWS as f64;
            let row_height = (side_length / 2.0) * angle.tan();
            let x_shift = side_length / 2.0;
            let y_shift = row_height;
            let c = c.scale( 1.0, 1.0 );

            let exit_row = ROWS - 1;
            let exit_col = 2 * exit_row / 2;

            for row in 0..ROWS {
                for col in 0..=(2 * row) {
                    let frow = row as f64;
                    let fcol = col as f64;

                    let (x1, x2, x3) = if col % 2 == 0 {
                        (
                            center - x_shift - x_shift * frow + ( side_length * fcol / 2.0 ),
                            center - x_shift * frow + ( side_length * fcol / 2.0 ),
                            center + x_shift - x_shift * frow + ( side_length * fcol / 2.0 )
                        )
                    } else {
                        (
                            center - x_shift - x_shift * frow + ( side_length * fcol / 2.0 ),
                            center - x_shift * frow + ( side_length * fcol / 2.0 ),
                            center + x_shift - x_shift * frow + ( side_length * fcol / 2.0 )
                        )
                    };
                    let y1 = padding + y_shift * frow;
                    let y2 = padding + y_shift + y_shift * frow;

                    let vertices = [
                        [ x1, if col % 2 == 0 { y2 } else { y1 } ],
                        [ x2, if col % 2 == 0 { y1 } else { y2 } ],
                        [ x3, if col % 2 == 0 { y2 } else { y1 } ]
                    ];
                    let triangle_color =
                        if row == 0 && col == 0 { red }
                        else if row == exit_row && col == exit_col { green }
                        else if backtracked_cells.contains(&(row,col)) { orange }
                        else if visited_cells.contains(&(row,col)) { blue }
                        else { white };

                    polygon( triangle_color, &vertices, c.transform, g );

                    let wall_index: usize = get_wall_index(row, col);
                    if col % 2 == 0 {
                        if walls[wall_index][LEFT] {
                            line(color, line_width, [ x1, y2, x2, y1 ], c.transform, g);
                        }
                        if walls[wall_index][RIGHT] {
                            line(color, line_width, [ x2, y1, x3, y2 ], c.transform, g);
                        }
                        if walls[wall_index][BOTTOM] {
                            line(color, line_width, [ x1, y2, x3, y2 ], c.transform, g);
                        }
                    }
                }
            }

            let (current_row, current_col) = cell_stack.back().unwrap();
            if ! (*current_row == exit_row && *current_col == exit_col) {
                let mut possible_directions: Vec<(usize,usize)> = Vec::new();
                let last_row = ROWS - 1;
                if *current_col % 2 == 0 {
                    let wall_index = get_wall_index(*current_row, *current_col);
                    if *current_col > 0 && ! walls[wall_index][LEFT] {
                        let candidate = (*current_row, *current_col - 1);
                        if ! visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }

                    let last_col_in_this_row = 2 * *current_row;
                    if *current_col < last_col_in_this_row && ! walls[wall_index][RIGHT] {
                        let candidate = (*current_row, *current_col + 1);
                        if ! visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }

                    if *current_row < last_row && ! walls[wall_index][BOTTOM] {
                        let candidate = (*current_row + 1, *current_col + 1);
                        // TODO: Maybe just build the list and filter at the end.
                        if ! visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }
                } else {
                    {
                        let prev_wall_index = get_wall_index(*current_row, *current_col);
                        if ! walls[prev_wall_index][RIGHT] {
                            let candidate = (*current_row, *current_col - 1);
                            if ! visited_cells.contains(&candidate) {
                                possible_directions.push(candidate);
                            }
                        }
                    }

                    {
                        let next_wall_index = get_wall_index(*current_row, *current_col + 1);
                        if ! walls[next_wall_index][LEFT] {
                            let candidate = (*current_row, *current_col + 1);
                            if ! visited_cells.contains(&candidate) {
                                possible_directions.push(candidate);
                            }
                        }
                    }

                    {
                        let above_wall_index = get_wall_index(*current_row - 1, *current_col - 1);
                        if ! walls[above_wall_index][BOTTOM] {
                            let candidate = (*current_row - 1, *current_col - 1);
                            if ! visited_cells.contains(&candidate) {
                                possible_directions.push(candidate);
                            }
                        }
                    }
                }

                let num_possibilities: usize = possible_directions.len();
                // if zero: backtrack
                // else:
                if num_possibilities > 0 {
                    let prefer_nearest_to_exit = |a: &(usize,usize), b: &(usize,usize)| -> Ordering {
                        let x1 = a.0 as isize;
                        let y1 = a.1 as isize;
                        let x2 = b.0 as isize;
                        let y2 = b.1 as isize;
                        let exit_x = exit_col as isize;
                        let exit_y = exit_row as isize;

                        let d1 = (exit_x - x1) + (exit_y - y1);
                        let d2 = (exit_x - x2) + (exit_y - y2);

                        return d1.cmp(&d2);
                    };
                    possible_directions.sort_by(prefer_nearest_to_exit);
                    let next_cell = possible_directions[0];
                    cell_stack.push_back(next_cell);
                    visited_cells.insert(next_cell);
                } else {
                    // TODO: Be careful about unwrap
                    backtracked_cells.insert( cell_stack.pop_back().unwrap() );
                }
            }
       });
    }
}
