
# Ramen

**ramen** is the rust static site generator for からつばLABS hosted anime
and manga.

## BUILD FOR DEVELOPMENT

build **ramen** in debug mode
```
$ make dev-build
```

## BUILD FOR PRODUCTION

compile and install **ramen**:
```
$ make build && sudo make install
```

## DIRECTORY STRUCTURE

**ramen** expects a specific directory structure for your content directory as
follows:
```
└── content
    ├── girls_last_tour
    │   ├── data
    │   │   └── metadata
    │   ├── files
    │   │   ├── 1.mp3
    │   │   ├── 2.mp3
    │   │   ├── 3.mp3
    │   │   └── ...
    │   └── subtitles
    │       ├── 1_en.ass
    │       ├── 1_ja.ass
    │       ├── 2_en.ass
    │       ├── 2_ja.ass
    │       ├── 3_en.ass
    │       ├── 3_ja.ass
    │       └── ...
    └── ...
```

## TODO

- [ ] rewrite builder pattern (a lot of duplicated code)
- [ ] support decimal episode numbers (rust doesn't have Ord on f32?)
- [ ] error output on invalid configs
- [ ] better error handling in commands.rs
