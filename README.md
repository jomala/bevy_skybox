# Bevy Skybox

Provides a skybox from a given image that can be attached to a [bevy](https://docs.rs/bevy) camera.

Assumes that the image is of the same format as the examples given in the `assets` folder:
a net of six squares in the same shape as these examples, exactly aligned with the image
rectangle and filling most of its width and height.

## Usage

See `examples\board_flyover.rs` for an example of its usage.

## Build

Build using `nightly` toolchain, e.g.

```
cargo +nightly run --release --example board_flyover
```

## Image processing

Many skybox are available as net images. `bevy_skybox` assumes that the image is a specific net
of a cube.

The assumptions about the image are listed in `src/image.rs`.

## Example

The example puts a skybox around a flat "board". Key/mouse camera movement is provided by `bevy_fly_camera`.

![Board Flyover example](docs/board_flyover.png)

Skybox images come from the following sources.

* **sky1.png** - https://www.cleanpng.com/png-skybox-cube-mapping-texture-mapping-terragen-textu-1384141
* **sky2.png** - https://www.cleanpng.com/png-skybox-texture-mapping-cube-mapping-sky-cloud-920475 (flipped)

## Futures

So far, this is suitable for demos, not production.

* Lighting is currently from the scene's light source. The `SkyboxBox` should have its own.
* Some may have a preferred orientation for their skybox that they want to set,
  and the XZ plane may not be horizontal for all.
* The `SkyboxBox` should have a different draw distance to the rest of the scene.
* Multiple `SkyboxCamera` objects should be handled better, at least as an error.
