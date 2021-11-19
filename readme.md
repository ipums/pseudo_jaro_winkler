## Rlink

Rlink is a library used for creating very fast psuedo jaro winkler scores. 
It is currently in a beta state, and is mainly used as a proof of concept for the idea.

## How to use as library

There is currently just one function availabile: `compare_batches`. You can call it like this:

```
compare_batches(PathBuf::from("output_dir"), names_a, names_b, 0.8)
```

This will compare all the strings in names_a to all the strings in names_b.
It writes out matches to the files `X.txt` within the output dir, where `X` is the index of the name in name_a.
It will only write out matches if the score is greater than 0.8.

## How to use at the command line

First build rlink using cargo:

```
cargo build --release
```

Then you can call `./target/release/rlink input/file_a.txt input/file_b.txt output`. 
Use the `--help` flag for more information on the arguments.

## Performance

When testing on an Intel Core i7-6700K CPU @ 4GHz, I get a throughput of about 47 million comparisons a second. 

## Caveats
The scores that it returns are "psuedo" jaro winkler socres, in that there are some approximations involved to increase performance.

1) The scores are not accurate past the 2nd decimal place. 
2) Transpositions are approximated, and in some cases are over counted. 
   This usually happens whenever characters at the beginning and end of a string are swapped, for example "abcd" and "dbca". 

I'm currently working on ways to improve the accuracy of the algorithm without comprimising on throughput.

## How does it work
It is inspired by the https://github.com/dbousque/batch_jaro_winkler library, and builds a lookup table of words by letter.
It then keeps track of a score for each match and updates that score letter by letter.
Additionally, all comparisons are done using bitwise operations.

I plan on writing a blog post in the near future to explain more about the algorithm.

## Contributing

We greatly appreciate bug reports, suggestions or pull requests. They can be submitted via github.

Before contributing, please be sure to read the Contributing Guidelines and the Code of Conduct.
