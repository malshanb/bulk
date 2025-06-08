# Bulk Watermarker

A fast and flexible CLI tool to apply watermarks to images in bulk. Built with Rust for speed and reliability.

**Developed by Malshan Bandara**

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
cargo install --git https://github.com/malshanb/bulk.git
```

### Usage

```bash
bulk <INPUT_FOLDER> <OUTPUT_FOLDER> <WATERMARK_IMAGE> [options]
```

### Example

```bash
bulk ./input ./output ./logo.png -q -r
```

### CLI Options

Option	Description
-q	Rotate watermark 90° left
-e	Rotate watermark 90° right
-w	Rotate final image 90° left
-r	Rotate final image 90° right

> Only one of -q / -e and one of -w / -r can be used at a time.

#### Supported Formats

.jpg

.jpeg

.png