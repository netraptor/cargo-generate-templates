let ws_top = if variable::get("include_xtask") {
    `
[workspace]
resolver = "3"
members = [
    # "crates/*",
    "tools/xtask",
]
`
} else {
    `
[workspace]
resolver = "3"
members = [
    # "crates/*",
]
`
};

let ws_details = ##"[workspace.package]
version = "0.1.0-dev"
authors = ["{{authors}}"]
license = "{{license}}"
edition = "2024"
rust-version = "{{rust-version}}"
repository = ""

[workspace.lints]
rust.unsafe_code = "forbid"
rust.missing_debug_implementations = "warn"
rust.missing_docs = "warn"
rust.unused_crate_dependencies = "warn"
rust.unused_macro_rules = "warn"
rust.unused_qualifications = "warn"
clippy.pedantic = "warn"
clippy.unwrap_used = "deny"
clippy.verbose_file_reads = "warn"
clippy.clone_on_ref_ptr = "deny"
clippy.todo = "warn"
clippy.unimplemented = "warn"

[workspace.dependencies]
"##;

let manifest = if variable::get("toplevel_crate").to_lower() != "none" {
    let toplevel_details = ##"[package]
name = "{{project-name}}"
version.workspace = true
authors.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[lints]
workspace = true

[dependencies]
"##;

    `
${ws_top}
${ws_details}
${toplevel_details}`

} else {
    `
${ws_top}
${ws_details}`
};

file::write("Cargo.toml", manifest);
