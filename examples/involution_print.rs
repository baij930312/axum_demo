fn main() {
    let v = vec![
        vec![1, 2, 3, 2],
        vec![8, 3, 9, 4],
        vec![7, 2, 6, 5],
        vec![7, 6, 6, 5],
        vec![7, 6, 6, 5],
    ];
    involution(v);
}
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
struct State {
    dirc: Direction,
    star_row: usize,
    star_column: usize,
    end_row: usize,
    end_column: usize,
}

fn involution(v: Vec<Vec<i32>>) {
    if v.is_empty() {
        return;
    }
    let n = v.len(); //行
    let m = v[0].len(); // 列
    let mut state = State {
        dirc: Direction::Right,
        star_row: 0,
        star_column: 0,
        end_row: n - 1,
        end_column: m - 1,
    };

    while state.star_column <= state.end_column && state.star_row <= state.end_row {
        match state.dirc {
            Direction::Right => {
                for i in state.star_column..=state.end_column {
                    print!("{}", v[state.star_row][i]);
                }
                state.star_row += 1;
                state.dirc = Direction::Down;
            }
            Direction::Down => {
                for i in state.star_row..=state.end_row {
                    print!("{}", v[i][state.end_column]);
                }
                state.end_column -= 1;
                state.dirc = Direction::Left;
            }
            Direction::Left => {
                for i in (state.star_column..=state.end_column).rev() {
                    print!("{}", v[state.end_row][i]);
                }
                state.end_row -= 1;
                state.dirc = Direction::Up;
            }
            Direction::Up => {
                for i in (state.star_row..=state.end_row).rev() {
                    print!("{}", v[i][state.star_column]);
                }
                state.star_column += 1;
                state.dirc = Direction::Right;
            }
        }
    }
}
