use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input.parse::<i32>() {
            Ok(num) => {
                if num == -1 {
                    println!("{}", sum);
                    return;
                }

                if num >= 0 {
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
