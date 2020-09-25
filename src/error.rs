use std::{
    fmt,
    fmt::Display,
    io,
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
    string::FromUtf8Error,
};

use lf2_codec::{DecodeError, EncodeError};
use pest::iterators::{Pair, Pairs};

use crate::{
    BdyKindParseError, CPointKindParseError, EffectParseError, FrameNumber, ItrKindParseError,
    OPointKindParseError, ObjectData, Rule, StateParseError, WPointKindParseError,
};

#[derive(Debug)]
pub enum Error<'i> {
    /// Error while decoding a data file.
    DecodeError {
        /// Underlying `DecodeError`.
        error: DecodeError,
    },
    /// Error while decoding a data file.
    EncodeError {
        /// Underlying `EncodeError`.
        error: EncodeError,
    },
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
    FrameNumberNonUnique {
        /// `FrameNumber` that is used across multiple frames.
        frame_number: FrameNumber,
        /// Parsed `Pair`s of the frames with non-unique frame numbers.
        frame_pairs: Vec<Pair<'i, Rule>>,
    },
    /// Data file is not valid UTF8.
    DecodedDataInvalidUtf8(FromUtf8Error),
    /// Expected to parse object data, but got nothing.
    ObjectDataExpected,
    /// `ObjectData` is successfully parsed, but there is surplus data.
    ObjectDataSurplus {
        /// The successfully parsed `ObjectData`.
        object_data: ObjectData,
        /// Additional pairs.
        surplus_pairs: Pairs<'i, Rule>,
    },
    /// Pest could not parse the input with the object grammar.
    PestError(pest::error::Error<Rule>),
    /// A pair failed to parse as a `BdyKind`.
    ParseBdyKind {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `BdyKindParseError` from the parse attempt,
        error: BdyKindParseError,
    },
    /// A pair failed to parse as an `CPointKind`.
    ParseCPointKind {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `CPointKindParseError` from the parse attempt,
        error: CPointKindParseError,
    },
    /// A pair failed to parse as an `ItrKind`.
    ParseItrKind {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `ItrKindParseError` from the parse attempt,
        error: ItrKindParseError,
    },
    /// A pair failed to parse as an `Effect`.
    ParseItrEffect {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `EffectParseError` from the parse attempt,
        error: EffectParseError,
    },
    /// A pair failed to parse as an `OPointKind`.
    ParseOPointKind {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `OPointKindParseError` from the parse attempt,
        error: OPointKindParseError,
    },
    /// Failed to parse `opoint: action:` value as `FrameNumberNext`.
    ParseOPointAction {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `OPointKindParseError` from the parse attempt,
        error: ParseIntError,
    },
    /// A pair failed to parse as a `WPointKind`.
    ParseWPointKind {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `WPointKindParseError` from the parse attempt,
        error: WPointKindParseError,
    },
    /// Failed to parse `weaponact:` value as `FrameNumberNext`.
    ParseWeaponAct {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `ParseIntError` from the parse attempt,
        error: ParseIntError,
    },
    /// Failed to parse `attacking:` value as `WeaponStrengthIndex`.
    ParseWeaponStrengthIndex {
        /// The value that failed to be parsed.
        value_pair: Pair<'i, Rule>,
        /// The `ParseIntError` from the parse attempt,
        error: ParseIntError,
    },
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
    /// A pair failed to parse as a `Path`.
    ParsePath {
        /// Human readable name of the field.
        field: &'static str,
        /// The string that failed to be parsed into its value type.
        value_pair: Pair<'i, Rule>,
    },
    /// Frame element was built but returned with `None`.
    ///
    /// If this is reached, there is a bug in the `Element` object data parsing
    /// code.
    ElementBuildNone(Pair<'i, Rule>),
    /// Error should be unreachable based on the `lf2_object.pest` grammar.
    ///
    /// If this variant is hit, then there is a bug in either the grammar, or
    /// pest.
    GrammarSingle {
        /// The data rule that is valid in this position.
        rule_expected: Rule,
        /// The actual data rule.
        pair_found: Option<Pair<'i, Rule>>,
    },
    /// Error should be unreachable based on the `lf2_object.pest` grammar.
    ///
    /// If this variant is hit, then there is a bug in either the grammar, or
    /// pest.
    Grammar {
        /// The data rules that are valid in this position.
        rules_expected: &'static [Rule],
        /// The actual data rule.
        pair_found: Option<Pair<'i, Rule>>,
    },
    /// Expected a tag value, but got nothing.
    ///
    /// Error should be unreachable based on the `lf2_object.pest` grammar.
    ///
    /// If this variant is hit, then there is a bug in either the grammar, or
    /// pest.
    ValueExpected {
        /// Pair of the preceeding rule.
        tag_pair: Pair<'i, Rule>,
    },
    /// Errors when parsing a string as a `State`.
    StateParse {
        /// The string that failed to be parsed into the `State`.
        value_pair: Pair<'i, Rule>,
        /// The underlying error.
        error: StateParseError,
    },
    /// Variant that should not be reachable, such as through an `Infallible`
    /// error type..
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

impl<'e> From<FromUtf8Error> for Error<'e> {
    fn from(e: FromUtf8Error) -> Self {
        Self::DecodedDataInvalidUtf8(e)
    }
}

impl<'e> From<DecodeError> for Error<'e> {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError { error }
    }
}

impl<'e> From<EncodeError> for Error<'e> {
    fn from(error: EncodeError) -> Self {
        Self::EncodeError { error }
    }
}

impl<'i> std::error::Error for Error<'i> {}

impl<'i> Display for Error<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DecodeError { error } => write!(f, "{}", error),
            Self::EncodeError { error } => write!(f, "{}", error),
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
            Self::FrameNumberNonUnique {
                frame_number,
                frame_pairs,
            } => {
                write!(
                    f,
                    "Frame numbers must only be used once, but `{}` is used multiple times:
                    \n",
                    frame_number,
                )?;

                frame_pairs.iter().try_for_each(|frame_pair| {
                    write!(f, "- ")?;

                    if let Some(frame_first_line) = frame_pair.as_str().lines().next() {
                        write!(f, "`{}` ", frame_first_line)?;
                    }

                    let (line, col) = frame_pair.as_span().start_pos().line_col();
                    writeln!(f, "at position `{}:{}`", line, col)?;

                    Ok(())
                })?;

                writeln!(f)
            }
            Self::DecodedDataInvalidUtf8(e) => {
                writeln!(f, "Decoded object data is not valid UTF8.\n\
                    Try redownloading the object. If it doesn't work, then it likely cannot be used.\n\
                    Underlying error: {}", e)
            }
            Self::ObjectDataExpected => {
                write!(f, "Expected to parse object data, but got nothing.")
            }
            Self::ObjectDataSurplus {
                object_data: _,
                surplus_pairs,
            } => {
                writeln!(
                    f,
                    "Object data successfully parsed, but surplus pairs exist. Surplus:"
                )?;

                writeln!(f)?;
                writeln!(f, "{}", surplus_pairs)?;
                writeln!(f)?;

                Ok(())
            }
            Self::PestError(pest_error) => write!(f, "{}", pest_error),
            Self::ParseBdyKind { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `bdy: kind:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseCPointKind { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `cpoint: kind:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseItrKind { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `itr: kind:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseItrEffect { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `itr: effect:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseOPointKind { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `opoint: kind:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseOPointAction { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `opoint: action:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseWPointKind { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `wpoint: kind:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseWeaponAct { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `weaponact:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseWeaponStrengthIndex { value_pair, error } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `wpoint: attacking:` value `{}` at position: `{}:{}`. Error: `{}`.",
                    value_string, line, col, error
                )
            }
            Self::ParseFloat {
                field,
                value_pair,
                error,
            } => {
                let value_string = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse `{}` value `{}` at position: `{}:{}`. Error: `{}`.",
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
                    "Failed to parse `{}` value `{}` at position: `{}:{}`. Error: `{}`.",
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
            Self::ElementBuildNone(element_pair) => {
                let element_str = element_pair.as_str();
                let (line, col) = element_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "A frame element was built, but returned as `None`.\n\
                    File position: `{}:{}`\n\
                    \n\
                    ```\n\
                    {}\n\
                    ```\n\
                    \n\
                    This is likely a bug in `element.rs`.",
                    line, col, element_str,
                )
            }
            Self::GrammarSingle {
                rule_expected,
                pair_found,
            } => {
                write!(f, "Expected `{:?}`", rule_expected)?;

                if let Some(pair_found) = pair_found {
                    let rule = pair_found.as_rule();
                    let (line, col) = pair_found.as_span().start_pos().line_col();
                    write!(
                        f,
                        " at position: `{}:{}`, but grammar parsed a `{:?}`.\n",
                        line, col, rule,
                    )?;
                } else {
                    write!(f, ", but nothing is found.\n")?;
                }

                write!(
                    f,
                    "This means there is a bug where the subrule functions do not match the \
                    `lf2_object.pest` grammar."
                )
            }
            Self::Grammar {
                rules_expected,
                pair_found,
            } => {
                write!(f, "Expected one of `{:?}`", rules_expected)?;

                if let Some(pair_found) = pair_found {
                    let rule = pair_found.as_rule();
                    let (line, col) = pair_found.as_span().start_pos().line_col();
                    write!(
                        f,
                        " at position: `{}:{}`, but grammar parsed a `{:?}`.\n",
                        line, col, rule,
                    )?;
                } else {
                    write!(f, ", but nothing is found.\n")?;
                }

                write!(
                    f,
                    "This means there is a bug where the subrule functions do not match the \
                    `lf2_object.pest` grammar."
                )
            }
            Self::ValueExpected { tag_pair } => {
                let rule = tag_pair.as_rule();
                let (line, col) = tag_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Expected value for the `{:?}` tag at position: `{}:{}`, but nothing is found.\n",
                    rule, line, col
                )?;

                write!(
                    f,
                    "This means there is a bug where the subrule functions do not match the \
                    `lf2_object.pest` grammar."
                )
            }
            Self::StateParse { value_pair, error } => {
                let state_str = value_pair.as_str();
                let (line, col) = value_pair.as_span().start_pos().line_col();
                write!(
                    f,
                    "Failed to parse state `{}` at position: `{}:{}`. Error: `{}`.",
                    state_str, line, col, error
                )
            }
            Self::Unreachable { error } => write!(f, "Something is really wrong. Error: {}", error),
        }
    }
}
