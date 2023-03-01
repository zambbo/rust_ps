use std::collections::VecDeque;

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut d = vec![vec!['.'; m]; n];
    for i in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let row: Vec<char> = input.trim().chars().collect();
        d[i].copy_from_slice(&row);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let sx: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let sy: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let ex: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let ey: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;

    let mut check = vec![vec![usize::MAX; m]; n];
    let mut q = VecDeque::new();
    q.push_back((sx, sy));
    check[sx][sy] = 0;

    while let Some((x, y)) = q.pop_front() {
        for i in 0..4 {
            let mut nx = x as i32 + DX[i];
            let mut ny = y as i32 + DY[i];
            let mut nk = 1;
            while nk <= k && nx >= 0 && ny >= 0 && nx < n as i32 && ny < m as i32 && d[nx as usize][ny as usize] != '#' && check[nx as usize][ny as usize] > check[x][y] {
                if check[nx as usize][ny as usize] == usize::MAX {
                    q.push_back((nx as usize, ny as usize));
                    check[nx as usize][ny as usize] = check[x][y] + 1;
                }
                nx += DX[i];
                ny += DY[i];
                nk += 1;
            }
        }
    }

    if check[ex][ey] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", check[ex][ey]);
    }
}
