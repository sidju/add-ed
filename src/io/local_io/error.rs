use std::io::ErrorKind;
#[derive(Debug)]
pub enum LocalIOError {
  FilePermissionDenied(String),
  FileNotFound(String),
  FileIOFailed((String, std::io::Error)),
  ChildCreationFailed(std::io::Error),
  ChildFailedToStart(std::io::Error),
  ChildReturnedError(i32),
  ChildKilledBySignal,
  // There is only one kind of error that the piping threads can return
  ChildPipingError,
  BadUtf8(std::string::FromUtf8Error),
}
impl LocalIOError {
  pub fn file_error(path: &str, e: std::io::Error) -> Self {
    let path = path.to_owned();
    match e.kind() {
      ErrorKind::PermissionDenied => Self::FilePermissionDenied(path),
      ErrorKind::NotFound => Self::FileNotFound(path),
      _ => Self::FileIOFailed((path, e)),
    }
  }
  pub fn child_return_res(ret: Option<i32>) -> Self {
    match ret {
      Some(retval) => Self::ChildReturnedError(retval),
      None => Self::ChildKilledBySignal,
    }
  }
}

impl std::error::Error for LocalIOError {}
impl crate::error::IOError for LocalIOError {}

impl std::fmt::Display for LocalIOError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::FilePermissionDenied(path) => { write!(f,
        "Permission denied, could not open file `{}`",
        path,
      )},
      Self::FileNotFound(path) => { write!(f,
        "Not found, could not open file `{}`",
        path,
      )},
      Self::FileIOFailed((path, e)) => { write!(f,
        "Unknown error, could not open file `{}`.\nUnderlying error: {}",
        path,
        e,
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
      (FilePermissionDenied(a),FilePermissionDenied(b)) => a == b,
      (FileNotFound(a),FileNotFound(b)) => a == b,
      (ChildReturnedError(a),ChildReturnedError(b)) => a == b,
      (ChildKilledBySignal,ChildKilledBySignal) => true,
      (ChildPipingError,ChildPipingError) => true,
      (BadUtf8(a),BadUtf8(b)) => a == b,
      // std::io::Error doesn't implement PartialEq, so we check the ErrorKind
      (FileIOFailed((a,b)),FileIOFailed((c,d))) => a == c && b.kind() == d.kind(),
      (ChildCreationFailed(a),ChildCreationFailed(b)) => a.kind() == b.kind(),
      (ChildFailedToStart(a),ChildFailedToStart(b)) => a.kind() == b.kind(),
      _ => false,
    }
  }
}
