use once_cell::sync::Lazy;

#[cfg(feature = "cat")]
pub mod cat;

#[derive(Debug, Clone, Copy)]
pub struct Ascii {
    pub big: [&'static str; 36],
    pub small: [&'static str; 34],
}

pub const ASCIIS: Lazy<Vec<Ascii>> = Lazy::new(|| {
    let asciis = vec![
        #[cfg(feature = "cat")]
        cat::ASCII,
    ];
    let mut result: Vec<Ascii> = Vec::new();

    if asciis.is_empty() {
        panic!("No ascii arts found, please enable at least one feature.");
    }

    for ascii in asciis.iter() {
        for i in 0..ascii.len() {
            result.push(ascii[i]);
        }
    }

    result
});

pub const SPACE: &str = "\x1b[49m                                               \x1b[m";

pub const COLORS: &str = "\x1b[38;5;8m\x1b[48;5;8m   \x1b[38;5;9m\x1b[48;5;9m   \x1b[38;5;10m\x1b[48;5;10m   \x1b[38;5;11m\x1b[48;5;11m   \x1b[38;5;12m\x1b[48;5;12m   \x1b[38;5;13m\x1b[48;5;13m   \x1b[38;5;14m\x1b[48;5;14m   \x1b[38;5;15m\x1b[48;5;15m   \x1b[0m";
