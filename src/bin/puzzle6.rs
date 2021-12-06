use std::{error::Error, fmt::Display};
use utils::input_parser;

const NEW_FISH_TIMER_VAL: usize = 8;
const RESET_FISH_TIMER_VAL: usize = 6;

struct Simulation {
    values: [u64; NEW_FISH_TIMER_VAL + 1],
    iteration: u16,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "After {} days: {:?}", self.iteration, self.values)
    }
}

impl Simulation {
    fn new(input: Vec<usize>) -> Self {
        let values = input
            .into_iter()
            .fold([0; NEW_FISH_TIMER_VAL + 1], |mut acc, value| {
                acc[value] += 1;
                acc
            });

        Simulation {
            values,
            iteration: 0,
        }
    }

    fn simulation_iter(&self) -> impl Iterator<Item = [u64; NEW_FISH_TIMER_VAL + 1]> {
        (0..).scan(self.values, |generation, _| {
            let num_new_fish = generation[0];
            for i in 0..NEW_FISH_TIMER_VAL {
                generation[i] = generation[i + 1];
            }
            generation[RESET_FISH_TIMER_VAL] += num_new_fish;
            generation[NEW_FISH_TIMER_VAL] = num_new_fish;
            Some(*generation)
        })
    }

    pub fn run_simulation(&self, for_days: usize) -> u64 {
        self.simulation_iter()
            .nth(for_days - 1)
            .into_iter()
            .flat_map(|fish| fish.into_iter())
            .sum::<u64>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle6");
    let mut initial_timers = input[0]
        .split(',')
        .into_iter()
        .map(|val| val.parse::<usize>().expect("Failed to parse timer input"))
        .collect::<Vec<usize>>();
    initial_timers.sort();

    let sim = Simulation::new(initial_timers);

    let part_1 = sim.run_simulation(80);
    println!("Number of lanternfish after 80 days: {}", part_1);

    let part_2 = sim.run_simulation(256);
    println!("Number of lanternfish after 256 days: {}", part_2);
    Ok(())
}
