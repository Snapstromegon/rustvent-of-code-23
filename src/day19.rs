use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (workflows, parts) = input.split_once("\n\n")?;
        let workflows: HashMap<String, Workflow> = workflows
            .lines()
            .map(|w| w.parse().unwrap())
            .map(|w: Workflow| (w.name.clone(), w))
            .collect();
        let parts = parts
            .lines()
            .map(|p| p.parse().unwrap())
            .collect::<Vec<Part>>();
        Some(
            parts
                .iter()
                .filter(|p| p.is_accepted(&workflows))
                .map(|p| p.value())
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (workflows, _parts) = input.split_once("\n\n")?;
        let workflows: HashMap<String, Workflow> = workflows
            .lines()
            .map(|w| w.parse().unwrap())
            .map(|w: Workflow| (w.name.clone(), w))
            .collect();
        let ranges = workflows.get("in").unwrap().get_accepted_ranges(&workflows);
        Some(
            ranges
                .iter()
                .map(|(x, m, a, s)| {
                    (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
                })
                .sum(),
        )
    }
}

type Range = (usize, usize);

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn is_accepted(&self, workflow: &HashMap<String, Workflow>) -> bool {
        let mut curr_workflow = "in".to_owned();
        while curr_workflow != "A" && curr_workflow != "R" {
            let workflow = workflow.get(&curr_workflow).unwrap();
            curr_workflow = workflow.apply(self);
        }
        curr_workflow == "A"
    }

    pub fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s[1..s.len() - 1]
            .split(',')
            .map(|a| a.split('=').collect::<Vec<_>>()[1])
            .map(|a| a.parse().unwrap())
            .collect();
        Ok(Part {
            x: parts[0],
            m: parts[1],
            a: parts[2],
            s: parts[3],
        })
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn apply(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.action.clone();
            }
        }
        unreachable!(
            "No matching rule found for part {:?} in {:?}",
            part, self.rules
        )
    }

    pub fn get_accepted_ranges(
        &self,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<(Range, Range, Range, Range)> {
        let first = self.rules.first().unwrap();
        let res = match (&first.condition, first.action.as_str()) {
            (None, "A") => vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000))],
            (None, "R") => vec![],
            (None, x) => workflows.get(x).unwrap().get_accepted_ranges(workflows),
            (Some(c), x) => {
                let mut cloned = self.clone();
                cloned.rules.remove(0);
                let mut this_rule_ok = c.split(&match x {
                    "A" => vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000))],
                    "R" => vec![],
                    x => workflows.get(x).unwrap().get_accepted_ranges(workflows),
                });
                let after_rule_ok = c.invert().split(&cloned.get_accepted_ranges(workflows));
                this_rule_ok.extend(after_rule_ok);
                this_rule_ok
            }
        };
        res
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = s.split_once('{').unwrap();
        let rules = rules[..rules.len() - 1]
            .split(',')
            .map(|l| l.parse().unwrap())
            .collect();
        Ok(Workflow {
            name: name.to_string(),
            rules,
        })
    }
}

#[derive(Debug, Clone)]
struct Condition {
    attr: char,
    operator: char,
    value: usize,
}

impl Condition {
    pub fn matches(&self, part: &Part) -> bool {
        match self.operator {
            '<' => match self.attr {
                'x' => part.x < self.value,
                'm' => part.m < self.value,
                'a' => part.a < self.value,
                's' => part.s < self.value,
                _ => unreachable!(),
            },
            '>' => match self.attr {
                'x' => part.x > self.value,
                'm' => part.m > self.value,
                'a' => part.a > self.value,
                's' => part.s > self.value,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn invert(&self) -> Condition {
        match self.operator {
            '<' => Condition {
                attr: self.attr,
                operator: '>',
                value: self.value - 1,
            },
            '>' => Condition {
                attr: self.attr,
                operator: '<',
                value: self.value + 1,
            },
            _ => unreachable!(),
        }
    }

    pub fn split(
        &self,
        ranges: &[(Range, Range, Range, Range)],
    ) -> Vec<(Range, Range, Range, Range)> {
        ranges
            .iter()
            .map(|range| {
                let (mut low, mut high) = match self.attr {
                    'x' => range.0,
                    'm' => range.1,
                    'a' => range.2,
                    's' => range.3,
                    _ => unreachable!(),
                };
                match self.operator {
                    '>' => low = low.max(self.value + 1),
                    '<' => high = high.min(self.value - 1),
                    _ => unreachable!(),
                }
                if low < high {
                    match self.attr {
                        'x' => ((low, high), range.1, range.2, range.3),
                        'm' => (range.0, (low, high), range.2, range.3),
                        'a' => (range.0, range.1, (low, high), range.3),
                        's' => (range.0, range.1, range.2, (low, high)),
                        _ => unreachable!(),
                    }
                } else {
                    *range
                }
            })
            .collect()
    }
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('<') {
            let parts = s.split('<').collect::<Vec<_>>();
            Ok(Condition {
                attr: parts[0].chars().next().unwrap(),
                operator: '<',
                value: parts[1].parse().unwrap(),
            })
        } else if s.contains('>') {
            let parts = s.split('>').collect::<Vec<_>>();
            Ok(Condition {
                attr: parts[0].chars().next().unwrap(),
                operator: '>',
                value: parts[1].parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    action: String,
}

impl Rule {
    pub fn matches(&self, part: &Part) -> bool {
        if let Some(condition) = &self.condition {
            condition.matches(part)
        } else {
            true
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<_>>();
        if parts.len() == 1 {
            Ok(Rule {
                condition: None,
                action: parts[0].to_string(),
            })
        } else {
            Ok(Rule {
                condition: parts[0].parse().ok(),
                action: parts[1].to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        if let Some(input) = read_input(19, true, 1) {
            assert_eq!(Day.part1(&input), None);
        }
    }
    #[test]
    fn test_part1_challenge() {
        if let Some(input) = read_input(19, false, 1) {
            assert_eq!(Day.part1(&input), None);
        }
    }

    #[test]
    fn test_part2_example() {
        if let Some(input) = read_input(19, true, 2) {
            assert_eq!(Day.part2(&input), None);
        }
    }
    #[test]
    fn test_part2_challenge() {
        if let Some(input) = read_input(19, false, 2) {
            assert_eq!(Day.part2(&input), None);
        }
    }
}
