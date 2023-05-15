use std::iter::repeat;

pub fn get_putable_nums(board: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut nums = repeat(repeat(vec![]).take(9).collect::<Vec<Vec<i32>>>())
        .take(9)
        .collect::<Vec<Vec<Vec<i32>>>>();
    for n in 1..10 {
        for y in 0..9 {
            for x in 0..9 {
                if board[y as usize][x as usize] != 0 {
                    continue;
                }
                if !putable_in_box(&board.clone(), n, x, y) {
                    continue;
                }
                if !putable_in_col(&board.clone(), n, x) {
                    continue;
                }
                if !putable_in_row(&board.clone(), n, y) {
                    continue;
                }
                nums[y as usize][x as usize].push(n);
            }
        }
    }

    return nums;
}

fn putable_in_box(board: &Vec<Vec<i32>>, num: i32, x: i32, y: i32) -> bool {
    let x_rng;
    let y_rng;
    if 0 <= x && x <= 2 {
        x_rng = 0..3;
    } else if 3 <= x && x <= 5 {
        x_rng = 3..6;
    } else {
        x_rng = 6..9;
    }
    if 0 <= y && y <= 2 {
        y_rng = 0..3;
    } else if 3 <= y && y <= 5 {
        y_rng = 3..6;
    } else {
        y_rng = 6..9;
    }

    for y in y_rng {
        for x in x_rng.clone() {
            if board[y][x] == num {
                return false;
            }
        }
    }
    return true;
}

fn putable_in_col(board: &Vec<Vec<i32>>, num: i32, x: i32) -> bool {
    for y in 0..9 {
        if board[y][x as usize] == num {
            return false;
        }
    }
    return true;
}

fn putable_in_row(board: &Vec<Vec<i32>>, num: i32, y: i32) -> bool {
    for x in 0..9 {
        if board[y as usize][x] == num {
            return false;
        }
    }
    return true;
}

pub fn put_nums(board: &Vec<Vec<i32>>, nums: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut res = board.clone();
    for y in 0..9 {
        for x in 0..9 {
            for n in &nums[y as usize][x as usize] {
                if can_put(nums.clone(), n.clone(), x.clone(), y.clone()) {
                    res[y as usize][x as usize] = n.clone();
                }
            }
        }
    }
    return res;
}

pub fn can_put(nums: Vec<Vec<Vec<i32>>>, n: i32, x: i32, y: i32) -> bool {
    let mut only_in_y = true;
    for sy in 0..9 {
        if nums[sy as usize][x as usize].contains(&n) {
            if y != sy {
                only_in_y = false;
            }
        }
    }
    let mut only_in_x = true;
    for sx in 0..9 {
        if nums[y as usize][sx as usize].contains(&n) {
            if x != sx {
                only_in_x = false;
            }
        }
    }

    let mut only_in_box = true;
    let x_rng;
    let y_rng;
    if 0 <= x && x <= 2 {
        x_rng = 0..3;
    } else if 3 <= x && x <= 5 {
        x_rng = 3..6;
    } else {
        x_rng = 6..9;
    }
    if 0 <= y && y <= 2 {
        y_rng = 0..3;
    } else if 3 <= y && y <= 5 {
        y_rng = 3..6;
    } else {
        y_rng = 6..9;
    }
    for sy in y_rng {
        for sx in x_rng.clone() {
            if nums[sy][sx].contains(&n) {
                if sx != x as usize || sy != y as usize {
                    only_in_box = false;
                }
            }
        }
    }
    return only_in_x || only_in_y || only_in_box;
}
