# VeriContest — problems and solutions

A stripped-down copy of [VeriContest](https://hiprel-group.github.io/VeriContest/)
([arXiv:2605.08553](https://arxiv.org/abs/2605.08553)) that keeps only the
natural-language problem statements and the plain Rust solutions accepted by the
online judge. Specifications, Verus proofs, metadata (`tags`), testcases, and
tooling have been removed.

This branch keeps only the 10 problems used in the model-effort experiments:
cf1669D, cf1772B, cf1921A, cf2070C, lc172, lc704, lc1406, lc1431, lc1572 and
lc3100.

The solutions are grouped into one crate per source:

```text
leetcode/                 # library crate, 6 problems
  src/lib.rs              # declares one module per problem
  src/lc1406/
    description.md        # problem statement (also the module's rustdoc)
    mod.rs                # solution: `impl Solution { ... }`
codeforces/               # binary crate, 4 problems
  src/bin/cf1921A/
    description.md        # problem statement
    main.rs               # complete stdin/stdout program
```

## Building and running

```sh
cargo build --workspace --release
./target/release/cf1921A < input.txt
```

LeetCode solutions are plain functions. Call them as
`leetcode::lc1406::Solution::stone_game_iii(...)`, or browse the statements with
`cargo doc -p leetcode --open`.

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
