# `modern-libr`

This is a MVP for a bindings strategy that aims to provide a complete
binding of R's C-headers.

With this approach, the R-headers can be made into rust modules.

See the following images of the resulting `cargo doc`

![Front page](image.png)

![Inside of `bindings`](image-1.png)

![This is how `R_ext` looks like.](image-2.png)


## TODO

- [ ] Compile and even run `cargo build --features generate_bindings` on
    other platforms than Windows. Report back new headers, etc.,
    and properly feature-gate them (and make them into modules).
