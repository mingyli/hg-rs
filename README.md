# hg-rs

An implementation of Mercurial in pure Rust.

## Example 

```sh
$ cd project
$ hg-rs init
$ echo 'fn main() { println!("Hello!"); }' > hello.rs
$ hg-rs add hello.rs
$ hg-rs commit -m "Add hello.rs"
$ hg-rs log
changeset: 0:f987a9e38cfcafa5f3149b94149a00fd9857547a
user:      mingyli34@gmail.com
date:      2020-06-22 00:20:57.545640 UTC
summary:   Add hello.rs

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

$ hg-rs debugdata 0 hello.rs
fn main() { println!("Hello!"); }
$ hg-rs debugdata 1 hello.rs
fn main() { println!("Hello world!"); }
```

## Roadmap

- single file
    - [x] write to rev log 
    - [x] dump revisions from rev log
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

