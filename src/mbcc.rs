
fn main() {
    println!(" == ohbuild (mbcc mode) started == ");
    let a = std::env::args().nth(1).unwrap();
    let p = std::path::Path::new(&a).file_name().unwrap().to_os_string().to_str().unwrap().to_string();
    let name = p.split('.').nth(0).unwrap();
    let ldscript = include_str!("user.ld");
    let tgd = include_str!("target.json");

    let cached_ld_script = "/tmp/ohbuild.cache/link.ld";
    let cached_targetspec = "/tmp/ohbuild.cache/target.json";
    let cargo_cache_dir = format!("{}/target/{}", std::env::current_dir().unwrap().as_os_str().to_string_lossy(), name);
    let out = format!("rootfs/bin/{}", name);
    std::fs::create_dir_all(cargo_cache_dir.clone()).unwrap();
    std::fs::create_dir_all(format!("/tmp/ohbuild.cache/ns.dir/src")).unwrap();
    std::fs::copy(a, "/tmp/ohbuild.cache/ns.dir/src/main.rs").unwrap();
    std::fs::write(cached_ld_script.clone(), ldscript).unwrap();
    std::fs::write("/tmp/ohbuild.cache/ns.dir/Cargo.toml", include_str!("cargo-mbcc.toml")).unwrap();
    std::fs::write(cached_targetspec.clone(), tgd).unwrap();
    println!("{}", cached_targetspec);

    std::process::Command::new("cargo")
        .args(&[
            "build",
            "-Z",
            "build-std=core,alloc",
        ])
        .env("CARGO_TARGET_DIR", cargo_cache_dir.clone())
        .env("CARGO_BUILD_TARGET", cached_targetspec)
        .env("RUSTFLAGS",format!("-Clink-args=-T{} -Ccode-model=large", cached_ld_script))
        .current_dir("/tmp/ohbuild.cache/ns.dir")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    std::fs::copy(format!("{}/target/debug/{}", cargo_cache_dir, name), out).unwrap();
}
