## 2023-04-12, Version 6.0.0
### Commits

- [[`6a7bbb5272`](https://github.com/datrs/flat-tree/commit/6a7bbb5272b51754ce47ee12430c5221ad22aa7b)] Release 6.0.0 (Timo Tiuraniemi)
- [[`539ed6eb0d`](https://github.com/datrs/flat-tree/commit/539ed6eb0de7a1939c1c2ab65444cf23ecc47049)] Run CI on master branch instead of main (Timo Tiuraniemi)
- [[`694c6ef688`](https://github.com/datrs/flat-tree/commit/694c6ef688b61eefb338dce79d07fdbfee108141)] Switch from Travis to GHA (Timo Tiuraniemi)
- [[`597c8caa29`](https://github.com/datrs/flat-tree/commit/597c8caa29a047d05b3eb34dcc942c2579e8ade0)] Fix clippy warnings (Timo Tiuraniemi)
- [[`e707e790fb`](https://github.com/datrs/flat-tree/commit/e707e790fb0219d9516055c44711488962f12c53)] Move documentation to lib.rs for easier maintainability (Timo Tiuraniemi)
- [[`179a44d8bb`](https://github.com/datrs/flat-tree/commit/179a44d8bb1af953db5e7b60e8c32fb1d49557b8)] Fix Cargo.toml, bump edition to 2021 and, because of that, version to next major 6 (Timo Tiuraniemi)
- [[`27277762f6`](https://github.com/datrs/flat-tree/commit/27277762f62bb46ec18599775f2fab8713f6e986)] Use a new version number to avoid resolution mixups (Timo Tiuraniemi)
- [[`3c669d8c70`](https://github.com/datrs/flat-tree/commit/3c669d8c70ac42c5dfc5fa0ea46b9ec2a203a5e0)] Fix duplicated slashes in contains docs (Timo Tiuraniemi)
- [[`ec7a72ef14`](https://github.com/datrs/flat-tree/commit/ec7a72ef14f3cdf7460e87976f009960f8af5370)] Add Iterator::full_tree() (Timo Tiuraniemi)
- [[`8bc607394a`](https://github.com/datrs/flat-tree/commit/8bc607394af5303c229fe43326913ffc6ce41920)] Use a bigger tree in README to ease understanding of tests. Add a few more tests. (Timo Tiuraniemi)
- [[`3acd091d7e`](https://github.com/datrs/flat-tree/commit/3acd091d7e482299a430620e0f84a04451ffc4ad)] Add Iterator::next_tree() and Iterator::prev_tree() (Timo Tiuraniemi)
- [[`7048931423`](https://github.com/datrs/flat-tree/commit/7048931423738a7e3b3318ffe3cc625249b9ea79)] Add count_leaves(), Iterator::count_nodes() and Iterator::count_leaves() (Timo Tiuraniemi)
- [[`3b136873ce`](https://github.com/datrs/flat-tree/commit/3b136873ceedb3c6af5de6c00d5e59cfbce9bc6a)] Fix README picture which had confused indexes (Timo Tiuraniemi)
- [[`2bb6bf478d`](https://github.com/datrs/flat-tree/commit/2bb6bf47811d33051f3fe632c5736b4bacbf608a)] Add a contains() method to the iterator (Timo Tiuraniemi)
- [[`b1cb6ab914`](https://github.com/datrs/flat-tree/commit/b1cb6ab914e946cf3404769a782ec4442dc28e4d)] Update changelog (Bruno Tavares)

### Stats
```diff
  .github/CODE_OF_CONDUCT.md       |  75 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 .github/CONTRIBUTING.md          |  63 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 .github/ISSUE_TEMPLATE.md        |  41 +++++++++++++++++++++++++++++++++++++++++
 .github/PULL_REQUEST_TEMPLATE.md |  21 +++++++++++++++++++++
 .github/stale.yml                |  17 +++++++++++++++++
 .github/workflows/ci.yml         |  95 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 CERTIFICATE                      |  37 +++++++++++++++++++++++++++++++++++++
 CHANGELOG.md                     |  20 ++++++++++++++++++++
 Cargo.toml                       |  11 ++++++-----
 README.md                        |  61 ++-----------------------------------------------------------
 src/iterator.rs                  | 166 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 src/lib.rs                       | 103 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---------------
 tests/iterator.rs                |   8 ++++----
 13 files changed, 634 insertions(+), 84 deletions(-)
```


## 2020-03-03, Version 5.0.0
### Commits
- [[`05ffe8699d`](https://github.com/mafintosh/flat-tree-rs/commit/05ffe8699d452b01152a596e5cb0a652e7915503)] (cargo-release) version 5.0.0 (Bruno Tavares)
- [[`709b2d5635`](https://github.com/mafintosh/flat-tree-rs/commit/709b2d5635c4d2f9fdad48e23d55d257294dbdd9)] Merge pull request #44 from bltavares/usize-to-u64 (Bruno Tavares)
- [[`bc9c866bdb`](https://github.com/mafintosh/flat-tree-rs/commit/bc9c866bdb2101aa892b4bf917a8b1a30be1da45)] Fix travis script (Bruno Tavares)
- [[`68f7e67ed4`](https://github.com/mafintosh/flat-tree-rs/commit/68f7e67ed4c76fc6c11bb45ea78e9ec41b3d2449)] Change from usize to u64 (Bruno Tavares)
- [[`f9ca2ef68e`](https://github.com/mafintosh/flat-tree-rs/commit/f9ca2ef68e19a861117788bb6624e9529b1d6419)] Update criterion requirement from 0.2 to 0.3 (dependabot-preview[bot])
- [[`b23328579d`](https://github.com/mafintosh/flat-tree-rs/commit/b23328579d00947d6480c3e894e18e1598905204)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml     |  6 +++---
 CHANGELOG.md    | 15 +++++++++++++++
 Cargo.toml      |  4 ++--
 src/iterator.rs | 36 ++++++++++++++++++------------------
 src/lib.rs      | 34 +++++++++++++++++-----------------
 5 files changed, 55 insertions(+), 40 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


## 2016-03-09, Version v1.0.0
### Commits
- [[`b256517aef`](https://github.com/mafintosh/flat-tree-rs/commit/b256517aefb4ce2092e2920931d4c78cca6590af)] add badge (Mathias Buus)
- [[`017436cbae`](https://github.com/mafintosh/flat-tree-rs/commit/017436cbaec00ef57b7e8311a5f674fd0cf418e4)] add desc (Mathias Buus)
- [[`388d2445bc`](https://github.com/mafintosh/flat-tree-rs/commit/388d2445bc567148867a79f315aeee3ca492b100)] first commit (Mathias Buus)

### Stats
```diff
 Cargo.toml   |   2 +-
 rustfmt.toml |   2 +-
 src/lib.rs   | 328 ++++++++++++++++++++++++++----------------------------------
 3 files changed, 144 insertions(+), 188 deletions(-)
```


