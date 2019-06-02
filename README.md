# Auto Sage

A de-make
of [Zach Gage][zg]'s
[Sage Solitaire][sage]
by [Nat Knight](https://twitter.com/scriptsimian)

Published with permission.

[sage]: http://sagesolitaire.com/
[zg]: https://twitter.com/helvetica/

# About

[Sage Solitaire][sage] is a card game by [Zach Game][zg] published on iPhone and Android. I've played it a bunch and enjoyed it immensely, but in order to improve, I'd need to be able to count cards effectively.

My puny human brain balked at that idea, so I built this de-make that does the counting for me.

# Building

Auto Sage requires Rust 2018 (it was built with v 1.35.0). It also requires the [BearLibTerminal][blt] text-UI library; the pre-compiled binary for Windows is included in this repository.

[blt]: https://bitbucket.org/cfyzium/bearlibterminal/src/default/

In theory, if you've got Rust installed on Windows host, you should be able to build Auto Sage by cloning the project and running

    cargo build





