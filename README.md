# tws

Parse stdin with expressions of that file type.

### Example

Lets say we want to see the value of questions in this [json file](./examples/test.json). We can simply,

```
twilight-sparkle on  master [?] is 📦 v0.1.0 via 🦀 v1.50.0
❯ cat ./examples/test.json | tws -t JSON -e "quiz.sport.q1.question"
Which one is correct team name in NBA?
```

### Todo

For the moment, this is only implemented for JSON and with very basic expression. It should be possible to do this for any kind of input, even HTTP requests, arbitrary HTML data etc etc.

### Help

```
twilight-sparkle 0.1.0
Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>


USAGE:
    tws [OPTIONS] --expression <expression> --type <type>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --expression <expression>    Expression to evaluate the input with
    -i, --input <input>              Input file. If not specified, will read from stdin
    -o, --output <output>            Output file. If not specified, will write to stdout
    -t, --type <type>                What to interpret the input as
```
