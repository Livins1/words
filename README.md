### Words


Command line tool for querying the ECDICT Chinese/English dictionary.

##### Usage
I am using this tool to log search history of the words.

```
words 0.1.0


USAGE:
    words [OPTIONS] [WORD]

ARGS:
    <WORD>    The word you want to search for

OPTIONS:
    -?, --help           Display help information.
    -i, --interactive    Enter the interactive mode
    -s, --save           Save the words history to a log file ( words.log ).
    -V, --version        Display version information.
```
##### Installation
```
Cargo build --release 
# eg. ~/.local/bin or /any/path/in/env
cp ./target/release/words ~/.local/bin/

# You can find the stardict.db at https://github.com/skywind3000/ECDICT
mv stardict.db /usr/share/wordlists/stardict.db
```

##### Thanks
https://github.com/skywind3000/ECDICT



