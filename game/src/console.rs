pub mod console {
    use std::io::{self, Write};

    pub(crate) fn output(args: std::fmt::Arguments) {
        print!("{}", args);
        io::stdout().flush().unwrap();
    }

    #[macro_export]
    macro_rules! console_output {
        ($($arg:tt)*) => {
            $crate::console::console::output(format_args!($($arg)*));
        };
    }

    pub fn read_input() -> String {
        const PROMPT: &str = "\n> ";
        output(format_args!("{}", PROMPT));
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
