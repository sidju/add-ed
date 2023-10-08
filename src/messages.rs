/// Help text, generally printed when "help" is entered as a command.
pub const COMMAND_LIST: &str = include_str!("../COMMANDS_SHORT.md");
/// Longer help text/manual, generally printed when "Help" is entered as a command.
pub const COMMAND_DOCUMENTATION: &str = include_str!("../COMMANDS.md");

/// Printed when 'h' command is called and no error has occured yet.
pub const NO_ERROR: &str = "No errors recorded.";
/// Printed when 'f' command is called and no default path is yet set.
pub const NO_FILE: &str = "No default file currently set.";
