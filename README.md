# libkdbx-rs

Rust wrapper for the `libkdbx` library.

The library currently depends on the following other libraries:
* QT (The `QCore` components `QDateTime`, `QString`, `QUuid`, `QList`, and others are currently used by the core classes.
* Botan

## Installing

```
git submodule init
git submodule update
// TODO create the config-keepassx.h file in src/core
cargo build
```

## References
* https://gist.github.com/msmuenchen/9318327
