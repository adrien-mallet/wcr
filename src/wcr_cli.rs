use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Debug, Parser)]
#[command(
    name = "wcr",
    version = "0.1.0",
    author = "adrien.mallet  <adrien.mallet@gmail.com>",
    about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.  A word is a non-zero-length sequence of characters delimited by white space.",
    long_about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.  A word is a non-zero-length sequence of characters delimited by white space.

       With no FILE, or when FILE is -, read standard input.

       The options below may be used to select which counts are printed, always in the following order: newline, word, character, byte, maximum line length."
)]
pub struct WcrCli {
    #[arg(
        name = "Words",
        short = 'w',
        long = "words",
        help = "Print Words count"
    )]
    words: bool,
    #[arg(
        name = "Lines",
        short = 'l',
        long = "lines",
        help = "Print lines count"
    )]
    lines: bool,
    #[arg(
        name = "Bytes",
        short = 'c',
        long = "bytes",
        help = "Print bytes count"
    )]
    bytes: bool,
    #[arg(name = "FILE")]
    paths: Option<Vec<PathBuf>>,
}

type WcrCliResult<T> = Result<T, Box<dyn Error>>;

impl WcrCli {
    pub fn run(&self) -> WcrCliResult<()> {
        if self.paths.is_some() {
            self.run_files()?;
        } else {
            self.run_stdin()?;
        }

        Ok(())
    }

    fn run_files(&self) -> WcrCliResult<()> {
        let Some(paths) = &self.paths else {
            panic!("Not paths provided");
        };

        for path in paths {
            match path {
                p if p.is_dir() => {
                    println!("is dir")
                }
                p if p.is_file() & self.lines & self.words & self.bytes => {
                    println!(
                        "{:4} {:4} {:4} {}",
                        count_lines(&path)?,
                        count_words(&path)?,
                        count_bytes(&path)?,
                        path.to_str().unwrap()
                    )
                }
                p if p.is_file() & self.lines & !self.words & !self.bytes => {
                    println!("{} {}", count_lines(&path)?, path.to_str().unwrap())
                }
                p if p.is_file() & !self.lines & self.words & !self.bytes => {
                    println!("{} {}", count_words(&path)?, path.to_str().unwrap())
                }
                p if p.is_file() & !self.lines & !self.words & self.bytes => {
                    println!("{} {}", count_bytes(&path)?, path.to_str().unwrap())
                }
                _ => panic!("Not implemented yet"),
            }
        }

        Ok(())
    }

    fn run_stdin(&self) -> WcrCliResult<()> {
        Ok(())
    }
}

fn count_lines(path: &PathBuf) -> WcrCliResult<usize> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines().count())
}

fn count_words(path: &PathBuf) -> WcrCliResult<usize> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines().fold(0, |agr, i| {
        agr + i
            .expect("Error unwrap line")
            .split(" ")
            .filter(|x| !x.is_empty())
            .count()
    }))
}

fn count_bytes(path: &PathBuf) -> WcrCliResult<usize> {
    let file = File::open(path)?;
    let mut buf = vec![];
    Ok(BufReader::new(file).read_to_end(&mut buf)?)
}
