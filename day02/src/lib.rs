use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let games = input.lines().map(|game| {
        let (id, reveals) = game.split_once(": ").unwrap();

        (
            id.split_once(' ').unwrap().1.parse::<u32>().unwrap(),
            reveals.split(";").map(|reveal| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for s in reveal.trim().split(", ") {
                    let (n, color) = s.split_once(' ').unwrap();
                    let n = n.parse::<u8>().unwrap();
                    match color {
                        "red" => red = n,
                        "green" => green = n,
                        "blue" => blue = n,
                        _ => panic!("Unknown color {}", color),
                    }
                }
                (red, green, blue)
            }),
        )
    });

    let part1 = games
        .clone()
        .filter_map(|(id, mut reveals)| reveals.all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14).then_some(id))
        .sum::<u32>();

    let part2 = games
        .map(|(_id, reveals)| {
            let (r, g, b) = reveals.fold((0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| {
                (r1.max(r2), g1.max(g2), b1.max(b2))
            });
            u32::from(r) * u32::from(g) * u32::from(b)
        })
        .sum::<u32>();

    (part1, part2)
}
