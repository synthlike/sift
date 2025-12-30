# sift

sift is a simple CLI tool for extracting function selectors from Solidity source code.  
It can be used to maintain a local database of function selectors parsed from Solidity code.

Support for SQLite and Vyper is planned for future releases.

**Note:** This is a personal learning project. While functional, it may not cover every edge case in the Solidity grammar.

## Installation

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
git clone https://github.com/synthlike/sift.git
cd sift
cargo build --release
```

The binary will be located at `./target/release/sift`.

## Usage

You can run `sift` against a single file or a directory.

```bash
$ ./sift assets/erc20.sol

selector    signature
0xa9059cbb  transfer(address,uint256)
0x095ea7b3  approve(address,uint256)
0x23b872dd  transferFrom(address,address,uint256)
0x40c10f19  mint(address,uint256)
0x9dc29fac  burn(address,uint256)
```

Use the `--json` flag to get a structured array, useful for piping into tools like `jq`.

```bash
$ ./sift --json assets/erc20.sol
```

```json
[
  {
    "selector": "0xa9059cbb",
    "signature": "transfer(address,uint256)"
  },
  {
    "selector": "0x095ea7b3",
    "signature": "approve(address,uint256)"
  },
  {
    "selector": "0x23b872dd",
    "signature": "transferFrom(address,address,uint256)"
  },
  {
    "selector": "0x40c10f19",
    "signature": "mint(address,uint256)"
  },
  {
    "selector": "0x9dc29fac",
    "signature": "burn(address,uint256)"
  }
]
```

Sift can also be run against a directory of Solidity files, and duplicates can be removed using jq.

```bash
$ ./sift --json assets | jq 'unique_by(.selector)'
```
