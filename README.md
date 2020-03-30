## Blacknet Reference Software

[![Pipeline Status](https://gitlab.com/blacknet-ninja/blacknet/badges/master/pipeline.svg)](https://gitlab.com/blacknet-ninja/blacknet/pipelines)
[![Matrix](https://img.shields.io/matrix/blacknet:matrix.org)](https://riot.im/app/#/room/#blacknet:matrix.org)
[![Website](https://img.shields.io/website?url=https%3A%2F%2Fblacknet.ninja)](https://blacknet.ninja)

Blacknet is an experimental peer-to-peer network based on blockchain technology.

## How to build

#### Setup the environment

Install the Java JDK (not only JRE) version 8th number or greater.

- Debian & Ubuntu: `sudo apt-get install default-jdk git`
- Red Hat & Oracle: `sudo yum install java-11-openjdk git`
- SUSE: `sudo zypper install java-11-openjdk git`
- FreeBSD: `sudo pkg install openjdk11 git`
- Gentoo: `sudo emerge -av1 --noreplace virtual/jdk dev-vcs/git`
- Arch Linux: `sudo pacman -S --needed jdk-openjdk git`

#### Get the source code

```
git clone https://gitlab.com/blacknet-ninja/blacknet
cd blacknet
```

#### Make the build

```
./gradlew build
```

To run the built program, use `./gradlew run`
- Windows: use `.\gradlew.bat` in place of `./gradlew`


## How to contribute

We accept contributions from anyone in the Universe.
File a theoretical or practical [issue](https://gitlab.com/blacknet-ninja/blacknet/issues), or send changes as a [pull request](https://gitlab.com/blacknet-ninja/blacknet/-/merge_requests).


## License

This program is distributed under the terms of the Jelurida Public License
version 1.1 for the Blacknet Public Blockchain Platform. See [LICENSE.txt](LICENSE.txt).
