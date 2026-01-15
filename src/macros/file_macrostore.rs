//! File-based macro store implementation using YAML files
//!
//! This module provides a macro store that reads macros from YAML files.
//! Requires the `file_macrostore` feature to be enabled.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::Result;
use super::{Macro, MacroGetter};

/// A macro store that reads macros from a HashMap and falls back to files
///
/// This implementation first checks a HashMap for macros, and if not found,
/// attempts to read the macro from a file named `<macro-name>.macro` in the
/// user's configuration directory.
///
/// This implementation requires the `file_macrostore` feature to be enabled.
#[non_exhaustive]
pub struct MacroStore {
  /// Pre-loaded macros available for quick lookup
  macros: HashMap<String, Macro>,
  /// Configuration directory where macro files are stored
  config_dir: Option<PathBuf>,
}

/// Error type for macro store operations
#[derive(Debug)]
pub enum MacroStoreError {
  /// Failed to find or access configuration directory
  ConfigDirError,
  /// Failed to read macro file
  FileReadError(std::io::Error),
  /// Failed to parse macro YAML from file
  YamlParseError(serde_yaml::Error),
  /// Failed to validate macro structure
  ValidationError(String),
}

impl std::fmt::Display for MacroStoreError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MacroStoreError::ConfigDirError => write!(f, "Failed to find configuration directory"),
      MacroStoreError::FileReadError(e) => write!(f, "Failed to read macro file: {}", e),
      MacroStoreError::YamlParseError(e) => write!(f, "Failed to parse macro YAML: {}", e),
      MacroStoreError::ValidationError(msg) => write!(f, "Macro validation failed: {}", msg),
    }
  }
}

impl std::error::Error for MacroStoreError {}

impl crate::error::MacroErrorTrait for MacroStoreError {}

impl From<MacroStoreError> for crate::error::EdError {
  fn from(e: MacroStoreError) -> Self {
    Self::Macro(e.into())
  }
}

impl MacroStore {
  /// Create a new macro store with the given HashMap of pre-loaded macros
  ///
  /// The configuration directory must be provided as a parameter.
  pub fn new(macros: HashMap<String, Macro>, config_dir: Option<PathBuf>) -> Self {
    Self { macros, config_dir }
  }

  /// Read a macro from a YAML file
  ///
  /// Reads YAML representation of a Macro struct from a file named `<macro-name>.macro`
  fn read_macro_from_file(&self, name: &str) -> Result<Option<Macro>> {
    let config_dir = match &self.config_dir {
      Some(dir) => dir,
      None => return Err(MacroStoreError::ConfigDirError.into()),
    };

    let macro_file_path = config_dir.join(format!("{}.macro", name));
    
    // Try to read the file, return None if it doesn't exist
    let content = match std::fs::read_to_string(&macro_file_path) {
      Ok(content) => content,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
      Err(e) => return Err(MacroStoreError::FileReadError(e).into()),
    };

    // Parse the YAML content into a Macro
    match serde_yaml::from_str::<Macro>(&content) {
      Ok(macro_obj) => Ok(Some(macro_obj)),
      Err(e) => Err(MacroStoreError::YamlParseError(e).into()),
    }
  }
}

impl MacroGetter for MacroStore {
  fn get_macro(&self, name: &str) -> Result<Option<Cow<'_, Macro>>> {
    // First check the HashMap
    if let Some(macro_ref) = self.macros.get(name) {
      return Ok(Some(Cow::Borrowed(macro_ref)));
    }

    // If not found in HashMap, try to read from file
    match self.read_macro_from_file(name) {
      Ok(Some(macro_obj)) => Ok(Some(Cow::Owned(macro_obj))),
      Ok(None) => Ok(None),
      Err(e) => Err(e),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn macro_store_hashmap_lookup() {
    let mut macros = HashMap::new();
    macros.insert("test".to_string(), Macro::new("hello $1"));
    let store = MacroStore::new(macros, None);

    let result = store.get_macro("test").unwrap();
    assert!(result.is_some(), "Should find macro in HashMap");
  }

  #[test]
  fn macro_store_not_found() {
    let store = MacroStore::new(HashMap::new(), Some("/tmp".into()));
    let result = store.get_macro("nonexistent").unwrap();
    assert!(result.is_none(), "Should return None for nonexistent macro");
  }

  #[test]
  fn macro_store_yaml_parsing() {
    use crate::macros::{ModificationMode, NrArguments};
    let yaml_content = r#"
input: |
  hello $1 and $2
nr_arguments: 
  Exactly: 2
modification_mode: revert
"#;
    
    let result = serde_yaml::from_str::<Macro>(yaml_content);
    if let Err(ref e) = result {
      println!("YAML parse error: {:?}", e);
    }
    assert!(result.is_ok(), "Should parse macro YAML successfully");
    let macro_obj = result.unwrap();
    assert_eq!(macro_obj.input, "hello $1 and $2\n");
    assert!(matches!(macro_obj.nr_arguments, NrArguments::Exactly(2)));
    assert!(matches!(macro_obj.modification_mode, ModificationMode::Revert));
  }

  #[test]
  fn macro_store_yaml_simple() {
    let yaml_content = r#"
input: "simple macro content"
"#;
    
    let result = serde_yaml::from_str::<Macro>(yaml_content);
    if let Err(ref e) = result {
      println!("YAML parse error: {:?}", e);
    }
    assert!(result.is_ok(), "Should parse simple macro YAML successfully");
    let macro_obj = result.unwrap();
    assert_eq!(macro_obj.input, "simple macro content");
  }
}