# Wacky Pixels


<a id="readme-top"></a>
<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#examples">Examples</a></li>
    <li><a href="#screenshots">Screenshots</a></li>
    <li><a href="#built-with">Built With</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

### Abstract
WackyPixels takes an image and encodes it through a configurable pipeline of transformations, each adding a layer of obfuscation. The default pipeline:
1. Serializes your image to binary data
2. Embeds it in a PDF's /Info metadata
3. Compresses it with LZMA
4. Encodes it as multi-mode Unicode characters
5. Converts it to WAV audio file

### What do I take out of this?
Have fun with it! Coding doesn't have to be all corporate all the time!!

<p align="right">(<a href="#readme-top">back to top</a>)</p>
<!-- INSTALLATION -->

## Installation

### Prerequisites
- Rust 1.70+ ([install Rust](https://rustup.rs/))

### Build from Source
```bash
# Clone the repository
git clone https://github.com/PixelSaver/wackypixels.git
cd wackypixels

# Build release binary
cargo build --release

# Binary will be at ./target/release/wackypixels
```

### Install Globally 
> [!IMPORTANT] 
> I don't recommend this. This is an experimental fun, chaotic program, not something you should use on your filesystem. If you really want to, just know I warned you.
```bash
cargo install --path .

# Now you can use 'wackypixels' from anywhere
wackypixels --help
```

## Usage

For all of the below prompts, if you did not install and are running the development environment, replace all instances of `wackypixels` with `cargo run --`
For example,
```bash
wackypixels run 
# Should become
cargo run -- run
```


### Quick Start
This will:
1. Clean output/ and decrypted/ directories
2. Encode input.png through all stages
3. Decode it back to decrypted/decrypted.png
4. Save all intermediate files
```bash
# Run the full default pipeline (encode + decode)
wackypixels run -i input.png
```

### Basic Commands
```bash
# Encode an image
wackypixels encode -i image.png -o output/

# Decode back to original
wackypixels decode -i output/encrypted.wav -o decrypted/

# Clean output directories
wackypixels clean

# List available transformations
wackypixels list
```

### Creating Custom Pipelines
The `--pipeline` flag or the `-p` flag works on `encode`, `decode`, and `run`.
You can specify your own transformation pipeline:
```bash
# Lightweight: Just compression + unicode
wackypixels encode --pipeline image,lzma,unicode

# Maximum compression: Double compress!
wackypixels encode --pipeline image,lzma,unicode,gzip

# Audio without unicode
wackypixels encode --pipeline image,pdf,lzma,wav

# High-density WAV 
wackypixels encode --pipeline image,lzma,wav
```

### Pipeline Design Tips

#### For reasonable file sizes:
- Avoid WAV audio encoding 
- Compress!

#### For maximum cursedness:
- Everything: `image,pdf,lzma,unicode,wav,gzip`
- Image is now 6.5x

<p align="right">(<a href="#readme-top">back to top</a>)</p>
<!-- EXAMPLES -->

## Examples

### Example 1: Quick Encode/Decode
```bash
# Encode
wackypixels encode -i cat.png -o outputs/

# Decode
wackypixels decode -i outputs/encrypted.wav
```

### Example 2: Custom Lightweight Pipeline
```bash
# Just compression and unicode (no audio bloat)
wackypixels encode --pipeline image,lzma,unicode -i photo.jpg

# Result: ~120% of original size instead of 650%
```

### Example 3: Batch Processing
Why are you doing this???
```bash
#!/bin/bash
# Encode all PNGs in a directory

for file in images/*.png; do
    filename=$(basename "$file" .png)
    wackypixels encode -i "$file" -o "output/$filename/" -s false
done
```

### Example 5: Inspect Intermediate Stages
```bash
# Encode with intermediates saved
wackypixels encode -i secret.png -o debug/

# Check what each stage produces
ls -lh debug/
# 001_image_serialization.bin   52 KB
# 002_pdf_/info.pdf              53 KB
# 003_lzma_compression.xz        31 KB
# 004_unicode_encoding.txt       61 KB
# 005_wav_audio.wav             2.0 MB
# encrypted.wav                 2.0 MB

# Can decode from any intermediate stage!
wackypixels decode -i debug/003_lzma_compression.xz \
    --pipeline lzma,unicode,wav
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- SCREENSHOTS -->
## Screenshots

### Terminal Output
```bash
# Encode with intermediates saved
wackypixels encode -i secret.png -o debug/

# Check what each stage produces
ls -lh debug/
# 001_image_serialization.bin    52 KB
# 002_pdf.pdf                    53 KB
# 003_lzma_compression.xz        31 KB
# 004_unicode_encoding.txt       61 KB
# 005_wav_audio.wav             2.0 MB
# encrypted.wav                 2.0 MB

# Can decode from any intermediate stage!
wackypixels decode -i debug/003_lzma_compression.xz \
    --pipeline lzma,unicode,wav
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- SCREENSHOTS -->
## Screenshots
Not yet

<details>
  <summary><strong>Not yet</strong></summary>
  <img src="media/img2.png" alt="Screenshot of the added simulation controls">
</details>

> [!TIP]
> Use `--save-intermediates false` to skip saving intermediate files if you only care about the final output.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Built With
[Zed](https://zed.dev) - Main code editor
[Rust](https://www.rust-lang.org/) - Systems programming language
[Clap](https://github.com/clap-rs/clap) - Command-line argument parser
[Hound](https://github.com/ruuda/hound) - WAV encoding/decoding
[XZ2](https://github.com/alexcrichton/xz2-rs) - LZMA compression
[lopdf](https://github.com/J-F-Liu/lopdf) - PDF manipulation
[image](https://github.com/image-rs/image) - Image processing
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

### Current Features

 - [X] Multi-stage encoding pipeline
 - [X] Custom pipeline composition
 - [X] Intermediate file inspection
 - [X] Multiple compression algorithms
 - [X] Unicode variable-bit encoding
 - [X] WAV audio encoding
 - [X] CLI interface

### Planned Features

- [ ] QR code encoding stage
- [ ] Steganography in images
- [ ] Spectrogram image generation
- [ ] Base65536 encoding
- [ ] Polyglot file generation
- [ ] GUI interface
- [ ] Web-based demo
- [ ] Encryption layer (AES)
- [ ] Progressive streaming decode
- [ ] Parallel pipeline processing

### Notes
Why is this useful? It's not. Thanks for checking this out!

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/PixelSaver/wackypixels](https://github.com/PixelSaver/wackypixels)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments
- [Rust documentation](https://doc.rust-lang.org/) - For easy learning!
- [Wacky Files YSWS from Hackclub](https://wacky.hackclub.com) - For the inspiration!

---

Just because you can encode your image as a pdf as an xz as unicode as a wav file, doesn't mean you should. But where's the fun in that? :)

<p align="right">(<a href="#readme-top">back to top</a>)</p>