use std::io;
use std::collections::HashMap;

pub fn fibonacci(memo: &mut HashMap<i64, (i64, i64)>, n: i64) -> (i64, i64) {
    if n == 0 {
        return (0, 0);
    }

    if n == 1 {
        return (0, 1);
    }

    let result = memo.get(&n).copied();
    
    if result.is_some() {
        return result.unwrap();
    }

    let first_fibonacci = fibonacci(memo, n - 1);
    let second_fibonacci = fibonacci(memo, n - 2);

    let non_stored_result = (first_fibonacci.0 + second_fibonacci.0 + 2, first_fibonacci.1 + second_fibonacci.1);

    memo.insert(n, non_stored_result);

    return non_stored_result;
}

pub fn run_fibonacci() {
    // Pegar o input do usuário
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    let mut num_lines = input.trim().parse::<i64>().unwrap();

    // Fazer memoização para conseguir o resultado mais rápido
    let mut memo = HashMap::<i64, (i64, i64)>::new();

    while num_lines > 0 {
        let mut num_input = String::new();

        match io::stdin().read_line(&mut num_input) {
            Ok(_) => {}
            Err(_) => {}
        }

        let n = num_input.trim().parse::<i64>().unwrap();

        let a = fibonacci(&mut memo, n);
        println!("fib({:?}) = {:?} calls = {:?}", n, a.0, a.1);

        num_lines -= 1;
    }
}
