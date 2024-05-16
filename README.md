{{project-name}} is a binary program that can be run with the
[Pip-MPU](https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu) kernel to
be run as a isolated partition.

## Dependencies and toolchain installation

To build this project, you need:

1. a working rust environment with the nightly toolchain
   installed. Simplest solution is to follow the [Getting
   started](https://www.rust-lang.org/learn/get-started) and install
   `rustup`. Then do a simple: 
   ```bash
   rustup toolchain install nightly
   ``` 
    to add the nightly toolchain. This is needed as we are going to build
   `rust-std` from source for our target and use some unstable
   features

2. [cargo-make](https://crates.io/crates/cargo-make) that we using
   to perform all pre and post build operation to generate the final
   binary file that you will be able to link with Pip. You can install
   it using `cargo install cargo-make`

3. `arm-none-eabi-binutils` and `arm-none-eabi-gcc` as we are using
   `gcc` to perform link with specific options that are not currently
   supported by `lld`.

4. an environment in which the python modules listed in
   `relocation_tools/relocator/requirements.txt` are available.
   You can use a virtualenv to do have it
   ```bash
   python3 -m venv pip_venv
   source pip_venv/bin/activate
   pip install -r relocation_tools/relocator/requirements.txt
   ```
   
   
## Building project

To build the project, issue the following command

```bash
cargo make [-p release]
```

The optional `-p release` is used to select a release build profile
and thus build the code with all optimization and link time
optimization.

The final partition binary file will be located at the root of the
crate and be named `partition-dev.bin` or `partition-release.bin`
depending on the build profile used.
