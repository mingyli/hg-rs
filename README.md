- single file
    - [x] write to rev log 
    - [x] dump revisions from rev log
- multiple files
    - [ ] manage manifest with rev log
    - [ ] dirstate to view tracked and untracked files
- collaboration
    - [ ] clone locally
    - [ ] merges locally
    - [ ] remotely
- optimizations
    - [ ] deltas
    - [ ] compression

```
echo thing > hello.txt
hg status
hg add hello.txt
hg status
hg commit
hg debugindex hello.txt
hg debugdata hello.txt 0
hg summary
hg log
```