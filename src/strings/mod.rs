mod formatting;
mod grid_string;
mod parse;
mod tagged_string;
mod terminal_string;

pub use formatting::{DecoratedString, StringDecoration, StringDecorator, StringFormatting};
#[allow(deprecated)]
pub use grid_string::GridStringIterator;
pub use tagged_string::{
    Token, TokenIterator, wrap_line_count, wrap_string, wrap_tagged_line_count, wrap_tagged_string,
};
pub use terminal_string::{TerminalString, TerminalStringBuilder};
//pub use parse::{TerminalStringToken, parse_tokens};
