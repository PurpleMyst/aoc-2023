use std::fmt::Display;

use rustc_hash::FxHashMap as HashMap;

type WorkflowId = &'static str;
type Value = u64;

const ACCEPT: WorkflowId = "A";
const REJECT: WorkflowId = "R";

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
        let final_destination = rules.next_back().unwrap().trim();
        let mut rules: Box<[Rule]> = rules
            .map(|rule| {
                let (cmp, dest) = rule.split_once(':').unwrap();
                let threshold = cmp[2..].parse::<Value>().unwrap();
                let prop = match cmp.as_bytes()[0] {
                    b'x' => Property::ExtremelyCoolLooking,
                    b'm' => Property::Musical,
                    b'a' => Property::Aerodynamic,
                    b's' => Property::Shiny,
                    _ => panic!("Invalid property"),
                };
                let cmp = match cmp.as_bytes()[1] {
                    b'<' => Comparison::LessThan,
                    b'>' => Comparison::GreaterThan,
                    _ => panic!("Invalid comparison operator"),
                };

                Rule {
                    property: prop,
                    comparison: cmp,
                    threshold,
                    send_to: dest,
                }
            })
            .collect();

            if rules.iter().all(|rule| rule.send_to == final_destination) {
                rules = Box::new([]);
            }

        (
            id,
            Self {
                rules,
                final_destination,
            },
        )
    }
}

fn is_accepted(workflows: &HashMap<WorkflowId, Workflow>, part: &Part) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let rule = workflow.rules.iter().find(|rule| rule.matches(part));
        let desetination = match rule {
            Some(rule) => rule.send_to,
            None => workflow.final_destination,
        };
        match desetination {
            ACCEPT => return true,
            REJECT => return false,
            _ => workflow = workflows.get(desetination).unwrap(),
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

    let workflow = workflows.get(workflow_id).unwrap();
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
        "in",
        HypotheticalPart {
            min: [1; 4],
            max: [4000; 4],
        },
    );

    (part1, part2)
}
