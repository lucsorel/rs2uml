# rs2uml

Generate UML class diagrams to document your rust application, export in various formats (PlantUML, for now).

TODO use inheritance to represent traits, supertraits, extension traits (https://medium.com/@carlmkadie/nine-ways-to-do-inheritance-in-rust-a-language-without-inheritance-14825bf1e215)


## Use rs2uml

```sh
cargo run --bin rs2uml -- --help
cargo run --bin rs2uml -- --path src/main.rs --format plantuml
cargo run --bin rs2uml -- --path src --format plantuml
```

Outputs
- for enum, struct (associated functions, methods), tuple struct, implementation, trait, unit-like structs
- export in plantuml syntax


## Contribute

This repository uses moon to provide pre-commit hooks and tasks while automatically installing and updating the dev toolchain with proto (rust for the CLI, python for the pre-commit hooks) and moon.
