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

Five stages of the same benchmark, `gpu_sixteen3_tensor_kernel <n>`, n from
10^4 to 10^9, one `run_hosted_cmds.sh` boot per row so CUDA context/module
setup happens once and every number is the module's own internal `Instant`
timing:

`gpu_sixteen3_tensor_kernel_scaling_before_gpu_verify.csv` (stage 1, original)
  Verification was a single-threaded CPU loop reading every GPU result back
  and recomputing it against imasm_core. Single sample per row -- the
  10^4/10^5 points are noisy (cold GPU clocks, driver scheduling), not a
  clean trend; the finding that holds from 10^6 up is the ~30:1 ratio
  between that CPU loop and the GPU stage it was checking.

`gpu_sixteen3_tensor_kernel_scaling_gpu_verify_host_rng.csv` (stage 2)
  Verification moved onto the GPU: every thread also computes the same
  three stages via an independently-written flat kernel and folds any
  disagreement into one atomic counter per stage, O(N) work that never
  leaves the device. Input triples were still a host-side loop copied to
  the device (htod) -- that generation became the new bottleneck, ~89% of
  total time at n=10^9.

`gpu_sixteen3_tensor_kernel_scaling_gpu_verify_gpu_rng.csv` (stage 3)
  Input generation moved onto the GPU too: each thread derives its own x,
  y, z from its own index and a seed (a MurmurHash3-finalizer mix,
  counter-based -- no host loop, no htod of the inputs at all). x/y/z and
  the three stage-output arrays were still allocated and written at full n
  length even though only a fixed 10,000-entry control sample ever gets
  read back.

`gpu_sixteen3_tensor_kernel_scaling_gpu_verify_gpu_rng_k_inputs.csv` (stage 4)
  Device allocations switched from `alloc_zeros` to `alloc` for the six
  per-element buffers (every element is unconditionally overwritten before
  any read, so zeroing first was waste -- this alone was within measurement
  noise, not the win it looked like on paper). The regenerated x/y/z
  buffers shrunk from length n to length k=10,000, the actual number ever
  read back -- writing all n of them was 3*n bytes of global memory traffic
  for a 3*k-byte reader, and this one was real: consistently separated from
  stage 3's numbers, not noise.

`gpu_sixteen3_tensor_kernel_scaling.csv` (stage 5, current)
  stage1/stage2 (the intermediates phase_4 step 9 chains THROUGH on the way
  to the final result) shrunk from length n to length k too. Only stage3 --
  what phase_4 step 11 calls "the FINAL computed tensor," singular -- stays
  full length; it's this kernel's actual product. stage1/stage2 are still
  fully cross-checked on every one of the n threads, in registers, via the
  atomic counters -- that check never needed a global array to begin with.
  Real again: three repeated n=10^9 runs landed at 70-78ms, cleanly below
  stage 4's 101-119ms.

`plot_gpu_sixteen3_tensor_kernel_scaling.py` / `gpu_sixteen3_tensor_kernel_scaling.png`
  All five stages, and cumulative speedup vs stage 1: 425x at n=10^9.
  Below 10^5 the speedup is closer to 1-2x -- kernel-launch and
  atomic-counter overhead has little to amortize against at that size,
  shown rather than cropped out. Regenerate with `python3
  plot_gpu_sixteen3_tensor_kernel_scaling.py` from this directory.
