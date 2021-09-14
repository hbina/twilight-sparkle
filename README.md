# twilight-sparkle

Parse stdin with expressions of that file type.

### Example

Lets say we want to see the value of questions in this [json file](./examples/test.json). We can simply,

```
twilight-sparkle on  master [?] is 📦 v0.1.0 via 🦀 v1.50.0
❯ cat ./examples/test.json | twilight-sparkle -t JSON -e "quiz.sport.q1.question"
Which one is correct team name in NBA?
```

### Help

```
twilight-sparkle 0.5.1
Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>
Perform queries on files

USAGE:
    tws [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --input-file <input-file>

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    json    Perform queries on JSON files
    toml    Perform queries on TOML files
    yaml    Perform queries on YAML files
```
