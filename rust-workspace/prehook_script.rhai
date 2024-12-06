/// get the license selection and update files as necessary
fn select_license() {
    let license = variable::get("license").to_lower();

    let spdx_map = #{
        none: #{ spdx: "<license SPDX here>" },
        mit: #{ spdx: "MIT", file: "LICENSE-MIT" },
        apache: #{ spdx: "Apache-2.0", file: "LICENSE-APACHE" },
        mpl: #{ spdx: "MPL-2.0", file: "LICENSE-MPL" },
        gpl3: #{ spdx: "GPL-3.0-only", file: "LICENSE-GPL" },
    };

    while !(spdx_map.contains(license)) {
        license = variable::prompt("Which license do you want to use?", "none", spdx_map.keys());
    }

    variable::set("license", spdx_map[license].spdx);

    if spdx_map[license].file != () {
        file::rename(spdx_map[license].file, "LICENSE");
    }

    for some_license in spdx_map.values() {
        if (some_license.file != ()) && (some_license.file != spdx_map[license].file) {
            file::delete(some_license.file);
        }
    }

    if license == "mit" {
        cr_year = variable::prompt("Please provide a year for the MIT license copyright.");
        cr_name = variable::prompt("Please provide a name for the MIT license copyright.");

        variable::set("year", cr_year);
        variable::set("copyright-name", cr_name);
    }
}

fn crate_type() {
    let crate_type = variable::get("crate-type").to_lower();

    let crate_type_map = #{
        bin: || file::delete("src/lib.rs"),
        lib: || file::delete("src/bin.rs"),
        bin_with_lib: || {},
    };

    while !(crate_type_map.contains(crate_type)) {
        crate_type = variable::prompt("What type of crate is this?", "bin", crate_type_map.keys());
    }

    variable::set("crate-type", crate_type);
    crate_type_map[crate_type].call();
}

fn has_toplevel_crate() {
    if !variable::is_set("toplevel") {
        let toplevel = variable::prompt("Should there be a top-level crate in the workspace?", true);
        variable::set("toplevel", toplevel);
    }

    if variable::get("toplevel") {
        file::delete("Cargo-notoplevel.toml");
        file::rename("Cargo-withtoplevel.toml", "Cargo.toml");
        crate_type();
    } else {
        file::delete("Cargo-withtoplevel.toml");
        file::rename("Cargo-notoplevel.toml", "Cargo.toml");
        file::delete("src/lib.rs");
        file::delete("src/main.rs");
        file::delete("src");
    }
}

// Run script tasks here
select_license();
has_toplevel_crate();
