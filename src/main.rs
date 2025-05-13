use macroquad::prelude::*;
//use rand::prelude::*;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::LinkedList;

// async fn setup_graphics(config: Conf) -> Result<RenderTarget, macroquad::Error> {
//     let mut config = config;
//     .
//     config.title = "My Game".to_string();
//     config.window_config.resizable = false;
//     // Enable multisample antialiasing (MSAA) with a 2x factor by default
//     config.window_config.msaa_samples = 2;
//     config.window_config.render_target_msaa_support = true;
//     Ok(RenderTarget::new(config, WindowMode::default(), None).await?)
// }

fn conf() -> Conf {
    Conf {
        window_title: "Mazing".to_owned(),
        fullscreen: false,
        sample_count: 8,
        //you can add other options too, or just use the default ones:
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    const ROWS: usize = 40;
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM: usize = 2;
    let mut walls = [[true; 3]; ROWS * (ROWS + 1) / 2];

    let get_wall_index = |row: usize, col: usize| row * (row + 1) / 2 + col / 2;
    let mut cell_stack = LinkedList::new();
    cell_stack.push_back((0 as usize, 0 as usize));
    let mut visited_cells = HashSet::new();
    let mut backtracked_cells: HashSet<(usize, usize)> = HashSet::new();
    visited_cells.insert((0 as usize, 0 as usize));

    //while there are cells on the stack
    // if there is an unvisted neighbor
    //  push it on the stack
    //  break down the wall between
    // else
    // pop cell off the stack

    // let render_target = render_target(screen_width() as u32, screen_height() as u32);
    // render_target.set_sampler_filter(SamplerFilter::Nearest);

    //set_default_filter_mode(FilterMode::Linear);

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
                    if &first_col < col && !visited_cells.contains(&left_cell) {
                        unvisited_cells.push((left_cell, (left_wall_index, left_wall_side)));
                    }
                }

                let right_cell = (*row, *col + 1);
                let right_wall_index = get_wall_index(*row, col + (col % 2));
                let right_wall_side = if col % 2 == 0 { RIGHT } else { LEFT };
                if &last_col > col && !visited_cells.contains(&right_cell) {
                    unvisited_cells.push((right_cell, (right_wall_index, right_wall_side)));
                }

                let vertical_cell = if col % 2 == 0 {
                    (*row + 1, *col + 1)
                } else {
                    (*row - 1, *col - 1)
                };
                let vertical_wall_index = if col % 2 == 0 {
                    get_wall_index(*row, *col)
                } else {
                    get_wall_index(row - 1, col - 1)
                };
                if !visited_cells.contains(&vertical_cell)
                    && ((col % 2 == 0 && vertical_cell.0 < ROWS)
                        || (col % 2 == 1 && vertical_cell.0 > 0 && *col < 2 * (*row - 1)))
                {
                    unvisited_cells.push((vertical_cell, (vertical_wall_index, BOTTOM)));
                }

                if unvisited_cells.len() > 0 {
                    let next_cell_index = rand::rand() as usize % unvisited_cells.len();
                    let next_cell_info = unvisited_cells[next_cell_index];
                    match next_cell_info {
                        (next_cell, (wall_index, wall_selector)) => {
                            visited_cells.insert(next_cell);
                            walls[wall_index][wall_selector] = false;
                            cell_stack.push_back(next_cell);
                        }
                    }
                } else {
                    backtracked_cells.insert(cell_stack.pop_back().unwrap());
                }
            }
        }
    }

    let mut cell_stack = LinkedList::new();
    let mut visited_cells = HashSet::new();
    let mut backtracked_cells: HashSet<(usize, usize)> = HashSet::new();
    cell_stack.push_back((0, 0));
    visited_cells.insert((0 as usize, 0 as usize));

    loop {
        // set_camera(&Camera2D {
        //     zoom: vec2(0.01, 0.01),
        //     target: vec2(0.0, 0.0),
        //     render_target: Some(render_target.clone()),
        //     ..Default::default()
        // });

        clear_background(WHITE);

        let window_width = screen_width();
        let window_height = screen_height();
        // TODO: If there a more elegant way to deal with these type coercions?
        let smaller_dimension = min(window_width as u32, window_height as u32) as f32;
        let padding: f32 = 17.0; // padding between top of window and drawing
                                 // let center_x: f32 = window_width as f32 / 2.0;
                                 // let center_y: f32 = window_height as f32 / 2.0;
        let (center_x, center_y) = if window_width < window_height {
            (0.0, window_height as f32 / 2.0)
        } else {
            (window_width as f32 / 2.0, 0.0)
        };
        let line_width: f32 = 1.0;
        let angle: f32 = std::f32::consts::PI / 3.0;
        let side_length: f32 = (smaller_dimension - 2.0 * padding) / ROWS as f32;
        let row_height: f32 = (side_length / 2.0) * angle.tan();
        let x_shift: f32 = side_length / 2.0;
        let y_shift: f32 = row_height;

        let exit_row = ROWS - 1;
        let exit_col = 2 * exit_row / 2;

        for row in 0..ROWS {
            for col in 0..=(2 * row) {
                let frow = row as f32;
                let fcol = col as f32;

                let (x1, x2, x3) = if col % 2 == 0 {
                    (
                        center_x - x_shift - x_shift * frow + (side_length * fcol / 2.0),
                        center_x - x_shift * frow + (side_length * fcol / 2.0),
                        center_x + x_shift - x_shift * frow + (side_length * fcol / 2.0),
                    )
                } else {
                    (
                        center_x - x_shift - x_shift * frow + (side_length * fcol / 2.0),
                        center_x - x_shift * frow + (side_length * fcol / 2.0),
                        center_x + x_shift - x_shift * frow + (side_length * fcol / 2.0),
                    )
                };
                let y1 = center_y + padding + y_shift * frow;
                let y2 = center_y + padding + y_shift + y_shift * frow;

                let vertices = [
                    Vec2 {
                        x: x1,
                        y: if col % 2 == 0 { y2 } else { y1 },
                    },
                    Vec2 {
                        x: x2,
                        y: if col % 2 == 0 { y1 } else { y2 },
                    },
                    Vec2 {
                        x: x3,
                        y: if col % 2 == 0 { y2 } else { y1 },
                    },
                ];
                let triangle_color = if row == 0 && col == 0 {
                    RED
                } else if row == exit_row && col == exit_col {
                    GREEN
                } else if backtracked_cells.contains(&(row, col)) {
                    ORANGE
                } else if visited_cells.contains(&(row, col)) {
                    BLUE
                } else {
                    WHITE
                };

                //polygon( triangle_color, &vertices, c.transform, g );
                draw_triangle(vertices[0], vertices[1], vertices[2], triangle_color);

                let wall_index: usize = get_wall_index(row, col);
                let color = BLACK;
                if col % 2 == 0 {
                    if walls[wall_index][LEFT] {
                        draw_line(x1, y2, x2, y1, line_width, color);
                    }
                    if walls[wall_index][RIGHT] {
                        draw_line(x2, y1, x3, y2, line_width, color);
                    }
                    if walls[wall_index][BOTTOM] {
                        draw_line(x1, y2, x3, y2, line_width, color);
                    }
                }
            }
        }

        let (current_row, current_col) = cell_stack.back().unwrap();
        if !(*current_row == exit_row && *current_col == exit_col) {
            let mut possible_directions: Vec<(usize, usize)> = Vec::new();
            let last_row = ROWS - 1;
            if *current_col % 2 == 0 {
                let wall_index = get_wall_index(*current_row, *current_col);
                if *current_col > 0 && !walls[wall_index][LEFT] {
                    let candidate = (*current_row, *current_col - 1);
                    if !visited_cells.contains(&candidate) {
                        possible_directions.push(candidate);
                    }
                }

                let last_col_in_this_row = 2 * *current_row;
                if *current_col < last_col_in_this_row && !walls[wall_index][RIGHT] {
                    let candidate = (*current_row, *current_col + 1);
                    if !visited_cells.contains(&candidate) {
                        possible_directions.push(candidate);
                    }
                }

                if *current_row < last_row && !walls[wall_index][BOTTOM] {
                    let candidate = (*current_row + 1, *current_col + 1);
                    // TODO: Maybe just build the list and filter at the end.
                    if !visited_cells.contains(&candidate) {
                        possible_directions.push(candidate);
                    }
                }
            } else {
                {
                    let prev_wall_index = get_wall_index(*current_row, *current_col);
                    if !walls[prev_wall_index][RIGHT] {
                        let candidate = (*current_row, *current_col - 1);
                        if !visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }
                }

                {
                    let next_wall_index = get_wall_index(*current_row, *current_col + 1);
                    if !walls[next_wall_index][LEFT] {
                        let candidate = (*current_row, *current_col + 1);
                        if !visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }
                }

                {
                    let above_wall_index = get_wall_index(*current_row - 1, *current_col - 1);
                    if !walls[above_wall_index][BOTTOM] {
                        let candidate = (*current_row - 1, *current_col - 1);
                        if !visited_cells.contains(&candidate) {
                            possible_directions.push(candidate);
                        }
                    }
                }
            }

            let num_possibilities: usize = possible_directions.len();
            // if zero: backtrack
            // else:
            if num_possibilities > 0 {
                let prefer_nearest_to_exit = |a: &(usize, usize), b: &(usize, usize)| -> Ordering {
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
                backtracked_cells.insert(cell_stack.pop_back().unwrap());
            }

            next_frame().await;
        }
    }
}
