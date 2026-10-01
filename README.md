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

Some LeetCode solutions call `vstd` helpers (`str::unicode_len`,
`str::get_char`, `Vec::set`). Those modules import small plain-Rust stand-ins
from `leetcode/src/verus_compat.rs`, so the solution bodies stay exactly as they
were.

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
