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

[workspace.lints.rust]
unsafe_code = "forbid"
missing_debug_implementations = "warn"
# missing_docs = "warn"
# unused_crate_dependencies = "warn"
unused_macro_rules = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = 0 }
unwrap_used = { level = "deny", priority = 1 }
verbose_file_reads = { level = "warn", priority = 1 }
clone_on_ref_ptr = { level = "deny", priority = 1 }
todo = { level = "warn", priority = 1 }
unimplemented = { level = "warn", priority = 1 }

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
