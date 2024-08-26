use std::env;

#[cfg(test)]
mod tests;
mod defaults;


pub struct ArgsConfig {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl ArgsConfig {
    pub fn build(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        Ok(ArgsConfig {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }

    pub fn query(&self) -> &String {
        return &self.query;
    }
    pub fn file_path(&self) -> &String {
        return &self.file_path;
    }

    pub fn ignore_case(&self) -> &bool {
        return &self.ignore_case;
    }
}