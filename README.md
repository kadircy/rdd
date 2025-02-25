# `rdd`
`rdd` is a simple Rust library to interact with the `dd` command-line tool. It provides an easy way to configure and execute `dd` commands from Rust, with options like input and output files, block sizes, and more.

## Installation
Add `rdd` to your `Cargo.toml`:

```toml
[dependencies]
rdd = "0.1"
```

## Usage
```rust
use rdd::Dd;

fn main() {
    // Create a new Dd instance with the 'dd' binary path
    let dd = Dd::new("dd");

    // Set input and output files
    dd.input("./test.iso");
    dd.output("./copied.iso");

    // Set block size (bs)
    dd.bs("4M");

    // Run the 'dd' command and handle the result
    match dd.spawn() {
        Ok(output) => println!("DD command output: {}", output),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Options
You can set these options for the `dd` command:

- **Block size (`bs`)**: `dd.bs("4M");`
- **Conversion block size (`cbs`)**: `dd.cbs("1M");`
- **Count**: `dd.count(100);`
- **Seek**: `dd.seek(10);`
- **Skip**: `dd.skip(10);`
- **Status**: `dd.status("progress");`
- **Conversion (`conv`)**: `dd.conv("noerror,sync");`
- **Input block size (`ibs`)**: `dd.ibs("512K");`
- **Output block size (`obs`)**: `dd.obs("64K");`
- **Flags (`iflag`, `oflag`)**: `dd.iflag("direct"); dd.oflag("sync");`

## Contributing
Contributions are welcome! If you’d like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch `git checkout -b feature-branch`.
3. Make your changes and commit them `git commit -am 'Add new feature'`.
4. Push to your fork `git push origin feature-branch`.
5. Open a pull request to the main repository.

Please make sure your code follows the style and guidelines of the project and passes all the tests. You can format codebase and lint it with this commands:

```bash
cargo fmt    # format
cargo clippy # lint
cargo test   # test
```

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

