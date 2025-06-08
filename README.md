# Bulk Watermarker

A fast and flexible CLI tool to apply watermarks to images in bulk. Built with Rust for speed and reliability.

> Developed by [Malshan Bandara](https://github.com/your-username)

---

## Features

- Bulk process entire folders of images
- Supports `.jpg`, `.jpeg`, `.png`
- Rotate watermark or output image (90° left or right)
- Resizes watermark to fit each image
- Simple and fast command-line interface

---

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/your-username/bulk-watermarker
Replace with the real GitHub URL after you push the code.

Usage
bash
Copy
Edit
bulk <INPUT_FOLDER> <OUTPUT_FOLDER> <WATERMARK_IMAGE> [options]
Example:
bash
Copy
Edit
bulk ./input ./output ./logo.png -q -r
CLI Options
Option	Description
-q	Rotate watermark 90° left
-e	Rotate watermark 90° right
-w	Rotate final image 90° left
-r	Rotate final image 90° right

Only one of -q / -e and one of -w / -r can be used at a time.

Supported Formats
.jpg

.jpeg

.png

Output
All watermarked images will be saved in the specified output folder and prefixed with tagged-.

Example:

lua
Copy
Edit
input/
├── photo1.jpg
├── photo2.png

output/
├── tagged-photo1.jpg
├── tagged-photo2.png
Build from Source
bash
Copy
Edit
git clone https://github.com/your-username/bulk-watermarker
cd bulk-watermarker
cargo build --release
The binary will be in target/release/bulk.

License
This project is licensed under the MIT License.

Contributing
Pull requests and issues are welcome. Feel free to suggest improvements or request features.