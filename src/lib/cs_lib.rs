mod naive_lib;
mod cs_trait;
mod cli_parse;

pub use cs_trait::EventTrigger;
pub use cli_parse::Opt;
pub use naive_lib::{NaiveStrategy, get_naive_strategy};

