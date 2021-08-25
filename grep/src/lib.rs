use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    // Patterns to match: match word or as complete line
    pattern_as_word: bool,

    // how to search for the pattern:
    match_case_insensitive: bool,

    // how to collect the result
    collect_invert: bool,

    // format output
    format_line_number: bool,
    format_file_name: bool,
}

impl Flags {
    // Valid flags: "-n"/"-l"/"-i"/"-v"/"-x"
    pub fn new(flags: &[&str]) -> Self {
        let mut res = Self::default();
        res.pattern_as_word = true;

        for fl in flags {
            match *fl {
                "-n" => res.format_line_number = true,
                "-l" => res.format_file_name = true,
                "-i" => res.match_case_insensitive = true,
                "-v" => res.collect_invert = true,
                "-x" => res.pattern_as_word = false,
                _ => println!("INVALID FLAG PASSED: {}", *fl),
            };
        }

        res
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    // Grep result for all flags expect '-l'
    let mut grep_result: Vec<String> = Vec::new();
    // Grep result for flag '-l'
    let mut grep_result_file: Vec<String> = Vec::new();

    for file in files {
        let mut temp: Vec<&str> = Vec::new();
        let mut result: Vec<String> = Vec::new();
        let mut line_numbers: Vec<u32> = Vec::new();
        let files_count = files.len();
        let mut current_line_number = 0;
        let mut idx = 0;

        let contents = fs::read_to_string(*file)?;
        if flags.pattern_as_word {
            temp = contents
                .lines()
                .filter(|line| {
                    current_line_number = current_line_number + 1;
                    if flags.match_case_insensitive {
                        if flags.collect_invert {
                            let matched = !line
                                .to_lowercase()
                                .as_str()
                                .contains(pattern.to_lowercase().as_str());
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        } else {
                            let matched = line
                                .to_lowercase()
                                .as_str()
                                .contains(pattern.to_lowercase().as_str());
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        }
                    } else {
                        if flags.collect_invert {
                            let matched = !line.contains(pattern);
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        } else {
                            let matched = line.contains(pattern);
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        }
                    }
                })
                .collect::<Vec<&str>>();

            result = temp
                .iter()
                .map(|line| {
                    let mut res = String::new();
                    if files_count > 1 {
                        res.push_str(*file);
                        res.push(':');
                    }
                    if flags.format_line_number {
                        res.push_str(line_numbers[idx].to_string().as_str());
                        res.push(':');
                        idx = idx + 1;
                    }
                    res.push_str(line);
                    res
                })
                .collect::<Vec<String>>();
        } else {
            temp = contents
                .lines()
                .filter(|line| {
                    current_line_number = current_line_number + 1;
                    if flags.match_case_insensitive {
                        if flags.collect_invert {
                            let matched =
                                !(line.to_lowercase().as_str() == pattern.to_lowercase().as_str());
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        } else {
                            let matched =
                                line.to_lowercase().as_str() == pattern.to_lowercase().as_str();
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        }
                    } else {
                        if flags.collect_invert {
                            let matched = !(*line == pattern);
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        } else {
                            let matched = *line == pattern;
                            if matched {
                                line_numbers.push(current_line_number);
                            }
                            matched
                        }
                    }
                })
                .collect::<Vec<&str>>();

            result = temp
                .iter()
                .map(|line| {
                    let mut res = String::new();
                    if files_count > 1 {
                        res.push_str(*file);
                        res.push(':');
                    }
                    if flags.format_line_number {
                        res.push_str(line_numbers[idx].to_string().as_str());
                        res.push(':');
                        idx = idx + 1;
                    }
                    res.push_str(line);
                    res
                })
                .collect::<Vec<String>>();
        }

        if flags.format_file_name && result.len() > 0 {
            grep_result_file.push(String::from(*file));
        }

        grep_result.append(&mut result);
    }

    if flags.format_file_name {
        return Ok(grep_result_file);
    }

    Ok(grep_result)
}
