use std::io::{self, BufRead};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// Reads input in from stdin.  This tool is designed to have its input piped in from stdin via `tail -f` of the log file.
/// Blocks for the life of the program in a different thread.  Returns a receiver for the input lines.
pub fn read_lines() -> Receiver<String> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let stdin = io::stdin();

        // read input lines from stdin and send them through the channel.
        for line_res in stdin.lock().lines() {
            let line = match line_res {
                Ok(line) => line,
                Err(err) => {
                    println!("Error reading line from stdin: {:?}", err);
                    continue;
                },
            };

            // it's possible for some of these things to not actually be lines, so split them at newlines.
            let strings: Vec<&str> = line.split('\n').collect();

            for s in strings {
                tx.send(s.into()).expect("Unable to send message through the channel");
            }
        }
    });

    rx
}
