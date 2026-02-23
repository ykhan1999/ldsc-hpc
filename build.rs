use std::env;

fn main() {
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    if target_family != "windows" {
        return;
    }

    let use_system_openblas = env::var("CARGO_FEATURE_BLAS_OPENBLAS_SYSTEM").is_ok();
    if !use_system_openblas {
        return;
    }

    // On Windows, OpenBLAS from vcpkg does not ship LAPACK symbols.
    // Ensure Clapack is available and linked to satisfy LAPACK calls.
    if env::var("VCPKGRS_DYNAMIC").is_err() {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }
    if env::var("VCPKGRS_TRIPLET").is_err() {
        env::set_var("VCPKGRS_TRIPLET", "x64-windows");
    }
    match vcpkg::Config::new()
        .emit_includes(false)
        .find_package("clapack")
    {
        Ok(_) => {}
        Err(err) => panic!(
            "Failed to find vcpkg clapack package. Install with: vcpkg install clapack:x64-windows. Error: {err}"
        ),
    }
}
