use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_vecitems() -> Vec<usize>{
    read_line()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn main(){
    let _n: usize = read_line().parse().unwrap();
    let list = read_vecitems();

    println!("{:?}", list);
}