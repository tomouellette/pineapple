# :pineapple: pineapple

`pineapple` is a simple rust command-line tool for downloading bio-imaging data. The tool is under development but datasets in the examples below should be downloadable.

## Installation

`pineapple` can be installed from source with `cargo`. 

```bash
cargo install --git https://github.com/tomouellette/pineapple
```

Given the tool is under development, we don't have precompiled binaries at this time.

## Downloading

Below we provide examples of how to download data with `pineapple`. You can also run the following to see available commands and flags.

```bash
pineapple download --help
```

### jump-cpg0016

```bash
pineapple download jump-cpg0016 -o images/ --compound KYRVNWMVYQXFEU-UHFFFAOYSA-N 
```

## Development

If you would like any additional features added, please open an issue. In the near term, aiming for ability to download all JUMP cell painting datasets and for downloading images directly into chunked zarr arrays. Dataset specific options such as correcting illumination using precomputed data may be added too.

