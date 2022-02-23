<img src="https://raw.githubusercontent.com/maciejhirsz/logos/master/logos.svg?sanitize=true" alt="Logos logo" width="250" align="right">

# Logos

You can find documentation for this fork [here](https://kaylynn234.github.io/logos/). The venerable crates.io
unfortunately doesn't support namespacing, so I'm unable to publish this fork there. You'll have to point cargo at this
git repository instead, which is easy enough. In your `Cargo.toml`,
```toml
[dependencies]
logos = { git = "https://github.com/kaylynn234/logos", tag = "v0.13.0" }
```

See [this page in the reference](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for more detail
about how Cargo behaves in regards to Git repositories. 

## Why a fork?
It's important for me to mention that **the vast majority of the code here is not authored by me**. This is a *fork* of
Logos, and I do not want to steal any credit for the fantastic work done by its contributors. If you like the project,
you should support it by [buying the maintainer a coffee](https://github.com/maciejhirsz). 

I've mostly forked this to make changes for *myself*, but I hope that others find these changes useful too. I would
normally open an issue or make a PR, but it appears that the project's maintainer has been quite busy lately - at the
time of writing, work on the project hasn't really happened in the past eight months.

To be more specific, the changes I've made stem from a couple gripes that I ran into while using Logos myself:
- I don't like encoding failures with an error variant. We have `Result` for that!
- I don't like the options available at present for emitting errors within callbacks.
- I think some of the documentation could be improved, or otherwise made a bit clearer.
- "looking ahead" with Logos is currently very frustrating. `Peekable<I>` doesn't allow any access to the internal
  operator, so accessing fields of the lexer (such as `source`) becomes impossible. 

## What about the changes you made?

I ran benchmarks on both this fork and the upstream repository, and rest assured that even *after* my changes, Logos is
still just as fast. Hooray!

## Acknowledgements

- [Pedrors](https://pedrors.pt/) for the Logos logo.
- All upstream contributors, and especially [Maciej](https://maciej.codes/), for making Logos what it is today.

## Thank you

Logos is very much a labor of love, so consider
[buying the maintainer a coffee](https://github.com/sponsors/maciejhirsz). ☕

## License

Logos is licensed under the terms of both the MIT license and the Apache License (Version 2.0), at your option.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
