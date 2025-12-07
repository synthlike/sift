# sift

sift is a simple tool for collecting function selectors from soldity source code.

This tool is a learning project so please don't expect anything :)

## Usage

sift can parse either a file or a directory. Two output formats are supported: tab-separated (defult) and json.

```
$ sift counter/src/Counter.sol
selector        signature                      file
0x3fb5c1cb      setNumber(uint256)             counter/src/Counter.sol
0xd09de08a      increment()                    counter/src/Counter.sol

$ sift -f json counter/src/Counter.sol
[
  {
    "selector": "0x3fb5c1cb",
    "signature": "setNumber(uint256)",
    "file": "counter/src/Counter.sol"
  },
  {
    "selector": "0xd09de08a",
    "signature": "increment()",
    "file": "counter/src/Counter.sol"
  }
]

### example: erc20

$ sift erc20.sol
selector        signature                             file
0xa9059cbb      transfer(address,uint256)             erc20.sol
0x095ea7b3      approve(address,uint256)              erc20.sol
0x23b872dd      transferFrom(address,address,uint256) erc20.sol
0x40c10f19      mint(address,uint256)                 erc20.sol
0x9dc29fac      burn(address,uint256)                 erc20.sol
```
