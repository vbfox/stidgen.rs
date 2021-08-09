# Strongly Typed ID types macro

A macro to simplify usage of srongly type ID types instead of plain
`String`, `u64` or `Guid` in Rust codebases.

```rust
use stidgen::{Id, id};

#[id]
pub struct UserId(String);

#[id(NoDefaults, Format, Debug)]
pub struct UserId(u64);
```

While the `derive` macro can already be used to achieve most of what this
macro proposes using it has the following advantages:

- It's simplier to use for well known types, simply add `#[id]` to your struct
- The defaults provide a lot of blanket implementations that might be missed when doing it manually
- Some features provided don't have corresponding derive macros (`AsBytes`, `AsRef`, `Borrow`)
- All trivial functions are marked inline

### Settings

- `Defaults`/`NoDefaults`: Enable or disable defaults for known types
- `Clone`/`NoClone`: Enable or disable deriving [`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- `Hash`/`NoHash`: Enable or disable deriving [`std::hash::Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)
- `PartialEq`/`NoPartialEq`: Enable or disable deriving [`std::cmp::PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
- `Eq`/`NoEq`: Enable or disable deriving [`std::cmp::Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
- `PartialOrd`/`NoPartialOrd`: Enable or disable deriving [`std::cmp::PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
- `Ord`/`NoOrd`: Enable or disable deriving [`std::cmp::Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
- `Display`/`NoDisplay`: Enable or disable deriving [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) and adding a `to_string` method
- `Debug`/`NoDebug`: Enable or disable deriving [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- `AsBytes`/`NoAsBytes`: Enable or disable deriving `std::convert::AsRef<[u8]>` and adding a `as_bytes` method
- `Borrow`/`NoBorrow`: Enable or disable deriving [`std::borrow::Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
- `AsRef`/`NoAsRef`: Enable or disable deriving [`std::convert::AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

### Default types

For unknown types all features are disabled by default but some types like `String` have smart defaults.

```rust
#[id(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Display, ToString, Debug, AsBytes, ...)]
pub struct Id(String);
```
