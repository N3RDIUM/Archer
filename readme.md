# Archer

![Archer Logo](ArcherLogo.png)
A rusty ray tracing engine being built to run with a decent frame rate even on CPU, allowing for realtime raytracing applications without GPUs.
Every line of code is written with extensibility, hackability and the project's future in mind. Although it is kinda dirty at the moment, it will
get good eventually, possibly turning into the best ray tracer on the planet.

__Currently working on:__ Tiling/chunking for efficient parallelism, Optimisations, SDL2 Window rendering.

- [x] Basic Sphere [Devlog 1]
- [x] Multiple Reflections + BVH [Devlog 2]
- [ ] Optimisations [Devlog 3]
- [ ] Refraction [Devlog 4]
- [ ] More optimisations? [Devlog 5]
- [ ] More stuff coming soon!

## Latest Render

![Latest render](./archer/output.png)

Render time: ~20 seconds on Intel i5 11th gen (8 max bounces, 64 samples per pixel)

## Thanks and Citations

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html): for showing how to do stuff

- [_The Ray Tracing Road To Rust_](https://the-ray-tracing-road-to-rust.vercel.app/): for showing how to do stuff in rust
