# Mandate Pallet

A Substrate pallet to allow the use of `sudo` functions from a runtime module like the `collective`.


# Usage

## Add the depedency

Edit `runtime/Cargo.toml` and add the following:
```toml
[dependencies.mandate]
default-features = false
version = '2.0.0'
package = "pallet-mandate"
git = 'https://github.com/ETeissonniere/pallet-mandate'
```

Then add the `mandate/std` to the `[features]` section in the `std` array, it should
look like this:
```toml
[dependencies.mandate]
default-features = false
version = '2.0.0'
package = "pallet-mandate"
git = 'https://github.com/ETeissonniere/pallet-mandate'

[features]
default = ['std']
std = [
    # Your substrate modules /std calls here
    # ...

    'mandate/std',
]
```


## Add the module to your runtime

### Trait implementation

You can use the `ExternalOrigin` type to specify who can dispatch calls to the module.
For instance you can use with the `collective`:
```rust
impl mandate::Trait for Runtime {
    type Proposal = Call;

    // A majority of the committee can dispatch root calls
    type ExternalOrigin =
        collective::EnsureProportionAtLeast<_1, _2, AccountId, TechnicalCollective>;
}
```


### Adding the module

In the `construct_runtime` macro call just add the `Mandate: mandate::{Module, Call}`, it should
look like this:
```rust
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
    	// Your modules here
    	// ...
        Mandate: mandate::{Module, Call},
    }
);
```