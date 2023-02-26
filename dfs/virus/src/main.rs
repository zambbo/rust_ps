use std::io;

fn dfs(computers: &Vec<Vec<bool>>, virus: &mut Vec<bool>, comp_num: usize) {
    for (i, &neighbor) in computers[comp_num].iter().enumerate() {
        if neighbor {
            if !virus[i] {
                virus[i] = true;
                dfs(computers, virus, i);
            }
        }
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse::<usize>().expect("error while parsing n");

    let mut pair_num = String::new();
    io::stdin().read_line(&mut pair_num).unwrap();
    let pair_num: usize = pair_num.trim().parse::<usize>().expect("error while parsing pair num");

    let mut computers: Vec<Vec<bool>> = vec![vec![false; n+1];n+1];

    for i in 0..pair_num {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line = line.trim().split(" ").map(|x| x.parse::<usize>().unwrap());
        
        let comp1: usize = line.next().unwrap();
        let comp2: usize = line.next().unwrap();

        computers[comp1][comp2] = true;
        computers[comp2][comp1] = true;
    }

    let mut virus: Vec<bool> = vec![false; n+1];
    virus[1] = true;

    dfs(&computers, &mut virus, 1);

    let total: usize = virus.iter().filter(|x| **x).collect::<Vec<&bool>>().len();

    println!("{}", total - 1);
}
