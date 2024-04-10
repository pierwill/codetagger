1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Download this repo.
3. From the repo root, run `cargo install --path .`. Now you can run `codetagger` from anywhere.
4. Pass the docs repo path and the path to the list of includes files:

```
codetagger --repo "/Users/me/repo/cloud-docs" --includesfile "path to includes file"
```

5. If you like the output, run again with `--dryrun=false`.
