- Setup

```
  # code lint
  rustup component add clippy
  cargo clippy

  # code format
  rustup component add rustfmt
  cargo fmt


  # create project
  cargo new kvs --lib
```

- copy the test cases from 
https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-1/tests/tests.rs
to
src/tests/cli.rs


- run test and got errors

- add dependency
```
assert_cmd = "0.11.0"
predicates = "1.0.0"
```

- Create KvStore in lib.rs
Driven by test



- Work on CLI program
https://qiita.com/tigercosmos/items/678f39b1209e60843cc3

Copy the code from 
https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-1/src/bin/kvs.rs
to
src/bin/kvs.rs

```
use clap::{App, AppSettings, Arg, SubCommand};
```


## Doc
Generate doc
```
  cargo doc
```

Open the doc
```
cargo doc --open
```