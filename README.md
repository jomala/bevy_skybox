# Bevy Skybox

[Bevy](https://docs.rs/bevy) now provides a Skybox component that can be attached to cameras to provide a background in all directions: see [examples/3d/skybox.rs](https://github.com/bevyengine/bevy/blob/main/examples/3d/skybox.rs). What this crate provides is a simple bit of image processing to turn the sort of skybox "net" image that you find on the internet into one suitable for the Bevy Skybox. It is therefore designed primarily for fast prototyping.

This crate assumes that the input image is of the same format as the examples given in the `assets` folder:
a net of six squares in the same shape as these examples. It also assumes that you only need one skybox at any given time, but that you might want to apply it to multiple cameras.

## Usage

The usage is documented in the module comment at the top of `src/lib.rs`.

To demonstrate this, `examples\board_flyover.rs` puts a skybox around a flat "board". Key/mouse camera movement is provided by `bevy_fly_camera`.

![Board Flyover example](docs/board_flyover.png)

Skybox images come from the following sources.

* [**sky1.png**](https://www.cleanpng.com/png-skybox-cube-mapping-texture-mapping-terragen-textu-1384141)
* [**sky2.png**](https://www.cleanpng.com/png-skybox-texture-mapping-cube-mapping-sky-cloud-920475) (flipped)

## Image processing

Many skybox are available as net images. `bevy_skybox` assumes that the image is a specific net
of a cube.

The assumptions about the image are listed in `src/image.rs`, but the image is measured like this.

![Measuring the cube net](docs/measuring_the_net.png)

It assumes the net is exactly aligned with the image
rectangle and filling most of its width and height.

Only PNG images are supported currently.

## Build

Build using `stable` or `nightly` toolchain, e.g.

```sh
cargo run --release --example board_flyover
```

## Compatibility

Currently compatible with Bevy 0.15.

Raise an issue or PR if you need support for other versions of Bevy.
