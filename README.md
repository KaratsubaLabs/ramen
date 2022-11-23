<div align="center">

<h1>ラーメン / ramen</h1>

static anime hosting site generator

</div>

**ramen** is the rust static site generator for からつばLABS hosted anime
and manga. **ramen** also has the ability to fetch metadata from myanimelist
using the jikan api.

## USAGE

```
$ ramen build [config directory]
```

## CONFIGURATION

**ramen** reads configuration from the `ramenrc` file:
```
site_url     =
files_url    =
content_path =
static_path  =
```

## BUILD

build **ramen** in debug mode:
```
$ make dev-build
```

build and install **ramen**:
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

- [ ] init and clean functions
- [ ] rewrite builder pattern (a lot of duplicated code)
- [ ] support decimal episode numbers (rust doesn't have Ord on f32?)
- [ ] error output on invalid configs
- [ ] better error handling in commands.rs
- [x] add conditional compilation to enable api metadata scrape features
