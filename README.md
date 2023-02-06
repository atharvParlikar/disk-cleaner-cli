# Simple Disk cleaner cli

dick-cleaner-cli is a tool written in rust that can detect duplicate files in your system and give an analysis of storage consumption by filetype.

## Installation

To install it you can simply clone the repository

```bash
git clone https://github.com/atharvParlikar/disk-cleaner-cli
```

## Usage

```bash
~/disk-cleanear-cli $ cargo r
enter command :> scan
enter path:
/home/username/
in                  1    -> 0Mb
ttf                 2    -> 4Mb
jsx                 7    -> 0Mb
rst                 275  -> 1Mb
rs                  2    -> 0Mb
yml                 9    -> 0Mb
gz                  4    -> 30Mb
.
.
.
// this is an example of scan command
// similarly there exists
// dup := to find duplicates
// getFiles := to list all files
// node_modules := get all the node modules path and storage analysis
// clear := clears the console
// exit := exit the cli
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
