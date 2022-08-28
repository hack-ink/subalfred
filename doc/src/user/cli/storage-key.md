# The Storage Key Command
Calculate the storage key for the storage prefix/item quickly.

## Examples
```sh
subalfred storage-key --prefix System --item Number
```
```
0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac
```

Actually, it equals to `twox128(prefix) + twox128(item)`.

```sh
subalfred convert ascii2hex System
subalfred hash --hasher twox128 0x53797374656d
subalfred convert ascii2hex Number
subalfred hash --hasher twox128 0x4e756d626572
```
```
0x53797374656d
0x26aa394eea5630e07c48ae0c9558cef7
0x4e756d626572
0x02a5c1b19ab7a04f536c519aca4983ac

0x26aa394eea5630e07c48ae0c9558cef7
+
0x02a5c1b19ab7a04f536c519aca4983ac
=
0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac
```
