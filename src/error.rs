use std::{
    fmt,
    fmt::Display,
    io,
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
};

use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug)]
pub enum Error<'i> {
    /// Failed to open data file from the file system.
    FileOpenError {
        /// Path that was attempted to be opened as a file.
        path: PathBuf,
        /// The `io::Error` returned by the OS.
        io_error: io::Error,
    },
    /// Failed to read data from a data file.
    FileReadError {
        /// Path to the file that was attempted to be read.
        path: PathBuf,
        /// The `io::Error` returned by the OS.
        io_error: io::Error,
    },
    /// Pest could not parse the input with the object grammar.
    PestError(pest::error::Error<Rule>),
    /// A pair failed to parse as a float.
    ParseFloat {
        /// Human readable name of the field.
        field: &'static str,
        /// The string that failed to be parsed into its value type.
        value_pair: Pair<'i, Rule>,
        /// The `ParseFloatError` from the parse attempt,
        error: ParseFloatError,
    },
    /// A pair failed to parse as an integer.
    ParseInt {
        /// Human readable name of the field.
        field: &'static str,
        /// The string that failed to be parsed into its value type.
        value_pair: Pair<'i, Rule>,
        /// The `ParseIntError` from the parse attempt,
        error: ParseIntError,
    },
    /// A pair failed to parse as an integer.
    ParsePath {
        /// Human readable name of the field.
        field: &'static str,
        /// The string that failed to be parsed into its value type.
        value_pair: Pair<'i, Rule>,
    },
    /// Unused?
    ObjectDataExpected(Pair<'i, Rule>),
    /// The `derive_builder::Builder::build()` method failed with the given
    /// message.
    DataBuildFailed(String),
    /// Error should be unreachable based on the `lf2_object.pest` grammar.
    ///
    /// If this variant is hit, then there is a bug in either the grammar, or
    /// pest.
    Grammar {
        /// The expected data rule.
        rule_expected: Rule,
        /// The actual data rule.
        pair_found: Option<Pair<'i, Rule>>,
    },
    Unreachable {
        /// This should really be unreachable, e.g. the `Error` type is
        /// `Infallible` during parsing.
        error: Box<dyn std::error::Error>,
    },
}

impl<'i> From<pest::error::Error<Rule>> for Error<'i> {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self::PestError(error)
    }
}

impl<'i> std::error::Error for Error<'i> {}

impl<'i> Display for Error<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileOpenError { path, io_error } => write!(
                f,
                "Failed to open file: `{}`. Error: {}",
                path.display(),
                io_error
            ),
            Self::FileReadError { path, io_error } => write!(
                f,
                "Failed to read file: `{}`. Error: {}",
                path.display(),
                io_error
            ),
            Self::PestError(pest_error) => write!(f, "{}", pest_error),
            Self::ParseFloat {
                field,
                value_pair,
                error,
            } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse {} value `{}` at position: `{}:{}`. Error: `{}`.",
                    field, value_string, line, col, error
                )
            }
            Self::ParseInt {
                field,
                value_pair,
                error,
            } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse {} value `{}` at position: `{}:{}`. Error: `{}`.",
                    field, value_string, line, col, error
                )
            }
            Self::ParsePath { field, value_pair } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse {} value `{}` at position: `{}:{}`.",
                    field, value_string, line, col
                )
            }
            Self::ObjectDataExpected(pair) => {
                let rule = pair.as_rule();
                let (line, col) = pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Expected object data at position: `{}:{}`, but found: `{:?}`.",
                    line, col, rule
                )
            }
            Self::DataBuildFailed(message) => write!(f, "{}", message),
            Self::Grammar {
                rule_expected,
                pair_found,
            } => {
                write!(f, "Expected `{:?}` as the next data element", rule_expected)?;

                if let Some(pair_found) = pair_found {
                    let rule = pair_found.as_rule();
                    let (line, col) = pair_found.as_span().start_pos().line_col();
                    write!(
                        f,
                        " at position: `{}:{}`, but found a `{:?}`. ",
                        line, col, rule,
                    )?;
                } else {
                    write!(f, ", but nothing is found. ")?;
                }

                write!(f, "This is an error in the `lf2_object.pest` grammar.")
            }
            Self::Unreachable { error } => write!(f, "Something is really wrong. Error: {}", error),
        }
    }
}
