use std::collections::HashMap;

use pathfinding::directed::strongly_connected_components::strongly_connected_components_from;

use day20::*;

fn write_dot(name: &str, modules: &HashMap<&'static str, Module>) {
    use std::io::prelude::*;
    let mut modules_dot = std::fs::File::create(name).unwrap();
    writeln!(modules_dot, "digraph {{").unwrap();
    writeln!(modules_dot, "newrank=true").unwrap();

    let empty = Vec::new();
    let mut groups = strongly_connected_components_from(&"broadcaster", |&name| {
        modules
            .get(name)
            .map_or(empty.iter().copied(), |m| m.destinations.iter().copied())
    });

    writeln!(modules_dot, "broadcaster [shape=trapezium]").unwrap();
    for destination in modules.get("broadcaster").unwrap().destinations.iter() {
        writeln!(modules_dot, "broadcaster -> {destination} [color=blue]",).unwrap();
    }
    groups.retain(|group| !group.contains(&"broadcaster"));

    for module_names in groups {
        if module_names.len() != 1 {
            writeln!(modules_dot, "subgraph cluster_{} {{", module_names[0]).unwrap();
        }

        for &name in &module_names {
            let Some(module) = modules.get(name) else {
                continue;
            };
            let (shape, color) = match module.ty {
                ModuleType::Broadcaster => unreachable!(),
                ModuleType::FlipFlop(state) => ("ellipse", if state { "green" } else { "red" }),
                ModuleType::Nand(ref state) => ("hexagon", if !state.values().all(|&b| b) { "green" } else { "red" }),
            };
            writeln!(modules_dot, "{name} [shape={shape} color={color}]",).unwrap();

            for destination in module.destinations.iter() {
                writeln!(
                    modules_dot,
                    "{name} -> {destination} [color={} {}]",
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
                    },
                    match modules.get(destination) {
                        // Some(Module {
                        //     ty: ModuleType::Nand(..),
                        //     ..
                        // }) => "rank=max",
                        // None => "rank=sink",
                        _ => "",
                    }
                )
                .unwrap();
            }
        }
        if module_names.len() != 1 {
            writeln!(modules_dot, "}}").unwrap();
        }
    }

    writeln!(modules_dot, "{{ rank=sink; rx [shape=invtrapezium] }}",).unwrap();

    writeln!(modules_dot, "}}").unwrap();
}

fn main() {
    let modules = include_str!("../input.txt")
        .lines()
        .map(Module::parse)
        .collect::<HashMap<_, _>>();
    write_dot("modules.dot", &modules);
}
