// A pub module with all the mocks, dummies and assorted helpers for testing
#![allow(unused)]

// A mock UI to verify that printing commands send the right data to UI
pub use add_ed::ui::mock_ui;

// A fake IO to verify that IO commands interact correctly with IO
pub use add_ed::io::fake_io;

// A dummy IO implementation that does nothing, for tests that don't use IO
pub use add_ed::io::dummy_io;

// A dummy UI that panics on everything except lock and unlock, verifies that
// tests that shouldn't cause a print don't.
pub use add_ed::ui::dummy_ui;

// All test fixtures
mod inner_fixture;
use inner_fixture::inner_fixture;
pub mod fixtures;
