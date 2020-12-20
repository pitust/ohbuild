
macro_rules! cwd {
    () => {
        std::env::current_dir().unwrap().as_os_str().to_string_lossy()
    };
}

fn main() {
    println!(" == ohbuild (mbcc mode) started == ");
    let a = std::env::args().nth(1).unwrap();
    let p = std::path::Path::new(&a).file_name().unwrap().to_os_string().to_str().unwrap().to_string();
    let name = p.split('.').nth(0).unwrap();
    let ldscript = include_str!("user.ld");
    let tgd = include_str!("target.json");

    let cached_ld_script = format!("{}/target/{}/link.ld", cwd!(), name);
    let cached_targetspec = format!("{}/target/{}/target.json", cwd!(), name);
    let cargo_cache_dir = format!("{}/target/{}", cwd!(), name);
    let out = format!("rootfs/bin/{}", name);
    std::fs::create_dir_all(cargo_cache_dir.clone()).unwrap();
    std::fs::create_dir_all(format!("{}/target/{}/ns.dir/src", cwd!(), name)).unwrap();
    std::fs::copy(a, format!("{}/target/{}/ns.dir/src/main.rs", cwd!(), name)).unwrap();
    std::fs::write(cached_ld_script.clone(), ldscript).unwrap();
    std::fs::write(format!("{}/target/{}/ns.dir/Cargo.toml", cwd!(), name), include_str!("cargo-mbcc.toml")).unwrap();
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
        .current_dir(format!("{}/target/{}/ns.dir", cwd!(), name))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    std::fs::copy(format!("{}/target/debug/mbcc-pkg", cargo_cache_dir), out).unwrap();
}
