Mosquito
===

Mosquito is an audio library for live coding, composition, sound design, DSP
prototyping and any kind of audio applications.

It's written for a small and easy to learn scripting language
[Koto](https://github.com/koto-lang/koto), which is easy to embed and designed
for audio live coding and stuff in mind (in fact, the author of Koto is 
[Ian Hobson](https://twitter.com/_hobson_), who previously worked at Ableton).

Mosquito uses [Csound](https://csound.com) as a sound compiler and it's
permissively licensed, allowing it to be used in proprietary software. It can be
either used on it's own or embedded into your application. Notice though, that
Csound is linked dynamically.




## Build

Additional steps are needed for 
[csound-rs](https://github.com/neithanmo/csound-rs) to build this project, check
out instructions [here](https://github.com/neithanmo/csound-rs#installation).

More details for installing/building Csound library, you'll find in 
[Csound repo](https://github.com/csound/csound).
