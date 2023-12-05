fn main() {
    color_eyre::install().unwrap();
    let (part1, part2) = day05::solve();
    println!("{part1}");
    println!("{part2}");
}
