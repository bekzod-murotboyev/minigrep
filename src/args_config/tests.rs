use crate::args_config::ArgsConfig;

#[test]
fn test_build() {
    // GIVEN
    let args = vec![String::new(), "the".to_string(), "test.txt".to_string()];

    // WHEN
    let config = ArgsConfig::build(args);

    // THEN
    assert_eq!(
        config.unwrap(),
        ArgsConfig { query: "the".to_string(), file_path: "test.txt".to_string(), ignore_case: true}
    )
}