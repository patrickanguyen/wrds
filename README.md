<!--
SPDX-FileCopyrightText: 2026 Patrick Nguyen

SPDX-License-Identifier: MPL-2.0
-->

# WRDS

> [!WARNING]
> This library is very experimental and should not be used for anything serious.

A [Radio Data System (RDS)](https://en.wikipedia.org/wiki/Radio_Data_System) decoder library written in Rust.

RDS is a communications protocol that allows FM broadcasts to transmit metadata to receivers like program identification, traffic announcements, and program information.

This library supports both `std` and `no_std` (with the `heapless` feature enabled) environments.

## Goals

The goal of this library is to prioritize depth over breadth, providing a robust RDS decoder that covers the majority of use cases. Less common features like Radio Paging are out of scope, but may be considered in the future.

It ultimately depends on what features I find the most interesting.

## Supported & Unsupported fields

This is the list of supported and planned to be supported fields.

- [X] Programme Identifier (PI)
- [X] Programme Type (PTY)
- [X] Traffic Program (TP)
- [X] Programme Service Name (PS)
- [X] Radio Text (RT)
- [X] Radio Text Plus (RT+)
- [X] Traffic Announcement (TA)
- [X] Decoder Identification (DI)
- [ ] Alternative Frequency (AF)
- [ ] Enhanced Other Networks (EON)
- [ ] Programme Type Name (PTYN)
- [ ] Traffic Message Channel (TMC)
- [ ] Enhanced Radio Text (eRT)

## Optional Cargo Features

- `heapless`: Use [heapless](https://docs.rs/heapless/latest/heapless/) crate for data structures, instead of `std::collections`.

## License

Licensed under [Mozilla Public License Version 2.0](LICENSES/MPL-2.0.txt)
