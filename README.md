# TurboPFor Rust Bindings for Open-Meteo TurboPFor-Version

This project provides bindings to the [TurboPFor](https://github.com/powturbo/TurboPFor-Integer-Compression) compression library.

It incorporates modified code from the [open-meteo](https://github.com/open-meteo/open-meteo)
project due to breaking changes in the original codec, which rendered the compression/decompression
incompatible. As a result, the [turbo-pfor-sys](https://github.com/mwlon/turbo-pfor-sys) crate
cannot be used. This repository adapts the code from [turbo-pfor-sys](https://github.com/mwlon/turbo-pfor-sys)
to support the open-meteo versions of TurboPFor.
