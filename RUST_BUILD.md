# Rust Modules Build Status

The Rust modules in this repository (`home-mixer`, `thunder`, `candidate-pipeline`) are currently provided as a **source-code transparency release**. 

## Current Limitations
- **Missing Build Files**: `Cargo.toml` files are not included in the open-source release.
- **Private Dependencies**: The code references several internal xAI libraries (`xai_home_mixer_proto`, `xai_http_server`, `xai_stats_macro`, `xai_init_utils`) that are not publicly available.

## Contributing to the Rust Pipeline
If you wish to help make these modules buildable for the community:
1. **Mock Services**: Create mocked versions of the `xai_*` internal crates to allow compilation.
2. **Cargo Workspace**: Help define the workspace structure and public dependencies (e.g., `tonic`, `tokio`, `axum`).
3. **Transparency Review**: The source code is full-production logic. You can contribute by analyzing the logic and suggesting optimizations even if it doesn't compile yet.

For the **Python/JAX** components in `phoenix`, they are fully buildable and runnable using `uv`.
