# Metavis - A visual debugging tool for MetaDL


## Getting started

Build and run using

```bash
cargo run
```

or 

```bash
cargo run -- <project>
```

## Limitations

Since this currently is a WIP these are the major limitations as of now:

* `unwrap()` used and therfore prone to crashes
* No performance optimizations
* Limited number of tests

## Usage

```bash
metavis <project>
```

where project is the directory where `debug.json` is present. This assumes that all files
`debug.json` refer to and source code are in the same directory as `debug.json`.

### Navigation

- Navigate the source and list using vim keybindings (`h`, `j`, `k` and `l`) or arrow keys
  - Left and right can also be used in the *diagnostic* window
- Switching active pane is done with `Tab`
- Opening file selection window is done with `f`
- Close the program with `q`

# License

This repository is covered by the license BSD 2-clause, see file LICENSE.
