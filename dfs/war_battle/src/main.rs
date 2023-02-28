use std::io;

static mut white_power: u32 = 0;
static mut blue_power: u32 = 0;

fn valid_next(war_space: &Vec<Vec<char>>, cur_point: (i32, i32), col_size: i32, row_size: i32) -> Vec<(usize, usize)> {
    let next: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    
    let valid_next: Vec<(i32, i32)> = next.iter().
                                        map(|x| (x.0 + cur_point.0, x.1 + cur_point.1)).
                                        filter(|x| x.0 >=0 && x.1 >=0 && x.0 < row_size && x.1 < col_size && war_space[cur_point.0 as usize][cur_point.1 as usize] == war_space[x.0 as usize][x.1 as usize]).
                                        collect();
    
    let valid_next: Vec<(usize, usize)> = valid_next.iter().map(|x| (x.0 as usize, x.1 as usize)).collect();

    valid_next
}


fn dfs(war_space: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, point: (usize, usize), col_size: i32, row_size: i32) {
    if visited[point.0][point.1] {
        return;
    }
    
    visited[point.0][point.1] = true;
    unsafe {
        match war_space[point.0][point.1] {
            'W' => {white_power += 1;},
            'B' => {blue_power += 1;},
            _ => panic!()
        }
    }
    let nexts: Vec<(usize, usize)> = valid_next(war_space, (point.0 as i32, point.1 as i32), col_size, row_size);

    for &next in nexts.iter() {
        dfs(war_space, visited, next, col_size, row_size);
    }
    
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut line_iter = line.trim().split_whitespace();

    let n: usize = line_iter.next().unwrap().parse().unwrap(); // col_size
    let m: usize = line_iter.next().unwrap().parse().unwrap(); // row_size

    let mut war_space: Vec<Vec<char>> = Vec::new();

    for i in 1..=m {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let war_line: Vec<char> = line.trim().chars().collect();
        war_space.push(war_line);
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; n];m];

    let mut total_white_power: u32 = 0;
    let mut total_blue_power: u32 = 0;

    unsafe {
        for i in 0..m {
            for j in 0..n {
                white_power = 0;
                blue_power = 0;
                dfs(&war_space, &mut visited, (i, j), n as i32, m as i32);
                total_white_power += white_power * white_power;
                total_blue_power += blue_power * blue_power;
            }
        }
    }

    println!("{} {}", total_white_power, total_blue_power);

}
