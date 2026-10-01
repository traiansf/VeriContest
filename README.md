# VeriContest — problems and solutions

A stripped-down copy of [VeriContest](https://hiprel-group.github.io/VeriContest/)
([arXiv:2605.08553](https://arxiv.org/abs/2605.08553)) that keeps only the
natural-language problem statements and the plain Rust solutions accepted by the
online judge. Specifications, Verus proofs, metadata (`tags`), testcases, and
tooling have been removed.

The solutions are grouped into one crate per source:

```text
leetcode/                 # library crate, 722 problems
  src/lib.rs              # declares one module per problem
  src/lc1004/
    description.md        # problem statement (also the module's rustdoc)
    mod.rs                # solution: `impl Solution { ... }`
codeforces/               # binary crate, 285 problems
  src/bin/cf1006C/
    description.md        # problem statement
    main.rs               # complete stdin/stdout program
```

Both the main benchmark problems and the `extended/` problems are included. The
extended set has the problems that accept more than one valid output or that use
`&mut` interfaces. `cf306A` appears in both sets with different solutions, so
the extended one is kept as `cf306A_alt`.

## Building and running

```sh
cargo build --workspace --release
printf "5\n1 3 1 1 4\n" | ./target/release/cf1006C   # prints 5
```

LeetCode solutions are plain functions. Call them as
`leetcode::lc1004::Solution::longest_ones(...)`, or browse the statements with
`cargo doc -p leetcode --open`.

## Differences from the benchmark's `code.rs`

The judge-accepted code was never stored separately. The benchmark's `code.rs`
already has the Verus-friendly form, and neither its git history nor the
[Hugging Face release](https://huggingface.co/datasets/Gax-c/VeriContest) keeps
another version. The solutions here are `code.rs` (LeetCode) and `main.rs`
(Codeforces) with these changes:

- Every LeetCode module starts with `#![doc = include_str!("description.md")]`
  and, when missing, `pub struct Solution;`. Codeforces programs only get the
  `doc` line.
- 16 LeetCode solutions called `vstd` helpers that don't exist in plain Rust.
  They were rewritten without changing the algorithm:
  - `s.as_str().unicode_len()` / `s.as_str().get_char(i)` became a
    `Vec<char>` collected once from the string, then `.len()` / `[i]`: lc242,
    lc520, lc551, lc944, lc1160, lc2038, lc2147, lc2483, lc2486, lc2546,
    lc2575, lc2914, lc2938, lc3803.
  - `v.set(i, x)` became `v[i] = x` (lc1160). In lc566 and lc2022, the
    clone-row, `set`, write-back sequence became `result[i][j] = x`.

  All 16 pass the benchmark's positive test cases (`testcases.jsonl`, 2,977
  cases).

## License

Code is Apache-2.0 ([LICENSE](LICENSE)). Benchmark artifacts are CC-BY-4.0
([DATA_LICENSE.md](DATA_LICENSE.md)). Problem statements come from LeetCode and
Codeforces and remain subject to their terms.

## Citation

```bibtex
@article{xie2026vericontest,
  title={VeriContest: A Competitive-Programming Benchmark for Verifiable Code Generation},
  author={Xie, Zichen and Pawagi, Mrigank and Liu, Yuxin and Rai, Aaditi and Shao, Lize and Berberian Jr, John and Che, Sicong and Wang, Wenxi},
  journal={arXiv preprint arXiv:2605.08553},
  year={2026}
}
```
