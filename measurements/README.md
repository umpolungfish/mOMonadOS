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

`gpu_sixteen3_tensor_kernel_scaling.csv`
  `gpu_sixteen3_tensor_kernel <n>` run once per row, n from 10^4 to 10^9, one
  `run_hosted_cmds.sh` boot per row so CUDA context/module setup happens once
  and every number is the module's own internal `Instant` timing (generate,
  htod+launch+sync, dtoh, CPU verify against imasm_core), not shell wall-clock
  including boot. Single sample per row -- the 10^4/10^5 points are noisy
  (cold GPU clocks, driver scheduling), not a clean trend; the finding that
  holds from 10^6 up is the ~30:1 ratio between the CPU verification loop and
  the GPU stage it's checking.

`plot_gpu_sixteen3_tensor_kernel_scaling.py` / `gpu_sixteen3_tensor_kernel_scaling.png`
  The chart from that CSV: where the time goes at each scale, and throughput
  (triples/second) for the GPU stage alone against the end-to-end pipeline
  including the CPU check. Regenerate with `python3
  plot_gpu_sixteen3_tensor_kernel_scaling.py` from this directory.
