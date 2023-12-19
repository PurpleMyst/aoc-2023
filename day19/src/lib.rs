use std::fmt::Display;

use rustc_hash::FxHashMap as HashMap;

type WorkflowId = u32;
type Value = u64;

const fn str2id(s: &[u8]) -> WorkflowId {
    match s.len() {
        1 => s[0] as WorkflowId,
        2 => (s[0] as WorkflowId) << 8 | (s[1] as WorkflowId),
        3 => (s[0] as WorkflowId) << 16 | (s[1] as WorkflowId) << 8 | (s[2] as WorkflowId),
        _ => unimplemented!(),
    }
}

const ACCEPT: WorkflowId = str2id(b"A");
const REJECT: WorkflowId = str2id(b"R");
const INITIAL_WORKFLOW: WorkflowId = str2id(b"in");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Property {
    ExtremelyCoolLooking, // b'x'
    Musical,              // b'm'
    Aerodynamic,          // b'a'
    Shiny,                // b's'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    properties: [Value; 4],
}

impl Part {
    fn parse(s: &str) -> Self {
        let mut this = Self { properties: [0; 4] };
        s.strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .zip(this.properties.iter_mut())
            .for_each(|(val, prop)| *prop = val.split_once('=').unwrap().1.parse::<Value>().unwrap());
        this
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    property: Property,
    comparison: Comparison,
    threshold: Value,
    send_to: WorkflowId,
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        let val = part.properties[self.property as usize];
        match self.comparison {
            Comparison::LessThan => val < self.threshold,
            Comparison::GreaterThan => val > self.threshold,
        }
    }

    fn parse(rule: &str) -> Self {
        let (comparison, send_to) = rule.split_once(':').unwrap();
        let threshold = comparison[2..].parse::<Value>().unwrap();
        let property = match comparison.as_bytes()[0] {
            b'x' => Property::ExtremelyCoolLooking,
            b'm' => Property::Musical,
            b'a' => Property::Aerodynamic,
            b's' => Property::Shiny,
            _ => panic!("Invalid property"),
        };
        let comparison = match comparison.as_bytes()[1] {
            b'<' => Comparison::LessThan,
            b'>' => Comparison::GreaterThan,
            _ => panic!("Invalid comparison operator"),
        };

        Rule {
            property,
            comparison,
            threshold,
            send_to: str2id(send_to.as_bytes()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    rules: Box<[Rule]>,
    final_destination: WorkflowId,
}

impl Workflow {
    fn parse(s: &'static str) -> (WorkflowId, Self) {
        let (id, rules) = s.strip_suffix('}').unwrap().split_once('{').unwrap();
        let mut rules = rules.split(',');
        let final_destination = str2id(rules.next_back().unwrap().as_bytes());
        let mut rules: Box<[Rule]> = rules.map(Rule::parse).collect();

        if rules.iter().all(|rule| rule.send_to == final_destination) {
            rules = Box::new([]);
        }

        (
            str2id(id.as_bytes()),
            Self {
                rules,
                final_destination,
            },
        )
    }
}

fn is_accepted(workflows: &HashMap<WorkflowId, Workflow>, part: &Part) -> bool {
    let mut workflow = workflows.get(&INITIAL_WORKFLOW).unwrap();
    loop {
        let rule = workflow.rules.iter().find(|rule| rule.matches(part));
        let send_to = match rule {
            Some(rule) => rule.send_to,
            None => workflow.final_destination,
        };
        match send_to {
            ACCEPT => return true,
            REJECT => return false,
            _ => workflow = workflows.get(&send_to).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HypotheticalPart {
    min: [Value; 4],
    max: [Value; 4],
}

fn do_part2(workflows: &HashMap<WorkflowId, Workflow>, workflow_id: WorkflowId, mut part: HypotheticalPart) -> Value {
    if workflow_id == ACCEPT {
        return part
            .max
            .iter()
            .zip(part.min.iter())
            .map(|(max, min)| max - min + 1)
            .product();
    } else if workflow_id == REJECT {
        return 0;
    }

    let workflow = workflows.get(&workflow_id).unwrap();
    let mut answer = 0;

    for rule in workflow.rules.iter() {
        let idx = rule.property as usize;
        match rule.comparison {
            Comparison::LessThan => {
                // min..minimum(max, threshold) is the bit that matches
                // max..threshold is the bit that doesn't match
                let matching_part = (part.min[idx] < rule.threshold).then(|| {
                    let mut new_part = part;
                    new_part.max[idx] = new_part.max[idx].min(rule.threshold - 1);
                    new_part
                });

                let non_matching_part = (part.max[idx] >= rule.threshold).then(|| {
                    let mut new_part = part;
                    new_part.min[idx] = rule.threshold;
                    new_part.max[idx] = part.max[idx];
                    new_part
                });

                if let Some(matching_part) = matching_part {
                    answer += do_part2(workflows, rule.send_to, matching_part);
                }

                if let Some(non_matching_part) = non_matching_part {
                    part = non_matching_part;
                } else {
                    break;
                }
            }
            Comparison::GreaterThan => {
                let matching_part = (part.max[idx] > rule.threshold).then(|| {
                    let mut new_part = part;
                    new_part.min[idx] = new_part.min[idx].max(rule.threshold + 1);
                    new_part
                });

                let non_matching_part = (part.min[idx] <= rule.threshold).then(|| {
                    let mut new_part = part;
                    new_part.min[idx] = part.min[idx];
                    new_part.max[idx] = rule.threshold;
                    new_part
                });

                if let Some(matching_part) = matching_part {
                    answer += do_part2(workflows, rule.send_to, matching_part);
                }

                if let Some(non_matching_part) = non_matching_part {
                    part = non_matching_part;
                } else {
                    break;
                }
            }
        }
    }

    answer += do_part2(workflows, workflow.final_destination, part);

    answer
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (workflows, parts) = include_str!("input.txt").split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(Workflow::parse).collect();

    let part1 = parts
        .lines()
        .map(Part::parse)
        .filter(|part| is_accepted(&workflows, part))
        .map(|part| part.properties.into_iter().sum::<Value>())
        .sum::<Value>();

    let part2 = do_part2(
        &workflows,
        INITIAL_WORKFLOW,
        HypotheticalPart {
            min: [1; 4],
            max: [4000; 4],
        },
    );

    (part1, part2)
}
