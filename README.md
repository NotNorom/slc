# Some Light Control

Control esp based led strips.
The protocol being used can be found here: [https://git.j3d1.de/jedi/microgl](https://git.j3d1.de/jedi/microgl).

## Usage

```
slc --help
```
will open the help page.


```
slc esp0:1234 -n42 -r255 -g255 -b255 -w100
```
will set 42 pixels of esp0 to red=255, green=255, blue=255 and white=100.


## Dependencies

At the time of writing I was using:
* rustc 1.24.0 (4d90ac38c 2018-02-12)
* cargo 0.25.0 (8c93e0895 2018-02-01)

## Installation
0. Clone this repo
1. Install the rust programming language (including cargo) according to your distro.
    I reccommend visiting this website: [https://rustup.rs/](https://rustup.rs/)
2. Run ```cargo install``` inside the cloned directory
3. Done!