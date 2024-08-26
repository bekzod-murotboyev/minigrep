#[cfg(test)]
mod tests;
pub fn search<'a>(query: &str, content: &'a str, ignore_case: &bool) -> Vec<&'a str> {
    if !*ignore_case {
        return search_sensitive(&query, &content);
    }
    search_insensitive(query.to_lowercase(), &content)
}
fn search_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}
fn search_insensitive(lowercase_query: String, content: &str) -> Vec<&str> {
    content.lines().filter(|line| line.to_lowercase().contains(&lowercase_query)).collect()
}
