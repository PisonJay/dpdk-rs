# dpdk-rs
1) Build DPDK on your own somewhere, and install it to some directory. You can do this with something like...
```
meson --prefix=$DPDK_PATH build
ninja -C build
ninja -C build install
```
2) Use this crate as a dependency, and be sure to call `load_mlx5_driver` somewhere in your program. This ensures
   that the linker includes the `librte_net_mlx5` PMD dependency. Set the `DPDK_PATH` environment variable to
   your installation directory while building.
3) Run your program while setting `LD_LIBRARY_PATH` to `$DPDK_PATH/lib/x86_64-linux-gnu`.
