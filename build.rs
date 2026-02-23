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
