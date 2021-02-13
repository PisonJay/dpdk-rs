# dpdk-rs
## Build configuration
This crate assumes you have DPDK installed globally, where both `pkg-config` and the dynamic loader can
find it. However, if you don't want to install it globally, you can build DPDK with a prefix. Say you
want to install it to `$DPDK_PATH`:
```
meson --prefix=$DPDK_PATH build
ninja -C build
ninja -C build install
```
Then, be sure to set `PKG_CONFIG_PATH` when building this crate (or a crate that depends on it):
```
PKG_CONFIG_PATH=$DPDK_PATH/lib/x86_64-linux-gnu/pkgconfig cargo build
```
Finally, when running the final binary, be sure to set `LD_LIBRARY_PATH`:
```
LD_LIBRARY_PATH=$DPDK_PATH/lib/x86_64-linux-gnu <your_binary>
```

## Usage
Use this crate as a dependency, and be sure to call `load_mlx5_driver` somewhere in your program.
This ensures that the linker includes the `librte_net_mlx5` PMD dependency: DPDK doesn't explicitly
depend on this library but rather has the driver register itself into a global list at runtime, and
the Rust-driven linking process will omit the dependency if it looks unused.

## Shortcomings
The `librte_net_mlx5` hack could be removed if we could tell Rust's linker to send `--no-as-needed` to the linker.

Ideally, we'd be able to avoid needing to set `LD_LIBRARY_PATH` at runtime by having the linker emit a library
rpath for the final dynamic shared library. We would need to instruct Cargo to do this with a `-rpath` argument
to the linker. Additionally, the newer form of an ELF `RUNPATH` does not apply to recursive dependencies, which
DPDK relies on, so we would also need to pass `--disable-new-dtags`.


