# Text Comprehension Index

The Text Comprehension Index library provides a tool to calculate the readability score for Hungarian texts based on the Szövegértési mutató formula. This formula helps determine the ease of understanding a given text, making it useful for writers, educators, and content creators who want to ensure their material is accessible to their intended audience.

## Features

- Calculate the Text Comprehension Index for Hungarian texts.
- Automatically count sentences, words, and syllables in the text.
- Provides a score clamped between 0 and 100 for easy interpretation.
- Includes comprehensive tests for accuracy and reliability.


### Explanation:

1.  **Text Comprehension Index Formula**:

    -   The assumed formula used is: Szo¨vegeˊrteˊsi mutatoˊ=206.835-(1.015×Words per Sentence)-(84.6×Syllables per Word)\text{Szövegértési mutató} = 206.835 - (1.015 \times \text{Words per Sentence}) - (84.6 \times \text{Syllables per Word})Szo¨vegeˊrteˊsi mutatoˊ=206.835-(1.015×Words per Sentence)-(84.6×Syllables per Word).
2.  **Helper Methods**:

    -   `count_sentences`: Counts the number of sentences using regex.
    -   `count_words`: Counts the number of words using regex.
    -   `count_syllables`: Counts the number of syllables using regex by matching vowel groups.
3.  **Score Clamping**:

    -   We use `score.clamp(0.0, 100.0)` to ensure that the Text Comprehension Index is within a reasonable range.


[![Crates.io](https://img.shields.io/crates/v/szovegertesimutato.svg)](https://crates.io/crates/szovegertesimutato)
[![Docs.rs](https://docs.rs/szovegertesimutato/badge.svg)](https://docs.rs/szovegertesimutato)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install szovegertesimutato`

## License

Licensed under:

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
