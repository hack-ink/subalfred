# The Convert Command
A set of really useful convertors.

## Ascii to Hex
Convert the ascii to hex.

### Examples
```sh
subalfred convert ascii2hex AurevoirXavier
```
```
0x41757265766f6972586176696572
```

## Bytes Style
Convert the bytes between several different styles.

### Examples

#### From Vec String to Byte String Literal
```sh
subalfred convert bytes-style --from vec-string --to byte-string-literal "[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]"
```
```
AurevoirXavier
```
```sh
subalfred convert bytes-style --from vec-string --to byte-string-literal "[1, 1, 1, 1]"
```
```
\x01\x01\x01\x01
```

#### From Byte String Literal to Vec String
```sh
subalfred convert bytes-style --from byte-string-literal --to vec-string "AurevoirXavier"
[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]
```
```sh
subalfred convert bytes-style --from byte-string-literal --to vec-string "\x01\x01\x01\x01"
```
```
[1, 1, 1, 1]
```



## Bytes to Hex
Convert the bytes to hex.

### Examples
```sh
subalfred convert bytes2hex "[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]"
```
```
0x41757265766f6972586176696572
```


## Hex to Bytes
Convert the hex to bytes.

### Examples
```sh
subalfred convert hex2bytes 0x41757265766f6972586176696572
```
```
[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]
```
