## 0.5.0 (2021-07-21)

* Add "props" demo (thanks gipsy-king)
* Use depth stencil, not near/far of camera (thank gipsy-king!)

## 0.4.0 (2021-04-11)

* Updated for Bevy v0.5

## 0.3.0 (2021-01-14)

* Shaders are no longer used to light the skybox
* A secondary camera is added and used for the skybox with a longer draw distance.
* Take the image from the centre of the border pixels, not the outside edge.
* Increase the tolerance for poor images.
* Improve the links in crates.io and the documentation.

## 0.2.0 (2021-01-08)

* The lighting on skybox now uniform.
* The example uses a light (for the board) relative to camera, rather than misusing SkyboxBox.
* The interface to image processing module has changed to a PbrBundle.
* Keywords and a category have been added for crates.io

## 0.1.0 (2021-01-04)

* Initial release