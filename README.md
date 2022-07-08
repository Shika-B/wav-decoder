# wav-decoder
A toy .wav file format decoder in Rust.

My main reference for the specification of the format is:
http://soundfile.sapp.org/doc/WaveFormat/

A list of things the .wav decoder does not handle:
- Multiple channels (should be easy to fix that tho)
- Any audio format different from PCM (in particular, any compressed .wav)
- Samples with a size different than 16bits (shouldn't be hard to fix that too, it's just a matter of how the actual data bytes are parsed at the end)

Todo-list:
- [x] .wav decoder
- [ ] Basic sound management (turn up/down the sound, stuff like that)
- [ ] Actual error handling (not panicking on every invalid file) ?