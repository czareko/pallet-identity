# Substrate Identity Pallet

## Purpose

Basic custom pallet construction.

## Dependencies

### Traits

This pallet does not depend on any externally defined traits.

### Pallets

This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-template]
default_features = false
git = 'https://github.com/czareko/pallet-identity.git'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-identity/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for test_module
impl pallet_template::Config for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
TemplatePallet: pallet_template::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
