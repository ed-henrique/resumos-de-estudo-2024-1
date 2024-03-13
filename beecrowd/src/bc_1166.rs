use std::io;
use std::vec::Vec;

pub fn is_perfect_square(n: i64) -> bool {
    if n == 0 {
        return true;
    }

    let square_n = (n as f64).sqrt() as i64;
    return n == square_n * square_n;
}

pub fn maximum_number_of_balls(n: i64) -> i64 {
    let mut vecs = Vec::<Vec<i64>>::new();

    for _ in 0..n {
        vecs.push(Vec::new())
    }

    let mut num_balls = 0;
    let mut can_ball = true;

    while can_ball {
        can_ball = false;

        for v in vecs.iter_mut() {
            if v.last().copied().unwrap_or(0) == 0 || is_perfect_square(v.last().copied().unwrap_or(0) + num_balls + 1) {
                num_balls += 1;
                v.push(num_balls);
                can_ball = true;
                break;
            }
        }
    }

    return num_balls;
}

pub fn run_hanoi_tower() {
    // Pegar o input do usuário
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    let mut num_lines = input.trim().parse::<i64>().unwrap();

    while num_lines > 0 {
        let mut num_input = String::new();

        match io::stdin().read_line(&mut num_input) {
            Ok(_) => {}
            Err(_) => {}
        }

        let n = num_input.trim().parse::<i64>().unwrap();

        let a = maximum_number_of_balls(n);
        println!("{:?}", a);

        num_lines -= 1;
    }
}
