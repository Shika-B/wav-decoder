# wav-decoder
A toy .wav file format decoder in Rust.
My goal is just to play around with Fast Fourier Transform algorithm(s).

My main reference for the specification of the format is:
http://soundfile.sapp.org/doc/WaveFormat/

Todo-list:
- [x] .wav decoder
- [ ] Implement FFT (Fast Fourier Transform) 
- [ ] Basic sound management (turn up/down the sound, stuff like that)
- [ ] A GUI to plot the whole thing ?
- [ ] Actual error handling (not panicking on every invalid file) ?