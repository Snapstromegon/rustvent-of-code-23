use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut modules = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|m: Module| (m.name().to_string(), m))
            .collect::<HashMap<String, Module>>();
        for (_, module) in modules.clone() {
            for target in module.target_names() {
                if modules.contains_key(&target) {
                    modules
                        .get_mut(&target)
                        .expect("Could not find module")
                        .register_input(&module);
                }
            }
        }
        let mut high_count = 0;
        let mut low_count = 0;
        for _ in 0..1000 {
            let mut signal_queue = VecDeque::from([Signal {
                from: "button".to_string(),
                to: "broadcaster".to_string(),
                pulse: Pulse::Low,
            }]);
            while let Some(signal) = signal_queue.pop_front() {
                // println!("Processing {:?}", signal);
                match signal.pulse {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                }
                if modules.contains_key(&signal.to) {
                    let module = modules.get_mut(&signal.to).unwrap();
                    let new_signals = module.handle_signal(signal);
                    signal_queue.extend(new_signals);
                }
            }
        }
        Some(high_count * low_count)
    }

    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    pub fn flip(&mut self) {
        *self = match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

impl From<bool> for Pulse {
    fn from(b: bool) -> Self {
        if b {
            Self::High
        } else {
            Self::Low
        }
    }
}

impl From<Pulse> for bool {
    fn from(p: Pulse) -> Self {
        match p {
            Pulse::High => true,
            Pulse::Low => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop((String, Vec<String>, Vec<String>, Pulse)),
    Conjunction((String, Vec<String>, HashMap<String, Pulse>)),
}

impl Module {
    pub fn name(&self) -> &str {
        match self {
            Self::Broadcaster(_) => "broadcaster",
            Self::FlipFlop((name, _, _, _)) => name,
            Self::Conjunction((name, _, _)) => name,
        }
    }

    pub fn target_names(&self) -> Vec<String> {
        match self {
            Self::Broadcaster(targets)
            | Self::FlipFlop((_, _, targets, _))
            | Self::Conjunction((_, targets, _)) => targets.clone(),
        }
    }

    pub fn register_input(&mut self, input: &Module) {
        match self {
            Self::Broadcaster(_) => {}
            Self::FlipFlop((_, inputs, _, _)) => {
                inputs.push(input.name().to_string());
            }
            Self::Conjunction((_, _, registry)) => {
                registry.insert(input.name().to_string(), Pulse::Low);
            }
        }
    }

    pub fn handle_signal(&mut self, signal: Signal) -> Vec<Signal> {
        let own_name = self.name().to_string();
        match self {
            Self::Broadcaster(targets) => targets
                .iter()
                .map(|t| Signal {
                    from: own_name.clone(),
                    to: t.clone(),
                    pulse: signal.pulse.clone(),
                })
                .collect(),
            Self::FlipFlop((name, _, targets, state)) => {
                if signal.pulse == Pulse::High {
                    return vec![];
                }
                state.flip();
                targets
                    .iter()
                    .map(|t| Signal {
                        from: name.clone(),
                        to: t.clone(),
                        pulse: state.clone(),
                    })
                    .collect()
            }
            Self::Conjunction((name, targets, inputs)) => {
                inputs.insert(signal.from, signal.pulse);
                if inputs.values().all(|v| v.clone().into()) {
                    targets
                        .iter()
                        .map(|t| Signal {
                            from: name.clone(),
                            to: t.clone(),
                            pulse: Pulse::Low,
                        })
                        .collect()
                } else {
                    targets
                        .iter()
                        .map(|t| Signal {
                            from: name.clone(),
                            to: t.clone(),
                            pulse: Pulse::High,
                        })
                        .collect()
                }
            }
        }
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, targets) = s.split_once(" -> ").unwrap();
        let targets = targets.split(", ").map(|s| s.to_string()).collect();

        Ok(match name {
            "broadcaster" => Self::Broadcaster(targets),
            name => match name.split_at(1) {
                ("%", name) => Self::FlipFlop((name.to_owned(), vec![], targets, Pulse::Low)),
                ("&", name) => Self::Conjunction((name.to_owned(), targets, HashMap::new())),
                _ => unreachable!("Invalid module"),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(20, true, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(20, false, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(20, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(20, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
