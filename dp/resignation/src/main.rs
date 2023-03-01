use std::io;
use std::cmp;

#[derive(Debug)]
struct State {
    time: u32,
    pay: u32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse::<usize>().unwrap();

    let mut schedule: Vec<State> = Vec::new();
    let mut dp: Vec<u32> = vec![0; n+1];
    
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let time: u32 = iter.next().unwrap().parse::<u32>().unwrap();
        let pay: u32 = iter.next().unwrap().parse::<u32>().unwrap();
        schedule.push(State {time, pay});
    }

    schedule.insert(0, State {time:0, pay:0});

    for i in (1..=n).rev() {
        let include_now: u32 = if i + schedule[i].time as usize > n + 1 {
            0
        } else if i + schedule[i].time as usize == n + 1 {
            schedule[i].pay
        }
        else {
            dp[i + schedule[i].time as usize] + schedule[i].pay
        };

        let exclude_now: u32 = if i == n {
            0
        } else {
            dp[i+1]
        };
        dp[i] = cmp::max(include_now, exclude_now);
    }

    println!("{}", dp[1]);
}
