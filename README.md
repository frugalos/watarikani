# Watari Kani (or, Gazami Crab)
Remote version of [KaniLS](https://github.com/frugalos/kanils) based on [Cannyls_rpc](https://github.com/frugalos/cannyls_rpc)

## List
`./watarikani List --addr 127.0.0.1:14278 --device file_0`

```
listed.len() = 294
LumpId("00000000000108000000000000000000")
LumpId("00000000000108010000000000000015")
...
LumpId("0100000000070600000000000000000d")
LumpId("0100000000070600000000000000000e")
```

## Head
`./watarikani Head --addr 127.0.0.1:14278 --device file_1 --lumpid 0100000000070600000000000000000e`

```
LumpHeader { approximate_data_size: 33280 }
```

`./watarikani Head --addr 127.0.0.1:14278 --device file_1 --lumpid 0100000000070600000000000000000f`

```
LumpId("0100000000070600000000000000000f") does not exist
```
