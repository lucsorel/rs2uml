# rs2uml

Generate UML class diagrams to document your rust application, export in various formats (PlantUML, for now).

## Use rs2uml

```sh
cargo run --bin rs2uml -- --help
```

Outputs
- for enum, struct (associated functions, methods )
- export in plantuml syntax


## Contribute

This repository uses moon to provide pre-commit hooks and tasks while automatically installing and updating the dev toolchain with proto (rust for the CLI, python for the pre-commit hooks) and moon.
