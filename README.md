# `rfc5545`

The aim of this library is to implement https://tools.ietf.org/html/rfc5545 starting with the bits I need for other projects.

Contributions are welcome.

### License

See the [LICENSE](./LICENSE) file.

### Building

This project is built with stable Rust (2018 edition) using the Cargo project manager:

```
$ cargo build
```

### Usage

To use this library you must add a git reference to your dependency list.

```toml
[dependencies]
rfc5545 = { git = "https://github.com/Phrohdoh/rfc5545-rs.git" }
```

Then you could use the API like so:

```rust
use rfc5545::{RecurrenceRule, RecurRulePart, Frequency};

fn main() {
    let rule = RecurrenceRule::new_with_parts(vec![
        RecurRulePart::Freq(Frequency::Monthly)
    ]);

    println!("{}", rule); // => RRULE:FREQ=MONTHLY
}
```

Currently this project only creates recurrence rule strings, it does not parse strings into typed object.

### Running the examples in this codebase

```
$ cargo run --example print
```

Change `print` to the name of whichever example you'd like to run.