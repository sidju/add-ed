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
- k tag line (Requires tag support in buffer)
- s substitute (Support repeating last substitution)
- u undo (Requires one-step-back buffer clone?)
- x paste (requires a cut buffer)
- y copy (requires a cut buffer)
- z "scroll"(What does it do? Test ed behaviour)
- ! shell escape, run input in shell and print result
- # no-op command
- = print current selection

# UI:
- Rewrite cli_ui into new structure

# Buffer:
- Add blocking features to Buffer trait (see commands above)
- Implement features on VecBuffer
