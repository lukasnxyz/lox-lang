#[macro_export]
macro_rules! red_text {
  ($text:expr) => {
    format!("\x1b[31m{}\x1b[0m", $text)
  };
}

#[macro_export]
macro_rules! error_indent {
  () => {
    format!("       ")
  };
}

// TODO: underline macro too when printing lexeme in error
