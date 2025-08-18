#[macro_export]
macro_rules! red_text {
    ($text:expr) => {
        format!("\x1b[31m{}\x1b[0m", $text)
    };
}
