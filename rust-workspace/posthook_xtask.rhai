fn add_dep(crate, features) {
    if features.is_empty() {
        system::command("cargo", [
            "add",
            "-p",
            "xtask",
            crate,
        ]);
    } else {
        let feature_list = features.reduce(|acc, el, i| if i == 0 {
            el
        } else {
            `{acc},{el}`
        });

        system::command("cargo", [
            "add",
            "-p",
            "xtask",
            crate,
            "--features",
            feature_list,
        ]);
    }
}

if variable::get("include-xtask") {
    add_dep("error-stack");
    add_dep("bpaf", ["dull-color"]);
    add_dep("duct");
    add_dep("cargo_metadata");
}
