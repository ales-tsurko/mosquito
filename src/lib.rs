//! Mosquito is a JavaScript framework for audio programming and live coding. It's written in Rust
//! and it uses the sound compiler [Csound](https://csound.com) as the audio engine and
//! [QuickJS](https://bellard.org/quickjs/) as the JavaScript runtime.

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts
)]
#![warn(
    deprecated_in_future,
    missing_docs,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unreachable_pub
)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
