use std::io;

struct ExponentiationInput {
    pub a: u32,
    pub e: u32,
    pub n: u32,
}

impl ExponentiationInput {
    fn new(input: &str) -> Result<ExponentiationInput, &'static str> {
        let numbers: Vec<u32> = input
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .flatten()
            .collect();

        if numbers.len() != 3 {
            let msg = "Expected 3 numbers";
            eprintln!("{}", msg);
            return Err(msg);
        }
        let a = numbers[0];
        let e = numbers[1];
        let n = numbers[2];

        if n == 0 {
            let msg = "Expected n to be larger than zero";
            eprintln!("{}", msg);
            return Err(msg);
        }

        Ok(ExponentiationInput { a, e, n })
    }
}

fn main() -> Result<(), &'static str> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let exponentiation_input = ExponentiationInput::new(&input).unwrap();

    let p = calculate_modular_exponentiation(&exponentiation_input);
    println!(
        "{}^{} is congruent to {} (mod {})",
        exponentiation_input.a, exponentiation_input.e, p, exponentiation_input.n
    );

    Ok(())
}

fn calculate_modular_exponentiation(input: &ExponentiationInput) -> u32 {
    let mut a = input.a;
    let mut p = 1;
    let mut exponent = input.e;

    while exponent != 0 {
        if is_odd(exponent) {
            p = a * p % input.n;
            exponent = (exponent - 1) / 2;
        } else {
            exponent = exponent / 2;
        }
        a = (a * a) % input.n;
    }

    p
}

fn is_odd(n: u32) -> bool {
    n & 1 == 1
}
