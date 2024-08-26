use std::fmt::{Debug, Display, Formatter};

use crate::args_config::ArgsConfig;

impl Display for ArgsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ArgsConfig[query={},file_path={},ignore_case={}]", self.query, self.file_path, self.ignore_case)
    }
}

impl Debug for ArgsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.to_string())
    }
}

impl PartialEq for ArgsConfig {
    fn eq(&self, other: &Self) -> bool {
        &self.query == other.query() &&
            self.file_path == other.file_path &&
            self.ignore_case == other.ignore_case
    }
}