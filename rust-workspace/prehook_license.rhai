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
