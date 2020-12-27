Pushbits
=========

![https://docs.rs/pushbits/](https://docs.rs/pushbits/badge.svg)

Push and pop the bit-packed fields.

It's common for network protocols to have bit-packed fields
as a fixed width row in its header.
Each fields have their own bit width, and its bit offset is defined
as a sum of the widths of previous fields of the same row.

Traditionaly to set/extract those fields one should manage
both width and offset of the fields.
But since all the fields of this row are handled,
the offset is a redaundant information we should not care ourselves.

That's where the pushbits came from.
This crate provides fixed width bit container where you can
push and pop the bits as a integer using bitshift left operation.
If the widths are constant, the compiler optimize out all the overheads.

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
