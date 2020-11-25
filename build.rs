use bindgen::Builder;
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    // // This is a bit hax (we only rebuild if the git version changes) but hopefully good enough.
    // println!("cargo:rerun-if-changed=.git/modules/dpdk/HEAD");

    // let out_dir_s = env::var("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir_s);
    // let dpdk_install_dir = out_dir.join("dpdk-install");
    // let dpdk_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("dpdk");

    // if !dpdk_install_dir.is_dir() {
    //     fs::create_dir_all(&dpdk_install_dir).unwrap();
    // }

    // println!("cargo:warning=Configuring DPDK...");
    // let build_dir = dpdk_dir.join("build");
    // if !build_dir.exists() {
    //     Command::new("meson")
    //         .arg(&format!("--buildtype=release"))
    //         .arg(&format!("--prefix={}", dpdk_install_dir.to_str().unwrap()))
    //         .arg("build")
    //         .current_dir(&dpdk_dir)                                                      
    //         .status()
    //         .unwrap_or_else(|e| panic!("Failed to configure DPDK: {:?}", e));
    // }

    // println!("cargo:warning=Building DPDK...");
    // Command::new("ninja")
    //     .args(&["-C", "build"])
    //     .current_dir(&dpdk_dir)
    //     .status()
    //     .unwrap_or_else(|e| panic!("Failed to build DPDK: {:?}", e));

    // println!("cargo:warning=Installing DPDK...");
    // Command::new("ninja")
    //     .args(&["-C", "build", "install"])
    //     .current_dir(&dpdk_dir)
    //     .status()
    //     .unwrap_or_else(|e| panic!("Failed to install DPDK: {:?}", e)); 
    //
    let dpdk_path = Path::new(env::var("DPDK_PATH").unwrap());

    let pkg_config_path = dpdk_install_dir.join("lib/x86_64-linux-gnu/pkgconfig");
    let cflags_bytes = Command::new("pkg-config")
        .env("PKG_CONFIG_PATH", &pkg_config_path)
        .args(&["--cflags", "libdpdk"])
        .output()
        .unwrap_or_else(|e| panic!("Failed pkg-config cflags: {:?}", e))
        .stdout;
    let cflags = String::from_utf8(cflags_bytes).unwrap();

    let mut header_locations = vec![];

    for flag in cflags.split(' ') {
        if flag.starts_with("-I") {
            let header_location = &flag[2..];
            header_locations.push(header_location);
        }
    }

    let ldflags_bytes = Command::new("pkg-config")
        .env("PKG_CONFIG_PATH", &pkg_config_path)
        .args(&["--libs", "libdpdk"])
        .output()
        .unwrap_or_else(|e| panic!("Failed pkg-config ldflags: {:?}",e ))
        .stdout;
    let ldflags = String::from_utf8(ldflags_bytes).unwrap();

    let mut library_location = None;
    let mut lib_names = vec![];

    for flag in ldflags.split(' ') {
        if flag.starts_with("-L") {
            library_location = Some(&flag[2..]);
        } else if flag.starts_with("-l") {
            lib_names.push(&flag[2..]);
        }
    }

    // Step 1: Now that we've compiled and installed DPDK, point cargo to the libraries.
    println!("cargo:rustc-link-search=native={}", library_location.unwrap());
    for lib_name in &lib_names {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    println!("cargo:rustc-link-lib=rte_net_mlx5");
    println!("cargo:rustc-link-lib=rte_common_mlx5");
    println!("cargo:rustc-link-lib=rte_regex_mlx5");
    println!("cargo:rustc-link-lib=rte_vdpa_mlx5");
    println!("cargo:rustc-link-lib=rte_bus_pci");
    println!("cargo:rustc-link-lib=mlx5");
    println!("cargo:rustc-link-lib=ibverbs");
    println!("cargo:rustc-link-lib=numa");

    // Step 2: Generate bindings for the DPDK headers.
    let mut builder = Builder::default();
    for header_location in &header_locations {
        builder = builder.clang_arg(&format!("-I{}", header_location));
    }
    let bindings = builder
        .blacklist_type("rte_arp_ipv4")
        .blacklist_type("rte_arp_hdr")

        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap_or_else(|e| panic!("Failed to generate bindings: {:?}", e));
    let bindings_out = out_dir.join("bindings.rs");
    bindings.write_to_file(bindings_out).expect("Failed to write bindings");

    // Step 3: Compile a stub file so Rust can access `inline` functions in the headers 
    // that aren't compiled into the staticlibs. 
    let mut builder = cc::Build::new();
    builder.opt_level(3);
    builder.pic(true);
    builder.flag("-march=native");
    builder.file("inlined.c");
    for header_location in &header_locations {
        builder.include(header_location);
    }
    builder.compile("inlined");

//    // Step 4: It seems like the flags from pkg-config above don't include all of the dynamic
//    // libraries we need to be able to load the right drivers at runtime. Tell cargo to express a
//    // dependency on these libraries, accumulated from running `ldd` on `testpmd`.
//    let dynamic_libs = &[
//        "ibverbs",
//        "mlx4",
//        "mlx5",
//        "nl-3",
//        "nl-route-3",
//        "numa",
//        "pcap",
//    ];
//    for dynamic_lib in dynamic_libs {
//        println!("cargo:rustc-link-lib=dylib={}", dynamic_lib);
//    }
}
