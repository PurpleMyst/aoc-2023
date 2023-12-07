macro_rules! doit {
    ($($day:ident: $solve:ident),+$(,)?) => {
        $(use $day::solve as $solve;)+
        iai::main!($($solve),+);
    };
}

doit!(
    day01: day01_solve,
    day02: day02_solve,
    day03: day03_solve,
    day04: day04_solve,
    day05: day05_solve,
    day06: day06_solve,
    day07: day07_solve,
);
