# hg-rs

An implementation of Mercurial in pure Rust.

## Example 

Add files to be tracked

```sh
$ hg-rs init
$ echo 'fn main() { println!("Hello!"); }' > hello.rs
$ hg-rs add hello.rs
$ hg-rs commit -m "Add hello.rs"
$ hg-rs log
changeset: 0:f987a9e38cfcafa5f3149b94149a00fd9857547a
user:      mingyli34@gmail.com
date:      2020-06-22 00:20:57.545640 UTC
summary:   Add hello.rs
```

Modify files

```sh
$ echo 'fn main() { println!("Hello world!"); }' > hello.rs
$ hg-rs status
M hello.rs
$ hg-rs commit -m "Say Hello World instead"
$ hg-rs log
changeset: 1:943d117b5effc7e93d47ca92853a251b2dd8ad03
user:      mingyli34@gmail.com
date:      2020-06-22 00:22:14.086339 UTC
summary:   Say Hello World instead

changeset: 0:f987a9e38cfcafa5f3149b94149a00fd9857547a
user:      mingyli34@gmail.com
date:      2020-06-22 00:20:57.545640 UTC
summary:   Add hello.rs
```

Inspect file history

```sh
$ hg-rs debugdata 0 hello.rs
fn main() { println!("Hello!"); }
$ hg-rs debugdata 1 hello.rs
fn main() { println!("Hello world!"); }
```

## Roadmap

- single file
    - [x] write to rev log 
    - [x] dump revisions from rev log
    - [x] inspect indices
- multiple files
    - [x] manage manifest with rev log
    - [x] dirstate to view tracked and untracked files
- collaboration
    - [ ] clone 
    - [ ] merges 
    - [ ] remote clone
- optimizations
    - [ ] deltas
    - [ ] compression
- nice to haves
    - [ ] run from nested directories
    - [ ] formatted output
- writing
    - [ ] write a series on implementing Mercurial

## Learnings

Mercurial uses [revlogs](https://www.mercurial-scm.org/wiki/Revlog) to track revisions of files.
A revlog for a particular file consists of an index file containing metadata
about its revisions and a data file containing the contents of that file over time.
`hg-rs`'s indices are identical to those in `hg`:

```sh
$ hexyl .hg-rs/store/data/hello.rs.i
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 00 00 00 00 00 00 00 00 ┊ 22 00 00 00 00 00 00 00 │00000000┊"0000000│
│00000010│ 00 00 00 00 00 00 00 00 ┊ ff ff ff ff ff ff ff ff │00000000┊××××××××│
│00000020│ 43 3c 98 ad 5f 23 dc 68 ┊ 43 07 eb d8 36 25 17 7f │C<××_#×h┊C•××6%••│
│00000030│ 80 92 c4 9a 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │××××0000┊00000000│
│00000040│ 22 00 00 00 00 00 00 00 ┊ 28 00 00 00 00 00 00 00 │"0000000┊(0000000│
│00000050│ 01 00 00 00 00 00 00 00 ┊ 00 00 00 00 ff ff ff ff │•0000000┊0000××××│
│00000060│ e3 fb 82 85 44 95 a9 db ┊ c6 80 c9 9e 1b f4 f4 d7 │××××D×××┊××××•×××│
│00000070│ eb cb b2 6d 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │×××m0000┊00000000│
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
```

The index file is composed of many 64 byte [records](http://aosabook.org/en/mercurial.html#tbl.hg.records),
each of which represents a particular revision of the file.
A record points to a section of the data file (`.hg-rs/store/data/hello.rs.d`)
which contains the bytes necessary to reconstruct the contents of the file
at that revision.
The data section can be either a snapshot of the entire file or a delta from
a previous revision.
A nice thing about revlogs is that since the index file is composed of fixed-sized
structs, we can bound the number of disk seeks required to view a given revision.

## Compatibility with Mercurial

`hg-rs` is not compatible with existing Mercurial repositories.
