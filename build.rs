// Example custom build script.
fn main() {
    println!("cargo:warning=This does show up though");

    read_spin_toml();
}

fn read_spin_toml() {
    let spin_toml = fs::read_to_string("./spin_toml")
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
}
