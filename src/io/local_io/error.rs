use std::io::ErrorKind;

/// Error type for [`LocalIO`]
#[derive(Debug)]
pub enum LocalIOError {
  /// Permission denied when performing current operation on path.
  #[allow(missing_docs)]
  FilePermissionDenied{path: String},
  /// Path not found. May be failing to read file at path, or the directory to
  /// create a file in doesn't exist.
  #[allow(missing_docs)]
  FileNotFound{path: String},
  /// More general failure to perform file operation on path, as std::io::Error
  /// doesn't report all the details in a reasonably accessible way.
  #[allow(missing_docs)]
  FileIOFailed{path: String, error: std::io::Error},
  /// The child thread running the shell command couldn't be created.
  ChildCreationFailed(std::io::Error),
  /// The child thread running the shell command failed to begin execution.
  ChildFailedToStart(std::io::Error),
  /// The child thread running a shell command returned a non-zero integer
  ChildReturnedError(i32),
  /// The child thread running a shell command was killed by a signal
  ChildKilledBySignal,
  /// Error occured in the child thread handling piping
  ChildPipingError,
   /// Failed to convert data read from file or command into UTF8
  BadUtf8(std::string::FromUtf8Error),
}
impl LocalIOError {
  pub(super) fn file_error(path: &str, error: std::io::Error) -> Self {
    let path = path.to_owned();
    match error.kind() {
      ErrorKind::PermissionDenied => Self::FilePermissionDenied{path},
      ErrorKind::NotFound => Self::FileNotFound{path},
      _ => Self::FileIOFailed{path, error},
    }
  }
  pub(super) fn child_return_res(ret: Option<i32>) -> Self {
    match ret {
      Some(retval) => Self::ChildReturnedError(retval),
      None => Self::ChildKilledBySignal,
    }
  }
}

impl std::error::Error for LocalIOError {}
impl crate::error::IOErrorTrait for LocalIOError {}

impl std::fmt::Display for LocalIOError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::FilePermissionDenied{path} => { write!(f,
        "Permission denied, could not open file `{}`",
        path,
      )},
      Self::FileNotFound{path} => { write!(f,
        "Not found, could not open file `{}`",
        path,
      )},
      Self::FileIOFailed{path, error} => { write!(f,
        "Unknown error, could not open file `{}`.\nUnderlying error: {}",
        path,
        error,
      )},
      Self::ChildCreationFailed(e) => { write!(f,
        "Failed to create shell process.\nUnderlying error: {}",
        e,
      )},
      Self::ChildFailedToStart(e) => { write!(f,
        "Failed to start shell process.\nUnderlying error: {}",
        e,
      )},
      Self::ChildReturnedError(ret) => { write!(f,
        "Shell process returned non-success result: {}",
        ret,
      )},
      Self::ChildKilledBySignal => { write!(f,
        "Shell process was killed by a signal.",
      )},
      Self::ChildPipingError => { write!(f,
        "Error while piping data.",
      )},
      Self::BadUtf8(e) => { write!(f,
        "Bad UTF-8 in read data.\nUnderlying error: {}",
        e,
      )},
    }
  }
}

impl std::cmp::PartialEq for LocalIOError {
  fn eq(&self, other: &Self) -> bool {
    use LocalIOError::*;
    match (self, other) {
      (FilePermissionDenied{path: a},FilePermissionDenied{path: b}) => a == b,
      (FileNotFound{path: a},FileNotFound{path: b}) => a == b,
      (ChildReturnedError(a),ChildReturnedError(b)) => a == b,
      (ChildKilledBySignal,ChildKilledBySignal) => true,
      (ChildPipingError,ChildPipingError) => true,
      (BadUtf8(a),BadUtf8(b)) => a == b,
      // std::io::Error doesn't implement PartialEq, so we check the ErrorKind
      (FileIOFailed{path: a, error: b},FileIOFailed{path: c, error: d}) =>
        a == c && b.kind() == d.kind()
      ,
      (ChildCreationFailed(a),ChildCreationFailed(b)) => a.kind() == b.kind(),
      (ChildFailedToStart(a),ChildFailedToStart(b)) => a.kind() == b.kind(),
      _ => false,
    }
  }
}
