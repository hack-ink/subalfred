# The Hash Command
Hash the data with the specify algorithm.

Available algorithms:
- blake2-128
- blake2-128-concat (default)
- blake2-256
- blake2-512
- twox64
- twox64-concat
- twox128
- twox256
- keccak256
- keccak512
- sha2-256

## Examples

### blake2-128-concat (Default)
```sh
subalfred hash 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0xe58b08eb4646f23e89bde2676c8e1feeb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```

### blake2-128
```sh
subalfred hash --hasher blake2-128 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0xe58b08eb4646f23e89bde2676c8e1fee
```

### blake2-256
```sh
subalfred hash --hasher blake2-256 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x83aa9248019a972a674ada58cfcb6b0dc85cc94c90691b1e93d3e71ded507d32
```

### blake2-512
```sh
subalfred hash --hasher blake2-512 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x7068368532a7b4a4dedf9994bfe584e4479304e8db003cee3658db7ad938add4ab2ce8d62e559e4de141227124d3b5b1d8d84d3e5e56084ea7804f871b3fde9d
```

### twox64
```sh
subalfred hash --hasher twox64 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x3ab053dc6b6a71e1
```

### twox64-concat
```sh
subalfred hash --hasher twox64-concat 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x3ab053dc6b6a71e1b4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```

### twox128
```sh
subalfred hash --hasher twox128 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x3ab053dc6b6a71e13f13e44949b8e388
```

### twox256
```sh
subalfred hash --hasher twox256 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x3ab053dc6b6a71e13f13e44949b8e388638838f880a0e3dc18e197b7f8efd6dc
```

### keccak256
```sh
subalfred hash --hasher keccak256 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x7774e3a14e4868bd5f3f6bd409685efd0e5179e783ed53d2a5da42c911efb05a
```

### keccak512
```sh
subalfred hash --hasher keccak512 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0xe7aeb316f9808098e7d71998d4642c784aecbdbbc03061fdcaec6992831bf6ae5abb5673af4f3dec4f4aeed543390ffeda9e0f8900f771f56043128177b1bcea
```

### sha2-256
```sh
subalfred hash --hasher sha2-256 0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747
```
```
0x5d0ae9ab625ce7c50e5ced079f414f28161b1b645df49679ebd794fc43c239bc
```

