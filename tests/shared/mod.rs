// A pub module with all the mocks, dummies and assorted helpers for testing

// A mock UI to verify that printing commands send the right data to UI
pub mod mock_ui;

// A fake IO to verify that IO commands interact correctly with IO
pub mod fake_io;

// A dummy IO implementation that does nothing, for tests that don't use IO
pub mod dummy_io;

// All test fixtures
pub mod fixtures;
