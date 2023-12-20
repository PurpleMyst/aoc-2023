use std::{
    collections::VecDeque,
    fmt::Display
};

use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Nand(HashMap<&'static str, bool>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub ty: ModuleType,
    pub destinations: Box<[&'static str]>,
}

impl Module {
    pub fn parse(s: &'static str) -> (&'static str, Self) {
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

    pub fn is_nand(&self) -> bool {
        matches!(
            self,
            Self {
                ty: ModuleType::Nand(..),
                ..
            }
        )
    }

    pub fn is_flipflop(&self) -> bool {
        matches!(
            self,
            Self {
                ty: ModuleType::FlipFlop(..),
                ..
            }
        )
    }
}

// Part 1 is just straight simulation.
fn solve_part1(mut modules: HashMap<&'static str, Module>) -> usize {
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _step in 1..=1000usize {
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

// Part 2 involves noticing that the graph can be split into four 12-bit counters, each that reset and send a HIGH pulse
// to an output NAND gate when they reach a certain value. Each counter therefore has a specific cycle length that is,
// by construction of the input, prime; we can find the answer by multiplying the cycle lengths of each counter.
fn solve_part2(modules: &HashMap<&'static str, Module>) -> u64 {
    modules["broadcaster"]
        .destinations
        .iter()
        .map(|mut bit| {
            let mut period = 0u64;

            // Walk the graph until we reach the leaf flipflop, setting a bit on the period for each flipflop that must
            // be ON for the counter to reset.
            loop {
                let children = &*modules[bit].destinations;

                let should_count = children.iter().any(|c| modules[c].is_nand());
                if should_count {
                    period |= 1;
                }

                if let Some(next) = children.iter().find(|&c| modules[c].is_flipflop()) {
                    period <<= 1;
                    bit = next;
                } else {
                    break;
                }
            }

            // Due to how we iterated, we'll actually have the bits in reverse order, so we need to reverse them as a
            // 12-bit integer.
            period.reverse_bits() >> period.leading_zeros()
        })
        .product::<u64>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut modules = include_str!("input.txt")
        .lines()
        .map(Module::parse)
        .collect::<HashMap<_, _>>();

    // Each NAND gate needs to know which inputs are connected to it, so we'll add that information to the graph.
    let mut nand_inputs = HashMap::<&'static str, Vec<&'static str>>::default();
    for (name, module) in &modules {
        for destination in module.destinations.iter() {
            if modules.get(destination).map_or(false, |m| m.is_nand()) {
                nand_inputs.entry(destination).or_default().push(name);
            }
        }
    }
    for (name, inputs) in nand_inputs {
        let module = modules.get_mut(name).unwrap();
        if let ModuleType::Nand(ref mut state) = module.ty {
            for input in inputs {
                state.insert(input, false);
            }
        }
    }

    let part2 = solve_part2(&modules);

    (solve_part1(modules), part2)
}
