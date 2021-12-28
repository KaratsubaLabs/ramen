
# Ramen

**ramen** is the rust static site generator for からつばLABS hosted anime
and manga.

## USAGE

compile and run **ramen**:
```
$ cargo build
$ ./target/debug/ramen
```

## TODO

- [ ] rewrite builder pattern (a lot of duplicated code)
- [ ] support decimal episode numbers (rust doesn't have Ord on f32?)
- [ ] error output on invalid configs
- [ ] better error handling in commands.rs
