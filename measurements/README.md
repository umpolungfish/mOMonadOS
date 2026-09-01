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

`gpu_sixteen3_tensor_kernel_scaling_before_gpu_verify.csv`
  `gpu_sixteen3_tensor_kernel <n>` run once per row, n from 10^4 to 10^9, one
  `run_hosted_cmds.sh` boot per row, module's own internal `Instant` timing.
  The ORIGINAL module: verification was a single-threaded CPU loop reading
  every GPU result back and recomputing it against imasm_core. Single sample
  per row -- the 10^4/10^5 points are noisy (cold GPU clocks, driver
  scheduling), not a clean trend; the finding that holds from 10^6 up is the
  ~30:1 ratio between that CPU loop and the GPU stage it was checking.

`gpu_sixteen3_tensor_kernel_scaling.csv`
  The same sweep after moving verification onto the GPU: every thread also
  computes the same three stages via an independently-written flat kernel
  and folds any disagreement into one atomic counter per stage, O(N) work
  that never leaves the device. A fixed 10,000-entry sample (regardless of
  n) still gets checked directly against the real CPU imasm_core functions
  as the ground-truth control a GPU-only self-check cannot be.

`plot_gpu_sixteen3_tensor_kernel_scaling.py` / `gpu_sixteen3_tensor_kernel_scaling.png`
  Before/after: where the time goes at each scale, and the wall-clock
  speedup from moving the O(N) verify loop onto the GPU (roughly 5x from
  10^6 triples up; a real ~2x slowdown below 10^5, atomic contention with
  nothing to amortize it against, shown rather than cropped out). Host-side
  RNG generation of the input triples is the new bottleneck at scale, not
  verification. Regenerate with `python3
  plot_gpu_sixteen3_tensor_kernel_scaling.py` from this directory.
