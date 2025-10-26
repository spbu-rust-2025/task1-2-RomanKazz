use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let num: i32 = input.trim().parse().unwrap();

        if num == -1 {
            println!("{}", sum);
            break;
        }

        if num <= 0 {
            sum += num;
        } else {
            println!("NaN");
            break;
        }
    }
}
