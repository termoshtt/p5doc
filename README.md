p5doc
======
[![main](https://img.shields.io/badge/docs-main-blue)](https://termoshtt.github.io/p5doc/p5doc_example/index.html)

[p5.js](https://p5js.org/) diagram within rustdoc

Example
-------

This crate introduces a proc-macro [`#[p5doc::p5doc]`](https://termoshtt.github.io/p5doc/p5doc/attr.p5doc.html), which converts `p5doc` inline code in Markdown into a HTML flagment with p5.js script:

```rust
#[cfg_attr(doc, p5doc::p5doc)]
/// Some function!
///
/// Before
///
/// ```p5doc:200x100
/// background(220);
/// ellipse(50,50,80,80);
/// ```
///
/// After
///
pub fn some() {}
```

This will be displayed as following:

![image](https://user-images.githubusercontent.com/1238153/223720335-bdf1e9a3-8a7b-43a4-ac2d-f188c90cd944.png)

See [the document on GitHub Pages](https://termoshtt.github.io/p5doc/p5doc_example/fn.some.html) and [its code](./p5doc-example/src/lib.rs).

License
--------

© 2023 Toshiki Teramura (@termoshtt)

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
