# TC37XPD

TC37X Peripheral Access Crates from Infineon's SVD file.

## Generate PAC from SVD

Generate the Rust sources with [`svd2pac`](https://bitbucket-dmz.infineon.com/projects/AURIXRS/repos/svd2pac/browse) maintained by Infineon:

```bash
$ svd2pac --target aurix TC37XPD-HTC.svd .
```

Format the sources with [`rustfmt`](https://github.com/rust-lang/rustfmt):

```bash
$ rustfmt src/*.rs
```