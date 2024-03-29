# Rosix

Experiments with OS on Rust according to information from the blog: https://os.phil-opp.com/

# Build this on Gentoo Linux

Problems:

* void llvm::MachineFunction::init(): Assertion `Target.isCompatibleDataLayout(getDataLayout()) && "Can't create a MachineFunction using a Module with a " "Target-incompatible DataLayout attached\n"' failed.

*   Error: llvm-tools not found
  Maybe the rustup component `llvm-tools-preview` is missing?
    Install it through: `rustup component add llvm-tools-preview`

* error: linker `rust-lld` not found 

Fixes:

```sh
$ cat /etc/portage/profile/package.use.mask/dev-lang::rust.conf
dev-lang/rust -nightly
dev-lang/rust -system-llvm

$ grep rust /etc/portage/package.use/misc
dev-lang/rust rustfmt rust-src rust-analyzer nightly system-llvm
virtual/rust rustfmt

$ grep llvm /etc/portage/package.accept_keywords
sys-devel/llvm
sys-devel/llvm-toolchain-symlinks
sys-devel/llvmgold

$ ln -s /usr/lib/llvm/16/bin/ld.lld /usr/local/bin/rust-lld
$ ln -s /usr/lib/llvm/16/bin/ /usr/lib/rust/1.74.1/lib/rustlib/x86_64-unknown-linux-gnu
```
