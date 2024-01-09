# Metavis - A visual debugging tool for MetaDL

## Getting started

Build and run using

```bash
cargo run
```

or

```bash
cargo run -- <root>
```

or dowload the latest build from the _actions_ tab.

**NOTE**: rust in debug mode is much slower than in release mode.

## Limitations

- `unwrap()` used and therfore crashes are possible
- Some performance optimizations should be done
- Limited number of tests

## Usage

```bash
Usage: metavis <root>
  <root>    The directory where debug.json resides (with source files in the same directory)
```

where project is the directory where `debug.json` is present. This assumes that all files
`debug.json` refer to and source code are in the same directory as `debug.json`.

### Navigation

- Navigate the source and list using vim keybindings (`h`, `j`, `k` and `l`) or arrow keys
  - Left and right can also be used in the _tuple_ window
- Switching active pane is done with `Tab` or `Shift-Tab` or the key in the brackets (`[]`)
- Opening file selection window is done with `f`
- Close the program with `q` or `ctrl-c`
- Jump to a line using `:`
- Select an item in a list using `Enter`

# License

This repository is covered by the license BSD 2-clause, see file LICENSE.
