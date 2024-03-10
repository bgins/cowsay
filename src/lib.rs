#[allow(warnings)]
mod bindings;

#[cfg(target_arch = "wasm32")]
use bindings::wasi::logging::logging::{log, Level};
use bindings::Guest;
use std::cmp;

const COW: &'static str = include_str!("cow.txt");

struct Component;

impl Guest for Component {
    fn say(message: String, width: u32) -> String {
        let said = say(message, width);

        #[cfg(target_arch = "wasm32")]
        log(Level::Info, "guest:cow:say", format!("{said}").as_str());

        said
    }
}

pub fn say(message: String, width: u32) -> String {
    let words: Vec<String> = message.split_whitespace().map(|s| s.to_string()).collect();
    let chunks = chunk_words(words, width);
    let largest_str = chunks.iter().map(|x| x.chars().count()).max();
    let width = match largest_str {
        Some(x) => cmp::min(width, x as u32),
        _ => width,
    };
    let formatted = match chunks.len() {
        1 => format!("< {} >\n", chunks.join(" ")),
        _ => multi_line(chunks, width),
    };
    let top_border = (0..width + 2).map(|_| "_").collect::<String>();
    let bottom_border = (0..width + 2).map(|_| "-").collect::<String>();

    format!(" {}\n{} {}\n{}", top_border, formatted, bottom_border, COW)
}

fn chunk_words(args: Vec<String>, max_size: u32) -> Vec<String> {
    let mut lines = Vec::with_capacity(args.len() * 2);
    let remainder: String = args.iter().fold(String::new(), |mut acc, arg| {
        if !acc.is_empty() {
            if (arg.chars().count() + 1) as u32 + acc.chars().count() as u32 <= max_size {
                return acc + " " + arg;
            }

            lines.push(acc.clone());
            acc.clear();
        }

        for c in arg.chars() {
            acc.push(c);
            if acc.chars().count() as u32 == max_size {
                lines.push(acc.clone());
                acc.clear();
            }
        }

        acc
    });

    if !remainder.is_empty() {
        lines.push(remainder);
    }

    lines
}

fn multi_line(lines: Vec<String>, width: u32) -> String {
    let total_length = lines.len() - 1;

    let formatted_lines = lines.iter().enumerate().map(|(idx, line)| {
        let current_length = line.clone().chars().count() as u32;
        let padding: String = (0..width - current_length).map(|_| ' ').collect();
        let (start, end) = match idx {
            0 => ('/', '\\'),
            _ if idx == total_length => ('\\', '/'),
            _ => ('|', '|'),
        };

        format!("{} {}{} {}\n", start, line, padding, end)
    });

    formatted_lines.collect()
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says() {
        assert_eq!(
            r#" _____________________
/ This is the best, I \
\ love being a cow!   /
 ---------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
"#,
            say("This is the best, I love being a cow!".to_string(), 20)
        )
    }
}
