// A pub module with all the mocks, dummies and assorted helpers for testing

// A mock UI to verify that printing commands send the right data to UI
pub mod mock_ui;

// A fake IO to verify that IO commands interact correctly with IO
pub mod fake_io;

// A dummy IO implementation that does nothing, for tests that don't use IO
pub mod dummy_io;

// A dummy UI that panics on everything except lock and unlock, verifies that
// tests that shouldn't cause a print don't.
pub mod dummy_ui;

// All test fixtures
mod inner_fixture;
use inner_fixture::inner_fixture;
pub mod fixtures;
