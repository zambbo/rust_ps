use std::io;
use std::fmt::Write;
//1...1 (M)
//N...N (M)

fn promising(v: &Vec<usize>) -> bool {
        !(1..v.len()).any(|i| v[i..].contains(&v[i-1])) && (1..v.len()).all(|i| v[i] > v[i-1])
}

fn backtracking(v: Vec<usize>, numbers: &Vec<usize>, N: usize, M: usize, output: &mut String) {
        if v.len() == M {
                writeln!(output, "{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
                return;
        }

        for &number in numbers.iter() {
                let mut v: Vec<usize> = v.clone();
                v.push(number);
                if promising(&v) {
                        backtracking(v, &numbers, N, M, output);
                }
        }
}

fn main() {
        let mut NM = String::new();
        io::stdin().read_line(&mut NM).unwrap();
        let mut NM_iter = NM.trim().split(" ").map(|x| x.parse::<usize>().unwrap());
        let N: usize = NM_iter.next().unwrap();
        let M: usize = NM_iter.next().unwrap();

        let v: Vec<usize> = Vec::new();
        let mut output = String::new();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers: Vec<usize> = line.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

        numbers.sort();

        backtracking(v, &numbers, N, M, &mut output);
        print!("{}", output);


}
