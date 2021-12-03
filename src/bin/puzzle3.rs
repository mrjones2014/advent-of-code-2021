use std::error::Error;

use utils::input_parser;

#[derive(Clone)]
struct DiagnosticReport {
    len: usize,
    values: Vec<u64>,
}

impl TryFrom<Vec<String>> for DiagnosticReport {
    type Error = Box<dyn Error>;

    fn try_from(input: Vec<String>) -> Result<Self, Self::Error> {
        let values = input
            .iter()
            .map(|line| u64::from_str_radix(line.trim(), 2))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            len: input
                .iter()
                .map(|line| line.trim().len())
                .max()
                .expect("Expected puzzle input"),
            values,
        })
    }
}

impl DiagnosticReport {
    fn compute_gamma_epsilon(&self) -> (u64, u64) {
        let mut gamma = 0;
        let mut epsilon = 0;
        let mut mask = 1;

        for _ in 0..self.len {
            let count = self.values.iter().filter(|n| *n & mask != 0).count();
            if count * 2 >= self.values.len() {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }

            mask <<= 1;
        }

        (gamma, epsilon)
    }

    fn life_support_rating(&self, use_gamma: bool) -> u64 {
        let mut report = self.clone();
        let mut mask = 1 << (report.len - 1);

        while report.values.len() > 1 {
            let (gamma, epsilon) = report.compute_gamma_epsilon();
            let rating = if use_gamma { gamma } else { epsilon };

            report.values = report
                .values
                .into_iter()
                .filter(|n| rating & mask == n & mask)
                .collect();

            mask >>= 1;
        }

        report.values[0]
    }

    pub fn oxygen(&self) -> u64 {
        self.life_support_rating(true)
    }

    pub fn c02(&self) -> u64 {
        self.life_support_rating(false)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle3");
    let report = DiagnosticReport::try_from(input)?;

    let (gamma, epsilon) = report.compute_gamma_epsilon();
    println!("Power consumption: {}", gamma * epsilon);
    println!("Life support rating: {}", report.oxygen() * report.c02());
    Ok(())
}
