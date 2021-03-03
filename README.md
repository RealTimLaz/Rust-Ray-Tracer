# Rust-Ray-Tracer
A Ray Tracer written in Rust. Based off of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

The renderer supports:
 - Spheres
 - Antialiasing
 - Diffuse, Metal and Dielectric materials
 - Moveable Camera
 - Depth of Field
 
To improve performance, it uses the _rayon_ library to allow for multithreaded rendering. This sees performance improvements of around 5x.

![example render](https://github.com/MasterObvious/Rust-Ray-Tracer/raw/main/output/Book%201.png)


