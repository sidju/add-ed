# Feature parity:
## Addressing:
- /regex/ (next matching)
- ?regex? (previous matching)
- '[tag]
- [address] +/- [offset] (currently only if no address)

## Commands:
- G interactive global (take command list for each match)
- v inverted global
- V inverted interactive global
- s substitute (Support repeating last substitution)
- u undo (Requires one-step-back buffer clone?)
- z "scroll"(What does it do? Test ed behaviour)
- ! shell escape, run input in shell and print result

# UI:
- Rewrite cli_ui into new structure
