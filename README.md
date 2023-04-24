# mondrians

![GitHub deployments](https://img.shields.io/github/deployments/albertlarsan68/mondrians/github-pages?label=deployment)
![GitHub](https://img.shields.io/github/license/albertlarsan68/mondrians)
![GitHub Sponsors](https://img.shields.io/github/sponsors/albertlarsan68?logo=github)
![Liberapay patrons](https://img.shields.io/liberapay/patrons/AlbertLarsan68?logo=liberapay)

![example](https://user-images.githubusercontent.com/74931857/217534598-3b2e41ae-bc26-4f00-9968-f6ea716de653.png)

Generates uniquely deterministic images, in the style of Mondrian.

This project is split in three parts:
1. A library that takes in the parameters of the generation and the seeds, and returns a base64 encoded binary blob containing the image in PNG format.
2. A web page that uses the library and Wasm to generate the image in a browser.
3. A WIP binary that does the same as the web page, but without a browser.

## Algorithm

Start with a blank canvas.

You have a one over `max depth` + 5 chance of stopping there, and choosing a random color between White, Red, Yellow or Blue.

Choose a middle point between `min middle` and `max middle` in the X axis.  
Create a black line through the whole height, between your chosen middle point - `sep width` and your chosen middle point + `sep width`.

For each half:
- You have a one over `max depth` + 5 chance of stopping there, and filling your part with a random color chosen between White, Red, Yellow or Blue.
- Choose a middle point between `min middle` and `max middle` in the Y axis.  
- Create a black line through the whole half's width, between your chosen middle point - `sep width` and your chosen middle point + `sep width`.
- Reduce `max depth` by one
- For each half half:
  - You have a one over `max depth` + 5 chance of stopping there, and filling your part with a random color chosen between White, Red, Yellow or Blue.
  - Choose a middle point between `min middle` and `max middle` in the x axis.  
  - Create a black line through the whole half's width, between your chosen middle point - `sep width` and your chosen middle point + `sep width`.
  - Reduce `max depth` by one
  - Repeat until `max depth` is 0.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
