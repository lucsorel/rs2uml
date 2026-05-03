use ignore::{Error, WalkBuilder, overrides::OverrideBuilder};
use std::path::PathBuf;

pub fn find_rust_files(root: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    if root.is_file() && root.extension().and_then(|extension| extension.to_str()) == Some("rs") {
        return Ok(vec![root.clone()]);
    }

    let mut rust_files: Vec<PathBuf> = Vec::new();

    let mut rust_files_filter_builder = OverrideBuilder::new(root);
    rust_files_filter_builder.add("**/*.rs")?;

    let walker = WalkBuilder::new(root)
        .standard_filters(true)
        .hidden(true)
        .overrides(rust_files_filter_builder.build()?)
        .build();

    for entry in walker.filter_map(Result::ok) {
        if entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            rust_files.push(entry.path().to_path_buf());
        }
    }

    Ok(rust_files)
}
