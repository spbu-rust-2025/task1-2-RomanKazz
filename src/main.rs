use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("NaN");
            return;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                if num == -1 {
                    println!("{}", sum);
                    return;
                } else if num > 0 {
                    sum += num;
                } else {
                    println!("NaN");
                    return;
                }
            }
            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }
}
