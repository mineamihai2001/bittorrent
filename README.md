# BitTorrent

## Description
BitTorrent is a peer-to-peer file sharing protocol used for distributing large amounts of data.
This project is an implementation of the BitTorrent protocol capable of downloading a publicly available file.

## How to run

```bash
# Generic command
$ ./runner.sh <command> <args>
```


### Decode command

```bash
# Generic decode command
$ ./runner.sh decode <args>
```

```bash
# Decoding a string
$ ./runner.sh decode <length>:<string>

# Example
$ ./runner.sh decode 5:hello
```


```bash
# Decoding an integer
$ ./runner.sh decode i<integer>e

# Example
$ ./runner.sh decode i420e
```

```bash
# Decoding a list
$ ./runner.sh decode l<list>e

# Example
$ ./runner.sh decode l5:helloi52ee
```



