## Blacknet Reference Software

[![Pipeline Status](https://gitlab.com/blacknet-ninja/blacknet/badges/master/pipeline.svg)](https://gitlab.com/blacknet-ninja/blacknet/pipelines)
[![Web Chat](https://img.shields.io/matrix/blacknet:matrix.org)](https://app.element.io/#/group/+blacknet:matrix.org)
[![Web Site](https://img.shields.io/website?url=https%3A%2F%2Fblacknet.ninja)](https://blacknet.ninja)

Blacknet is a public peer-to-peer network based on blockchain technology with proof of stake consensus mechanism.

## Get the source code

[Release tags](https://gitlab.com/blacknet-ninja/blacknet/-/tags)
`git clone https://gitlab.com/blacknet-ninja/blacknet.git`

## Setup the environment

Install the Java JDK (not only JRE) version 8th number or greater.

- Debian & Ubuntu: `sudo apt-get install default-jdk git`
- Red Hat & Oracle: `sudo yum install java-11-openjdk git`
- SUSE: `sudo zypper install java-11-openjdk git`
- Arch GNU/Linux: `sudo pacman -S --needed jdk-openjdk git`
- Gentoo: `sudo emerge -av1 --noreplace virtual/jdk dev-vcs/git`
- FreeBSD: `sudo pkg install openjdk11 git`
- OpenBSD: `sudo pkg_add jdk git`

## Make the build

```
./gradlew installDist
```

The built program is in `./build/install/`
To run use `./blacknet`, or on Windows use `.\blacknet.bat`
Gradle `:run` task is supported but not recommended.

## How to contribute

You are welcome to report a theoretical or practical [issue](https://gitlab.com/blacknet-ninja/blacknet/issues),
or send changes as a [GitLab pull request](https://gitlab.com/blacknet-ninja/blacknet/-/merge_requests) to the master branch.

## License

This program is distributed under the terms of the Jelurida Public License
version 1.1 for the Blacknet Public Blockchain Platform. See [LICENSE.txt](LICENSE.txt).
