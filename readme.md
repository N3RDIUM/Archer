# Archer

![Archer Logo](ArcherLogo.png)
A rusty ray tracing engine built to run with a decent frame rate even on CPU, allowing for realtime raytracing applications without GPUs.
All the code in this repo is heavily commented so that it is easy to follow. When complete, it will have a three.js-like usage format.

__Currently working on:__ Multiple bounces, lambertian material, using the `nalgebra` and `bvh` crates instead of re-inventing the wheel

- [x] Basic Sphere [Devlog 1]
- [ ] Multiple Reflections + BVH [Devlog 2]
- [ ] Refraction [Devlog 3]
- [ ] More stuff coming soon!

## Latest Render
![Latest render](./archer/output.png)

## Thanks and Citations

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html): for showing how to do stuff

- [_The Ray Tracing Road To Rust_](https://the-ray-tracing-road-to-rust.vercel.app/): for showing how to do stuff in rust
