//! Mosquito is an audio library for live coding, composition, sound design, DSP prototyping and
//! any kind of audio applications.
//! 
//! It's written for a small and easy to learn scripting language
//! [Koto](https://github.com/koto-lang/koto), which is easy to embed and designed for audio live
//! coding and stuff in mind (in fact, the author of Koto is 
//! [Ian Hobson](https://twitter.com/_hobson_), who previously worked at Ableton).
//!
//! Mosquito uses [Csound](https://csound.com) as a sound compiler and it's permissively licensed,
//! allowing it to be used in proprietary software. It can be either used on its own or embedded
//! into your application. Notice though, that Csound is linked dynamically.

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
