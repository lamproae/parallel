use std::io::{self, Write, stderr, stdout};
use std::path::PathBuf;
use std::process::exit;

use super::super::tokenizer::TokenErr;

/// A list of all the possible errors that may happen when working with files.
#[derive(Debug)]
pub enum FileErr {
    DirectoryCreate(PathBuf, io::Error),
    DirectoryRead(PathBuf, io::Error),
    Create(PathBuf, io::Error),
    Open(PathBuf, io::Error),
    Read(PathBuf, io::Error),
    Remove(PathBuf, io::Error),
    Path,
    Write(PathBuf, io::Error),
}

/// The `InputIterator` may possibly encounter an error with reading from the unprocessed file.
#[derive(Debug)]
pub enum InputIteratorErr {
    FileRead(PathBuf, io::Error),
}

/// The error type for the argument module.
#[derive(Debug)]
pub enum ParseErr {
    /// An error occurred with accessing the unprocessed file.
    File(FileErr),
    /// The value of jobs was not set to a number.
    JobsNaN(String),
    /// No value was provided for the jobs flag.
    JobsNoValue,
    /// An invalid argument flag was provided.
    InvalidArgument(String),
    /// No arguments were given, so no action can be taken.
    NoArguments,
    /// There was an error with parsing tokens with the tokenizer.
    Token(TokenErr),
}

impl ParseErr {
    pub fn handle(self) -> ! {
        // Always lock an output buffer before using it.
        let stderr = stderr();
        let stdout = stdout();
        let mut stderr = stderr.lock();
        let stdout = &mut stdout.lock();
        let _ = stderr.write(b"parallel: parsing error: ");
        match self {
            ParseErr::File(file_err) => match file_err {
                FileErr::Create(path, why) => {
                    let _ = write!(stderr, "unable to create file: {:?}: {}\n", path, why);
                },
                FileErr::DirectoryCreate(path, why) => {
                    let _ = write!(stderr, "unable to create directory: {:?}: {}\n", path, why);
                },
                FileErr::DirectoryRead(path, why) => {
                    let _ = write!(stderr, "unable to create directory: {:?}: {}\n", path, why);
                },
                FileErr::Open(file, why) => {
                    let _ = write!(stderr, "unable to open file: {:?}: {}\n", file, why);
                },
                FileErr::Read(file, why) => {
                    let _ = write!(stderr, "unable to read file: {:?}: {}\n", file, why);
                },
                FileErr::Remove(file, why) => {
                    let _ = write!(stderr, "unable to remove file: {:?}: {}\n", file, why);
                },
                FileErr::Path => {
                    let _ = write!(stderr, "unable to obtain input paths\n");
                },
                FileErr::Write(file, why) => {
                    let _ = write!(stderr, "unable to write to file: {:?}: {}\n", file, why);
                },
            },
            ParseErr::JobsNaN(value) => {
                let _ = write!(stderr, "jobs parameter, '{}', is not a number.\n", value);
            },
            ParseErr::JobsNoValue => {
                let _ = stderr.write(b"no jobs parameter was defined.\n");
            },
            ParseErr::InvalidArgument(argument) => {
                let _ = write!(stderr, "invalid argument: {}\n", argument);
            },
            ParseErr::NoArguments => {
                let _ = write!(stderr, "no input arguments were given.\n");
            },
            ParseErr::Token(token_err) => match token_err {
                TokenErr::File(why) => {
                    let _ = write!(stderr, "unable to obtain Nth input: {}\n", why);
                },
                TokenErr::OutOfBounds => {
                    let _ = write!(stderr, "input token out of bounds\n");
                },
            }
        };
        let _ = stdout.write(b"For help on command-line usage, execute `parallel -h`\n");
        exit(1);
    }
}
