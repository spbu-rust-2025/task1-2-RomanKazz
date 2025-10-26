use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let num: i32 = input.trim().parse().unwrap();

        if num == -1 {
            println!("{}", sum);
            return;
        }

        if num < 0 {
            println!("NaN");
            return;
        }

        sum += num;
    }
}
