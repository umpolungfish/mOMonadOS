# measurements — the inputs that produced a reported number

Kept because a measurement whose inputs are gone is a claim, not a measurement.

`imasm_write_60_tuples.cmds`
  Sixty tuples from `bip39_tuples_10000.json` as `imasm write` lines. Run through
  `./run_serial_cmds.sh` the kernel writes each tuple to its twelve-glyph word.

`banked_basis_words60.txt`
  The sixty words that came back.

`banked_60_words.cmds`
  Those words as `banked` lines. Result: 42 VACUOUS, 18 fired — the `banked`
  check fires on 30% of key-type words, which is the base rate the 10,000-key
  survey's §5 never took before calling one firing in four an outlier.

`banked_80_keylifts.cmds`
  The same check over 72-mark base-12 lifts of forty private and forty public
  keys. A different object from the twelve-glyph type words, and it is kept apart
  for that reason: 36 of 80 vacuous there, and that rate does NOT transfer.
