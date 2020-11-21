use clap::Clap;
use toml;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "pitust <piotr@stelmaszek.com>")]
struct Opts {
    /// Output of ohbuild
    #[clap(short, long)]
    out: Option<String>,
    #[clap(short, long, default_value = "/tmp/ohbuild.cache")]
    /// Cache dir.
    cache: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!(" == ohbuild started == ");
    println!("{:?}", opts);
    let cargo_toml: toml::Value = std::fs::read_to_string("Cargo.toml")
        .unwrap()
        .parse()
        .unwrap();
    let name = cargo_toml
        .as_table()
        .unwrap()
        .get("package")
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap();
    let ldscript = include_str!("user.ld");
    let tgd = include_str!("target.json");

    let cached_ld_script = opts.cache.clone() + "/link.ld";
    let cached_targetspec = opts.cache.clone() + "/target.json";
    let cargo_cache_dir = opts.cache.clone() + "/cargo-cache/" + name;
    let out = match  opts.out {
        Some(p) => { p }
        None => name.to_string()
    };
    std::fs::create_dir_all(cargo_cache_dir.clone()).unwrap();
    std::fs::write(cached_ld_script.clone(), ldscript).unwrap();
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
        .env("RUSTFLAGS",format!("-Clink-args=-T{}", cached_ld_script))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    std::fs::copy(format!("{}/target/debug/{}", cargo_cache_dir, name), out).unwrap();
}
