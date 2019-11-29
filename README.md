# procs-soulinjector.md
- ⊕ [Get Started - PyO3 user guide](https://pyo3.rs/v0.6.0/)

## start
```sh
$ cargo build --release
$ cp target/release/libsoulinjector.dylib ./soulinjector.so

$ using bigdata
$ python

>>> import soulinjector
>>> soulinjector.sum_as_string(5,3)
'8'
>>> 
```

