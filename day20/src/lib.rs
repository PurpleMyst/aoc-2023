use std::{collections::VecDeque, fmt::Display};

use indexmap::IndexMap as HashMap;

use pathfinding::directed::strongly_connected_components::strongly_connected_components_from;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Nand(HashMap<&'static str, bool>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    ty: ModuleType,
    destinations: Box<[&'static str]>,
}

impl Module {
    fn parse(s: &'static str) -> (&'static str, Self) {
        let (lhs, rhs) = s.split_once(" -> ").unwrap();
        let destinations: Box<[_]> = rhs.split(", ").collect();
        let (ty, name) = if lhs == "broadcaster" {
            (ModuleType::Broadcaster, "broadcaster")
        } else {
            let (ty_marker, name) = lhs.split_at(1);
            let ty = match ty_marker {
                "%" => ModuleType::FlipFlop(false),
                "&" => ModuleType::Nand(Default::default()),
                _ => panic!("Invalid module type {ty_marker:?}"),
            };
            (ty, name)
        };
        (name, Self { ty, destinations })
    }

    fn is_nand(&self) -> bool {
        matches!(
            self,
            Self {
                ty: ModuleType::Nand(..),
                ..
            }
        )
    }

    fn is_flipflop(&self) -> bool {
        matches!(
            self,
            Self {
                ty: ModuleType::FlipFlop(..),
                ..
            }
        )
    }
}

fn do_part1(mut modules: HashMap<&'static str, Module>) -> usize {
    let mut low_pulses = 0usize;
    let mut high_pulses = 0usize;

    for _ in 1..=1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster", false, "button"));
        while let Some((to, pulse, from)) = queue.pop_front() {
            match pulse {
                true => high_pulses += 1,
                false => low_pulses += 1,
            }

            let Some(module) = modules.get_mut(to) else {
                continue;
            };
            match module.ty {
                ModuleType::Broadcaster => {
                    for destination in module.destinations.iter() {
                        queue.push_back((destination, pulse, "broadcaster"));
                    }
                }
                ModuleType::FlipFlop(ref mut state) => {
                    if !pulse {
                        *state = !*state;
                        for destination in module.destinations.iter() {
                            queue.push_back((destination, *state, to));
                        }
                    }
                }
                ModuleType::Nand(ref mut state) => {
                    state.insert(from, pulse);
                    let c_pulse = !state.values().all(|&b| b);

                    for destination in module.destinations.iter() {
                        queue.push_back((destination, c_pulse, to));
                    }
                }
            }
        }
    }

    high_pulses * low_pulses
}

fn write_dot(name: &str, modules: &HashMap<&'static str, Module>, groups: &[Vec<&'static str>]) {
    use std::io::prelude::*;
    let mut modules_dot = std::fs::File::create(name).unwrap();
    writeln!(modules_dot, "digraph {{").unwrap();

    for module_names in groups {
        writeln!(modules_dot, "subgraph cluster_{} {{", module_names[0]).unwrap();

        for name in module_names {
            let Some(module) = modules.get(name) else {
                continue;
            };
            let (shape, color) = match module.ty {
                ModuleType::Broadcaster => ("box", "black"),
                ModuleType::FlipFlop(state) => ("ellipse", if state { "green" } else { "red" }),
                ModuleType::Nand(ref state) => ("hexagon", if !state.values().all(|&b| b) { "green" } else { "red" }),
            };
            writeln!(modules_dot, "{name} [shape={shape} color={color}]",).unwrap();

            for destination in module.destinations.iter() {
                writeln!(
                    modules_dot,
                    "{name} -> {destination} [color={}]",
                    match modules.get(destination) {
                        Some(Module {
                            ty: ModuleType::Nand(..),
                            ..
                        }) => "red",
                        Some(Module {
                            ty: ModuleType::FlipFlop(..),
                            ..
                        }) => "blue",
                        None => "green",
                        _ => "black",
                    }
                )
                .unwrap();
            }
        }
        writeln!(modules_dot, "}}").unwrap();
    }
    writeln!(modules_dot, "}}").unwrap();
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut modules = include_str!("input.txt")
        .lines()
        .map(Module::parse)
        .collect::<HashMap<_, _>>();
    let mut nand_inputs = HashMap::<&'static str, Vec<&'static str>>::new();
    for (name, module) in &modules {
        for destination in module.destinations.iter() {
            if matches!(
                modules.get(destination),
                Some(Module {
                    ty: ModuleType::Nand(..),
                    ..
                })
            ) {
                nand_inputs.entry(destination).or_default().push(name);
            }
        }
    }

    // each destination of the broadcaster defines one cycle
    let foo: u64 = modules["broadcaster"]
        .destinations
        .iter()
        .map(|mut bit| {
            let mut period = 0;
            let mut value = 1;

            loop {
                let children = &modules[bit].destinations;

                let should_count = children.iter().any(|c| modules[c].is_nand());
                if should_count {
                    period += value;
                }

                if let Some(next) = children.iter().find(|&c| modules[c].is_flipflop()) {
                    value *= 2;
                    bit = next;
                } else {
                    break;
                }
            }

            period
        })
        .product();

    for (name, inputs) in nand_inputs {
        let module = modules.get_mut(name).unwrap();
        if let ModuleType::Nand(ref mut state) = module.ty {
            for input in inputs {
                state.insert(input, false);
            }
        }
    }

    (do_part1(modules), foo)
}
