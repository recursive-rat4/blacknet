## Blacknet Reference Software

Blacknet is an experimental peer-to-peer network based on blockchain technology.
[Website](https://blacknet.ninja/).


## How to build

#### Setup the environment

Install the Java JDK (not only JRE) version 8th number or greater.

- Debian & Ubuntu: `sudo apt-get install default-jdk`
- Gentoo: `sudo emerge -av1 --noreplace virtual/jdk`
- Arch Linux: `sudo pacman -S --needed jdk-openjdk`

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
- Windows: use `gradlew.bat` in place of `./gradlew`


## How to contribute

We accept contributions from anyone in the Universe.
File a theoretical or practical [issue](https://gitlab.com/blacknet-ninja/blacknet/issues), or send changes as a [pull request](https://gitlab.com/blacknet-ninja/blacknet/-/merge_requests).


## License

This program is distributed under the terms of the Jelurida Public License
version 1.1 for the Blacknet Public Blockchain Platform. See [LICENSE.txt](LICENSE.txt).
