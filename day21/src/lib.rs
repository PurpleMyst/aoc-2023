use std::{
    collections::VecDeque,
    fmt::Display,
};


use ::bucket_queue::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const P1_TOTAL_STEPS: usize = 64;
const P2_TOTAL_STEPS: usize = 26_501_365;

fn do_solve_part1(map: &grid::Grid<bool>, start_pos: (usize, usize), total_steps: usize) -> usize {
    let mut q = BucketQueue::<Vec<_>>::new();
    q.push(start_pos, total_steps);
    let mut reachable = 0;
    let mut visited = HashSet::default();

    while let Some(remaining_steps) = q.max_priority() {
        let (x, y) = q.pop(remaining_steps).unwrap();
        let can_be_reached = remaining_steps % 2 == 0;
        if !visited.insert((x, y)) {
            continue;
        }

        if can_be_reached {
            reachable += 1;
        }

        if remaining_steps == 0 {
            continue;
        }

        let neighbors = [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ];
        neighbors
            .into_iter()
            .filter_map(|(x, y)| x.zip(y))
            .filter(|&(x, y)| !map.get(y, x).copied().unwrap_or(true))
            .for_each(|pos| q.push(pos, remaining_steps - 1))
    }

    reachable
}

fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

pub fn do_solve_part2(map: &grid::Grid<bool>, start_pos: (usize, usize), total_steps: usize) -> usize {
    let width = map.cols() as isize;
    let height = map.rows() as isize;

    let mut q = BucketQueue::<VecDeque<_>>::new();
    q.push_back((start_pos.0 as isize, start_pos.1 as isize), 0);

    let mut dist = HashMap::default();
    dist.insert((start_pos.0 as isize, start_pos.1 as isize), 0);

    let mut reachable = 0;

    let mut visited = HashSet::default();

    while let Some(steps_taken) = q.min_priority() {
        let (x, y) = q.pop_front(steps_taken).unwrap();
        visited.insert((x, y));

        if steps_taken % 2 == 0 {
            reachable += 1;
        }

        if steps_taken == total_steps {
            continue;
        }

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        neighbors
            .into_iter()
            .filter(|&(x, y)| {
                 !map[(usize::try_from(modulo(y, height)).unwrap(),usize::try_from(modulo(x, width)).unwrap())]
            })
            .filter(|&(x, y)| !visited.contains(&(x, y)))
            .for_each(|next| {
                let alt = steps_taken + 1;
                if dist.get(&next).map_or(true, |&d| d > alt) {
                    dist.insert(next, alt);
                    q.push_back(next, alt);
                }
            });
    }

    // draw reachable
    // let g = colorgrad::rd_yl_bu();
    // let x_min = visited.iter().map(|&(x, _)| x).min().unwrap();
    // let x_max = visited.iter().map(|&(x, _)| x).max().unwrap();
    // let y_min = visited.iter().map(|&(_, y)| y).min().unwrap();
    // let y_max = visited.iter().map(|&(_, y)| y).max().unwrap();
    // let img_w = (x_max - x_min + 1) as u32;
    // let img_h = (y_max - y_min + 1) as u32;
    // image::ImageBuffer::from_fn(img_w, img_h, |x, y| -> image::Rgb<u8> {
    //     let x = x as isize + x_min;
    //     let y = y as isize + y_min;
    //     if visited.contains(&(x, y)) {
    //         let dist = dist.get(&(x, y)).unwrap();
    //         let color = g.at(*dist as f64 / total_steps as f64);
    //         let [r, g, b, _a] = color.to_rgba8();
    //         image::Rgb([r, g, b])
    //     } else if map[(usize::try_from(modulo(y, height)).unwrap(),usize::try_from(modulo(x, width)).unwrap())] {
    //         image::Rgb([255, 255, 255])
    //     } else {
    //         image::Rgb([0, 0, 0])
    //     }
    // }).save(format!("day21_{total_steps}steps.png")).unwrap();

    reachable
}

pub fn load_input(input: &str) -> (grid::Grid<bool>, (usize, usize)) {
    let columns = input.lines().next().unwrap().len();
    let mut start_pos = None;
    let map = grid::Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.bytes())
            .enumerate()
            .map(|(idx, b)| match b {
                b'S' => {
                    start_pos = Some((idx / columns, idx % columns));
                    b'.'
                }
                b'#' | b'.' => b,
                _ => panic!(),
            })
            .map(|b| b == b'#')
            .collect(),
        columns,
    );
    let start_pos = start_pos.unwrap();
    (map, start_pos)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (map, start_pos) = load_input(input);
    println!("Map size: {}x{}", map.rows(), map.cols());

    for example in [6, 10, 50, 100, 500, 1000, 5000, P2_TOTAL_STEPS] {
        println!("Example {:4}: {}", example, do_solve_part2(&map, start_pos, example));
    }

    (do_solve_part1(&map, start_pos, P1_TOTAL_STEPS), 0)
}
