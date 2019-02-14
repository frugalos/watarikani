# Watari Kani (or, Gazami Crab)
Remote version of [KaniLS](https://github.com/frugalos/kanils) based on [Cannyls_rpc](https://github.com/frugalos/cannyls_rpc)

# Available Commands
## List
Using the command `List`, we can obtain the list of all the lump ids in the given device `device`.
```
$ watarikani List --rpc-addr 127.0.0.1:14278 --device file0
listed.len() = 24
LumpId("00000000000000000000000000000000")
LumpId("00000000000000010000000000000000")
...
LumpId("01000000000100000000000000000001")
LumpId("01000000000101000000000000000001")
LumpId("01000000000102000000000000000001")
```

## Head
Using the command `Head`, we can check if the given lump id belongs to the given device.
```
$ watarikani Head --rpc-addr 127.0.0.1:14278 --device file0 --lumpid 01000000000101000000000000000001
LumpHeader { approximate_data_size: 33280 }

$ watarikani Head --rpc-addr 127.0.0.1:14278 --device file0 --lumpid 01000000000101000000000000000002
LumpId("01000000000101000000000000000002") does not exist
```

## Get
Using the command `Get`, we can get the content of the given lump id if it belongs to the given device.
```
$ watarikani Get --rpc-addr 127.0.0.1:14278 --device file_0 --lumpid 0100000000070600000000000000000e
[2, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 204, 94,
12, 11, 0, 5, 1, 0, 80, 142, 138, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0,
121, 111, 117, 114, 95, 111, 98, 106, 101, 99, 116, 95, 100, 97,
116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ... (omitted)

$ watarikani Get --rpc-addr 127.0.0.1:14278 --device file0 --lumpid 01000000000101000000000000000002
LumpId("01000000000101000000000000000002") does not exist
```

## Delete
Using the command `Delete`, we can delete the given lump id if it belongs to the given device.
```
$ watarikani Delete --rpc-addr 127.0.0.1:14278 --device file0 --lumpid 01000000000101000000000000000001
Removed LumpId("01000000000101000000000000000001")

$ watarikani List --rpc-addr 127.0.0.1:14278 --device file0
listed.len() = 23
LumpId("00000000000000000000000000000000")
LumpId("00000000000000010000000000000000")
LumpId("00000000000001000000000000000000")
...
LumpId("01000000000100000000000000000001")
LumpId("01000000000102000000000000000001")
```
