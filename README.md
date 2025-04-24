# imgtools

A command-line image processing tool written in Rust that provides various image manipulation functionalities.

## Features

- Format conversion
- Image flipping (horizontal/vertical)
- Image rotation (90°/180°/270°)
- Image resizing with multiple filter options
- Grayscale conversion
- Blur effects (Gaussian/Fast)
- Brightness adjustment
- Hue rotation
- Contrast adjustment
- Image cropping with multiple position options
- Color inversion
- Image sharpening
- Watermark addition (text/image)

## Installation

```bash
cargo install imgtools
```

## Usage

Basic command structure:
```bash
imgtools -i <input_file> [-o <output_file>] <command> [options]
```

If output file is not specified, it will modify the input file directly.

### Examples

1. Convert image format:
```bash
imgtools -i input.jpg -o output.png convert -f png
```

2. Flip image:
```bash
imgtools -i input.jpg -o output.jpg flip -h  # horizontal flip
imgtools -i input.jpg -o output.jpg flip -v  # vertical flip
```

3. Rotate image:
```bash
imgtools -i input.jpg -o output.jpg rotate -r 90
```

4. Resize image:
```bash
imgtools -i input.jpg -o output.jpg resize -w 800 -h 600 -f lanczos3
```

5. Apply blur effect:
```bash
imgtools -i input.jpg -o output.jpg blur -s 3.0 -f  # fast blur
imgtools -i input.jpg -o output.jpg blur -s 3.0     # gaussian blur
```

6. Adjust brightness:
```bash
imgtools -i input.jpg -o output.jpg brighten -v 50  # increase brightness
imgtools -i input.jpg -o output.jpg brighten -v -50 # decrease brightness
```

7. Add watermark:
```bash
# Add text watermark
imgtools -i input.jpg -o output.jpg watermark -p center -r 45 text -t "Copyright" -c white -s 50

# Add image watermark
imgtools -i input.jpg -o output.jpg watermark -p bottom-right image watermark.png
```
8. Adjust hue:
```bash
imgtools -i input.jpg -o output.jpg huerotate -v 90  # rotate hue by 90 degrees
```

9. Adjust contrast:
```bash
imgtools -i input.jpg -o output.jpg contrast -v 1.5  # increase contrast
imgtools -i input.jpg -o output.jpg contrast -v 0.5  # decrease contrast
```

10. Crop image:
```bash
# Crop from center
imgtools -i input.jpg -o output.jpg crop -c "center(500,300)"

# Crop from custom position
imgtools -i input.jpg -o output.jpg crop -c "custom(100,100,500,300)"

# Crop from corners
imgtools -i input.jpg -o output.jpg crop -c "topleft(500,300)"
imgtools -i input.jpg -o output.jpg crop -c "bottomright(500,300)"
```

11. Invert colors:
```bash
imgtools -i input.jpg -o output.jpg invert
```

12. Unsharpen image:
```bash
imgtools -i input.jpg -o output.jpg unsharpen -s 3.0 -t 5
```

13. Convert to grayscale:
```bash
imgtools -i input.jpg -o output.jpg grayscale
```

### Available Commands and Options

#### Format Conversion
- Supported formats: PNG, JPEG, WebP, BMP, AVIF, TIFF

#### Resize Filters
- nearest: Nearest neighbor
- triangle: Linear interpolation
- catmullrom: Cubic interpolation
- gaussian: Gaussian filtering
- lanczos3: Lanczos with radius 3

#### Watermark Positions
- center (default)
- top-left
- top-center
- top-right
- middle-left
- middle-right
- bottom-left
- bottom-center
- bottom-right
- custom(x,y): Custom coordinates
- flat-lay(spacing): Tiled watermark with specified spacing

#### Text Watermark Colors
- Preset colors: white (default), black, red, green, blue
- Custom color: rgba(r,g,b,a) where r,g,b,a are in range 0-255

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.