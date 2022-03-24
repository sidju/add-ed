# Todos:
- Handle accidentally opening a dir. (Diffcult until std::io::ErrorKind::IsADirectory stabilises.)
- 'J' to reflow to width? Maybe?
- Create buffer tests.
- Modify escape handling. Buffer is expected to handle \\ and \n, parse_expression allows escaping separator.
- Maybe macros soon? Command line flag, if we don't want to go for a config.
