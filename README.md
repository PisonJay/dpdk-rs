# dpdk-rs
This crate directly integrates with DPDK's build system as minimally as possible. The build script just shells
out to DPDK's meson-based build system and then collects all of its static libraries. Then, it uses bindgen
to generate Rust bindings to the DPDK headers. Finally, some critical-path functions in DPDK are marked as 
`inline`, so they don't get generated into the static library. We compile a special file "inlined.c" that 
references these functions so Rust can link against them.

The only dependency is meson 0.47 or greater. Be sure to clone recursively to get the DPDK submodule.
