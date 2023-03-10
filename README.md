## flt

Simple CLI Tool that helps you filter file's content

## Usage

Basic usage

```bash
flt <path/to/file> <STRING>
```

Need more than one string? No problem!

```bash
flt <path/to/file> <STRING> <STRING>
```

By default it doesn't remove empty lines, so if you want to to do so, just pass --nompt argument which stands for "No empty"

```bash
flt <path/to/file> <STRING> --nompt
```

Also, by default it creates a backup copy of your file before filtering it, if you want to disable it, just add --drop parameter

```bash
flt <path/to/file> <STRING> --drop
```

## Build on Linux and Mac

Make sure you have [Rust](https://www.rust-lang.org) and [just](https://github.com/casey/just) installed

```bash
# build flt
just build

#install
sudo just install

#uninstall
sudo just uninstall
```

## Build on Windows
```bash
# build flt
just build

#install
sudo just install-win

#uninstall
sudo just uninstall-win
```

## Build on Ligma

what's ligma?
