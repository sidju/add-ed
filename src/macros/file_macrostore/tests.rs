// Real tests for the file-based macro store implementation
// Mocking these would defeat the point, so we risk side-effects instead,
// wherefore they are locked behind the features test_file_macrostore

use super::*;
use crate::macros::{ModificationMode, NrArguments};
use std::collections::HashMap;

#[test]
fn test_yaml_parsing_and_file_reading() {
  // Create a simple macro YAML file
  let simple_yaml = r#"input: |
  1,$p
"#;
  std::fs::write("simple.macro", simple_yaml).unwrap();
  
  // Create a macro store pointing to current directory
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  
  // Verify the macro is correctly parsed and returned
  let macro_result = store.get_macro("simple").unwrap();
  assert!(
    macro_result.is_some(),
    "Should find and parse macro from YAML file."
  );
  let macro_obj = macro_result.unwrap();
  assert_eq!(
    macro_obj.input,
    "1,$p\n",
    "Macro input should match the YAML content."
  );
  assert!(
    matches!(macro_obj.nr_arguments, NrArguments::Any),
    "Default nr_arguments should be Any."
  );
  assert!(
    matches!(macro_obj.modification_mode, ModificationMode::Default),
    "Default modification_mode should be Default."
  );
  
  // Cleanup
  std::fs::remove_file("simple.macro").unwrap();
}

#[test]
fn test_yaml_with_all_fields() {
  // Create a macro with all fields specified
  let full_yaml = r#"input: |
  s/$1/$2/g
  w
nr_arguments: !exactly 2
modification_mode: revert
"#;
  std::fs::write("substitute.macro", full_yaml).unwrap();
  
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  let macro_result = store.get_macro("substitute").unwrap();
  
  assert!(macro_result.is_some(), "Should find macro file.");
  let macro_obj = macro_result.unwrap();
  assert_eq!(
    macro_obj.input,
    "s/$1/$2/g\nw\n",
    "Multi-line input should be correctly parsed."
  );
  assert!(
    matches!(macro_obj.nr_arguments, NrArguments::Exactly(2)),
    "Should parse Exactly(2) for nr_arguments."
  );
  assert!(
    matches!(macro_obj.modification_mode, ModificationMode::Revert),
    "Should parse revert for modification_mode."
  );
  
  // Cleanup
  std::fs::remove_file("substitute.macro").unwrap();
}

#[test]
fn test_hashmap_overrides_file() {
  // Create a macro file
  let file_yaml = r#"input: |
  from file
"#;
  std::fs::write("override_test.macro", file_yaml).unwrap();
  
  // Create a macro store with a macro of the same name in the HashMap
  let mut macros = HashMap::new();
  macros.insert("override_test".to_string(), Macro::new("from hashmap\n"));
  let store = MacroStore::new(macros, Some(".".into()));
  
  // Verify that HashMap takes precedence
  let macro_result = store.get_macro("override_test").unwrap();
  assert!(macro_result.is_some(), "Should find macro.");
  let macro_obj = macro_result.unwrap();
  assert_eq!(
    macro_obj.input,
    "from hashmap\n",
    "HashMap macro should take precedence over file macro."
  );
  
  // Cleanup
  std::fs::remove_file("override_test.macro").unwrap();
}

#[test]
fn test_file_not_cached() {
  // Create initial macro file
  let first_content = r#"input: |
  first version
"#;
  std::fs::write("changing.macro", first_content).unwrap();
  
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  
  // Read the macro first time
  let first_result = store.get_macro("changing").unwrap();
  assert!(first_result.is_some(), "Should find macro file.");
  assert_eq!(
    first_result.unwrap().input,
    "first version\n",
    "First read should get first version."
  );
  
  // Modify the file
  let second_content = r#"input: |
  second version
"#;
  std::fs::write("changing.macro", second_content).unwrap();
  
  // Read the macro second time
  let second_result = store.get_macro("changing").unwrap();
  assert!(second_result.is_some(), "Should find macro file.");
  assert_eq!(
    second_result.unwrap().input,
    "second version\n",
    "Second read should get updated content - files must not be cached."
  );
  
  // Cleanup
  std::fs::remove_file("changing.macro").unwrap();
}

#[test]
fn test_nonexistent_macro() {
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  
  // Try to get a macro that doesn't exist
  let result = store.get_macro("doesnotexist").unwrap();
  assert!(
    result.is_none(),
    "Should return None for nonexistent macro file."
  );
}

#[test]
fn test_invalid_yaml() {
  // Create an invalid YAML file
  let invalid_yaml = r#"input: |
  test
nr_arguments: [this is invalid
"#;
  std::fs::write("invalid.macro", invalid_yaml).unwrap();
  
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  
  // Try to get the invalid macro
  let result = store.get_macro("invalid");
  assert!(
    result.is_err(),
    "Should return an error for invalid YAML."
  );
  
  // Cleanup
  std::fs::remove_file("invalid.macro").unwrap();
}

#[test]
fn test_no_config_dir() {
  let store = MacroStore::new(HashMap::new(), None);
  
  // Try to get a macro when no config dir is set
  let result = store.get_macro("anything");
  assert!(
    result.is_err(),
    "Should return ConfigDirError when config_dir is None."
  );
  
  // Verify it's the correct error type by checking the error message
  if let Err(e) = result {
    let error_string = format!("{}", e);
    assert!(
      error_string.contains("Failed to find configuration directory"),
      "Error should be about configuration directory"
    );
  }
}

#[test]
fn test_yaml_with_between_arguments() {
  // Create a macro with Between arguments
  let between_yaml = r#"input: |
  test macro
nr_arguments: !between
  incl_min: 1
  incl_max: 3
modification_mode: expose
"#;
  std::fs::write("between.macro", between_yaml).unwrap();
  
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  let macro_result = store.get_macro("between").unwrap();
  
  assert!(macro_result.is_some(), "Should find macro file.");
  let macro_obj = macro_result.unwrap();
  assert!(
    matches!(
      macro_obj.nr_arguments,
      NrArguments::Between{incl_min: 1, incl_max: 3}
    ),
    "Should parse Between arguments correctly."
  );
  assert!(
    matches!(macro_obj.modification_mode, ModificationMode::Expose),
    "Should parse expose modification mode."
  );
  
  // Cleanup
  std::fs::remove_file("between.macro").unwrap();
}

#[test]
fn test_yaml_with_none_arguments() {
  // Create a macro with None arguments
  let none_yaml = r#"input: |
  no args allowed
nr_arguments: !none
"#;
  std::fs::write("noargs.macro", none_yaml).unwrap();
  
  let store = MacroStore::new(HashMap::new(), Some(".".into()));
  let macro_result = store.get_macro("noargs").unwrap();
  
  assert!(macro_result.is_some(), "Should find macro file.");
  let macro_obj = macro_result.unwrap();
  assert!(
    matches!(macro_obj.nr_arguments, NrArguments::None),
    "Should parse None for nr_arguments."
  );
  
  // Cleanup
  std::fs::remove_file("noargs.macro").unwrap();
}
