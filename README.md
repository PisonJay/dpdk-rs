# dpdk-rs
1) Build DPDK on your own somewhere, and install it to some directory. You can do this with something like...
```
meson --prefix=$INSTALL_DIR build
ninja build
ninja build install
```
2) Use this crate as a dependency, and be sure to call `load_mlx5_driver` somewhere in your program. This ensures
   that the linker includes the `librte_net_mlx5` PMD dependency.
3) Run your program while setting `LD_LIBRARY_PATH` to `$INSTALL_DIR/lib/x86_64-linux-gnu`.
