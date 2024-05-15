# hazard-generator

This library generates hazards APIs starting from a JSON ontology.
For a more in depth explanation about hazards have a look at the [Hazard Documentation](hazards.md).

## Supported Languages

This is the list of supported programming languages:

- [ ] C
- [ ] C++
- [x] Rust

A check indicates the languages for which API generation has been implemented.

## Building

Use this command to build the library:

```console
cargo build 
```

## Testing

Testing has been performed via snapshots, using [insta](https://insta.rs). Use the following command to launch the tests:

``` console
cargo insta test
```

To review the snapshots, use:

``` console
cargo insta review
```

The next command combines the previous two operations:

``` console
cargo insta test --review
```