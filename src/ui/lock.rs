use super::UI;

/// A simple object that locks UI and calls unlock_ui on it when being dropped
/// (Since it holds a mutable ref to UI its existence locks UI interaction)
pub struct UILock<'a> {
  inner: &'a mut dyn UI,
}
impl <'a> UILock<'a> {
  pub fn new(ui: &'a mut dyn UI) -> Self {
    Self{inner: ui}
  }
}
impl Drop for UILock<'_> {
  fn drop(&mut self) {
    self.inner.unlock_ui();
  }
}
