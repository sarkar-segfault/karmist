#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        eprintln!("{}", console::style(format!($($arg)+)).red().bold());
        std::process::exit(1);
    }}
}

mod crud;
mod options;

fn main() {
    let _opts: options::Options = argh::from_env();
}
