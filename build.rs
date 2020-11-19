use bindgen::Builder;
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let buildtype = "release";
    let dpdk_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("dpdk");
    let build_dir = dpdk_dir.join("build");

    if !build_dir.exists() {
        println!("cargo:warning=Configuring DPDK...");
        Command::new("meson")
            .arg(format!("--buildtype={}", buildtype))
            .arg("build")
            .current_dir(&dpdk_dir)
            .status()
            .unwrap_or_else(|e| panic!("Failed to configure DPDK: {:?}", e));
    }

    println!("cargo:warning=Building DPDK...");
    Command::new("ninja")
        .current_dir(&build_dir)
        .status()
        .unwrap_or_else(|e| panic!("DPDK build failed: {:?}", e));
    
    let lib_dir = build_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.to_str().unwrap());

    for entry in std::fs::read_dir(lib_dir).expect("Failed to list dir") {
        let entry = entry.expect("Failed to read directory entry");
        let filename = entry.file_name().into_string().expect("Non UTF-8 filename");
        if filename.starts_with("lib") && filename.ends_with(".a") {
            println!("cargo:rustc-link-lib=static={}", &filename[3..filename.len()-2]);
        }
    }
    println!("cargo:rustc-link-lib=numa");

    println!("cargo:warning=Generating bindings...");
    let header_locations = &[
        "dpdk/build",
        "dpdk/config",
        "dpdk/lib/librte_ethdev",
        "dpdk/lib/librte_eal/include",
        "dpdk/lib/librte_eal/x86/include",
        "dpdk/lib/librte_eal/linux/include",
        "dpdk/lib/librte_net",
        "dpdk/lib/librte_mbuf",
        "dpdk/lib/librte_mempool",
        "dpdk/lib/librte_ring",
    ];
    let mut builder = Builder::default();
    for header_location in header_locations {
        builder = builder.clang_arg(&format!("-I{}", header_location));
    }
    let bindings = builder
        // TODO: These two structs generate bindings with repr(packed) and a repr(align) type
        // inside. I'm not exactly sure what DPDK is compiling for these, and we don't need them
        // for now, so I'm just excluding them for now.
        .blacklist_type("rte_arp_ipv4")
        .blacklist_type("rte_arp_hdr")

        .header("wrapper.h")
        .generate_inline_functions(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).expect("Failed to write bindings");

    println!("cargo:warning=Compiling inlined function stubs...");
    let mut builder = cc::Build::new();
    // Set -O3 so all of the helper functions get inlined and only the top level functions (like
    // rte_pktmbuf_alloc) are remaining.
    builder.opt_level(3);
    builder.pic(true);
    builder.flag("-march=native");
    builder.file("inlined.c");
    for header_location in header_locations {
        builder.include(header_location);
    }
    builder.compile("inlined");

}
