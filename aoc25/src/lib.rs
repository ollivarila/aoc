#[macro_export]
macro_rules! input {
    ($day:literal) => {
        const INPUT: &str = include_str!(concat!("../../input/", $day, ".txt"));
    };
}
