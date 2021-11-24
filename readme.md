# Pseudo Jaro Winkler

## Overview

Pseudo Jaro Winkler is a library used for creating very fast (almost) jaro winkler scores. It compares two datasets and writes out the indices of the matches above a specific threshold.

This library was developed primarily for matching historical US names, but could have other uses as well. All tests were done using names from the 1880 US census.

## How to use at the command line

First build `pseudo_jaro_winkler` using cargo:

```
cargo build --release
```

Then you can call `./target/release/pseudo_jaro_winkler input/file_a.txt input/file_b.txt output`. 
Use the `--help` flag for more information on the arguments.


## How to use as library

There is currently just one function availabile: `pseudo_jaro_winkler`. You can call it like this:

```
pseudo_jaro_winkler(PathBuf::from("output_dir"), names_a, names_b, 0.8)
```

This will compare all the strings in `names_a` to all the strings in `names_b`.
It writes out matches to the files `X.txt` within the output dir, where `X` is the index of the name in `names_a`.  It will only write out matches if the score is greater than 0.8.

## Rayon usage

This library uses rayon, which by default uses as many threads as your OS has available. If you would like to use less threads, you need to set the environment variable `RAYON_NUM_THREADS` to the number of threads you want to use. To just use one thread then: `RAYON_NUM_THREADS=1`.

## Performance

The algorithm's performance was tested by comparing 500 names from 1880 in the `input/file_a_small.txt` file to the ~86k names in the `input/file_b_small.txt`. I tested the performance in multiple libraries: 
- [batch jaro winkler](https://github.com/dbousque/batch_jaro_winkler)
  - One 'raw' implmentation which ignores duplicate names.
  - One 'lookup' implementation which looks up the ids of all the names.
- [strsim](https://github.com/dguo/strsim-rs)
- [eddie](https://docs.rs/eddie/0.4.2/eddie/)
- [jellyfish](https://github.com/jamesturk/jellyfish)

The `strsim` and `eddie` implementations are in the `lib.rs` file and the `batch jaro winkler` and `jellyfish` implementations are in the `other_langs` dir. 

I ran the tests using an Ubunut wsl2 instance on my Windows desktop which runs an Intel Core i7-6700K CPU at 4.00GHz. All tests were using only 1 core.

| program               | runtime (seconds) | ratio to `pseudo_jaro_winkler` |
| --------------------- | ----------------- | ------------------------------ |
| `pseudo_jaro_winkler`	| 0.125             |	1                              |
| eddie               	| 5.584             |	44.672                         |
| strsim	              | 6.598             |	52.784                         |
| jellyfish	            | 385.663           |	3085.304                       |
| batch raw	            | 1.552             |	12.416                         |
| batch lookup	        | 1.8               | 14.4                           |


## Differences to true Jaro Winkler

Transpositions are approximated, and in some cases are over counted. This usually happens whenever characters at the beginning and end of a string are swapped, for example "abcd" and "dbca". I'm currently working on ways to improve the accuracy of the algorithm without comprimising on performance.

## How does it work
It is inspired by the [batch jaro winkler](https://github.com/dbousque/batch_jaro_winkler) library, and builds a lookup table of words by letter.
It then keeps track of a score for each match and updates that score letter by letter. Additionally, all comparisons are done using bitwise operations.

## Contributing

We greatly appreciate bug reports, suggestions or pull requests. They can be submitted via github.

Before contributing, please be sure to read the Contributing Guidelines and the Code of Conduct.
