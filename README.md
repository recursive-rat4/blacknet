# Blacknet Full Node Wallet

[![Pipeline Status][]](https://gitlab.com/blacknet-ninja/blacknet/pipelines)
[![Web Chat][]](https://app.element.io/#/room/#blacknet-space:matrix.org)
[![Web Site][]](https://blacknet.ninja)

**Blacknet is decentralized peer-to-peer network that secures public blockchain platform with proof of stake consensus.**

## Get the Source Code

- [Release Tags][]
- `git clone https://gitlab.com/blacknet-ninja/blacknet.git`

## Setup the Environment

Install the Java JDK (not only JRE) version 21th number or greater.

- Debian & Ubuntu: `sudo apt-get install default-jdk git`
- Red Hat & Oracle: `sudo yum install java-21-openjdk git`
- SUSE: `sudo zypper install java-21-openjdk git`
- Arch GNU/Linux: `sudo pacman -S --needed jdk-openjdk git`
- Gentoo: `sudo emerge -av1 --noreplace virtual/jdk dev-vcs/git`
- FreeBSD: `sudo pkg install openjdk21 git`
- OpenBSD: `sudo pkg_add jdk git`

## Make the Build

```
./gradlew installDist
```

The built program is in `./build/install/`

To run use `./blacknet-daemon`, or on Windows use `.\blacknet-daemon.bat`

Gradle `:run` task is supported but not recommended.

## How to Contribute

You are welcome to report a theoretical or practical [Issue][],
or send changes as a [Pull Request][] to the master branch.

## License

This program is distributed under the terms of the Jelurida Public License
version 1.1 for the Blacknet Public Blockchain Platform. See the [LICENSE][] file.

[Issue]: https://gitlab.com/blacknet-ninja/blacknet/issues
[LICENSE]: https://gitlab.com/blacknet-ninja/blacknet/-/blob/master/LICENSE.txt
[Pipeline Status]: https://gitlab.com/blacknet-ninja/blacknet/badges/master/pipeline.svg
[Pull Request]: https://gitlab.com/blacknet-ninja/blacknet/-/merge_requests
[Release Tags]: https://gitlab.com/blacknet-ninja/blacknet/-/tags
[Web Chat]: https://img.shields.io/matrix/blacknet:matrix.org
[Web Site]: https://img.shields.io/website?url=https%3A%2F%2Fblacknet.ninja
