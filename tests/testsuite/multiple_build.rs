//! Tests for the multiple-build feature (declarative build scripts).

use std::str;

use cargo_test_support::prelude::*;
use cargo_test_support::{
    basic_lib_manifest, basic_manifest, is_coarse_mtime, project, registry::Package, rustc_host,
    str, Project,
};

#[cargo_test]
fn multiple_build_gated() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                edition = "2024"
                multiple-build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.cargo("check")
        .masquerade_as_nightly_cargo(&["multiple-build"])
        .with_status(101)
        .with_stderr_data(str![[r#"
[ERROR] failed to parse manifest at `[ROOT]/foo/Cargo.toml`

Caused by:
  feature `multiple-build` is required

  The package requires the Cargo feature called `multiple-build`, but that feature is not stabilized in this version of Cargo ([..]).
  Consider adding `cargo-features = ["multiple-build"]` to the top of Cargo.toml (above the [package] table) to tell Cargo you are opting in to use this unstable feature.
  See https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#multiple_build for more information about the status of this feature.

"#]])
        .run();
}

fn basic_project() -> Project {
    project()
        .file(
            "Cargo.toml",
            r#"
                cargo-features = ["multiple-build"]
                [package]
                name = "foo"
                version = "0.0.1"
                edition = "2024"
                multiple-build = ["build_fri.rs,build_sec.rs"]
            "#,
        )
        .file("src/lib.rs", "")
        .file("build_fri.rs", r#"println!("first build")"#)
        .file("build_sec.rs", r#"println!("second build")"#)
        .build()
}

#[cargo_test]
fn multiple_build_basic() {
    let p = basic_project();
    p.cargo("check -vv")
        .masquerade_as_nightly_cargo(&["multiple-build"])
        .run();
    //Currently, the actual function of multiple-build has not been realized.
    // After the implementation, the test needs to supplement the display content.
}
