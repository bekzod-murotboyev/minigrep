use super::*;

#[test]
fn test_search_case_sensitive() {
    // GIVEN
    let query = "duct";
    let content = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    // WHEN
    let result = search(query, content,&true);
    // THEN
    assert_eq!(result, vec!["safe, fast, productive."])
}

#[test]
fn test_search_case_insensitive(){
    // GIVEN
    let query = "rUsT";
    let content = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    // WHEN
    let result = search(query, content,&false);
    // THEN
    assert_eq!(result, vec!["Rust:","Trust me."])
}