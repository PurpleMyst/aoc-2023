use std::{fmt::Display, collections::{HashMap, VecDeque}};

use owo_colors::OwoColorize;

enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Nand(HashMap<&'static str, bool>),
}

struct Module {
    ty: ModuleType,
    destinations: Box<[&'static str]>,
}

impl Module {
    fn parse(s: &'static str) -> (&'static str,Self) {
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
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut modules = include_str!("input.txt")
        .lines()
        .map(Module::parse)
        .collect::<HashMap<_, _>>();

    use std::io::prelude::*;
    let mut modules_dot = std::fs::File::create("modules.dot").unwrap();
    writeln!(modules_dot, "digraph {{").unwrap();
    for (name, module) in &modules {
        match module.ty {
            ModuleType::Broadcaster => writeln!(modules_dot, "{name} [shape=box]").unwrap(),
            ModuleType::FlipFlop(_) => writeln!(modules_dot, "{name} [shape=ellipse]").unwrap(),
            ModuleType::Nand(_) => writeln!(modules_dot, "{name} [shape=hexagon]").unwrap(),
        }

        for destination in module.destinations.iter() {
            writeln!(modules_dot, "{name} -> {destination} [color={}]", match modules.get(destination) {
                Some(Module { ty: ModuleType::Nand(..), .. }) => "red",
                Some(Module { ty: ModuleType::FlipFlop(..), .. }) => "blue",
                None => "green",
                _ => "black",
            }).unwrap();
        }
    }
    writeln!(modules_dot, "}}").unwrap();

    // let mut signals_log = std::fs::File::create("signals.log").unwrap();
    // let mut signals_log = std::io::stderr().lock();

    let mut con_inputs = HashMap::<&'static str, Vec<&'static str>>::new();
    for (name, module) in &modules {
        for destination in module.destinations.iter() {
            if matches!(modules.get(destination), Some(Module { ty: ModuleType::Nand(..), .. })) {
                con_inputs.entry(destination).or_default().push(name);
            }
        }
    }

    for (name, inputs) in con_inputs {
        let module = modules.get_mut(name).unwrap();
        if let ModuleType::Nand(ref mut state) = module.ty {
            for input in inputs {
                state.insert(input, false);
            }
        }
    }

    let mut low_pulses = 0usize;
    let mut high_pulses = 0usize;

    let mut part1 = 0;
    let mut part2 = 0;

    for step in 1.. {
        if step == 1000 {
            part1 = high_pulses * low_pulses;
        }

        // writeln!(signals_log, "\nSTEP {step:4}").unwrap();

    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster", false, "button"));
    while let Some((to, pulse, from)) = queue.pop_front() {
        if to == "rx" && !pulse {
            part2 = step;
            break;
        }

        match pulse {
            true => high_pulses += 1,
            false => low_pulses += 1,
        }

        // writeln!(signals_log, "{} -{}-> {}", from, (if pulse { "high" } else { "low" }), to).unwrap();
        let Some(module) = modules.get_mut(to) else { continue; };
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

    (part1,part2)
}
