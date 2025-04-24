use ab_glyph::{FontRef, PxScale};
use clap::Parser;
use image::codecs::avif::AvifEncoder;
use image::codecs::bmp::BmpEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::tiff::TiffEncoder;
use image::codecs::webp::WebPEncoder;
use image::imageops::overlay;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, ImageReader, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use imageproc::geometric_transformations::{Interpolation, rotate_about_center};
use imgtools::{Cli, Color, Command, Crop, Format, Position, Rotate, Watermark};
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::PathBuf;

fn main() {
    // Parse command line arguments
    let Cli {
        input,
        output,
        command,
    } = Cli::parse();

    // Open and decode the input image
    let mut img = match ImageReader::open(input.clone()) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to decode image: {}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Failed to read image: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            return;
        }
    };

    // Get image dimensions and color type
    let width = img.width();
    let height = img.height();
    let color_type: ExtendedColorType = img.color().into();

    // Extract input file name and paths
    let input_file_name = match input.file_name() {
        Some(name) => name,
        None => {
            eprintln!("Failed to get input file name");
            return;
        }
    };
    let input_file_name = PathBuf::from(input_file_name);
    let input_path = match input.parent() {
        Some(path) => path.to_path_buf(),
        None => {
            eprintln!("Failed to get parent path");
            return;
        }
    };
    let output_path = output.unwrap_or(input_path);

    // Process the command
    match command {
        // Convert image to different format
        Command::Convert { format } => {
            let output = match output_path.is_dir() || output_path.as_os_str().is_empty() {
                true => {
                    let output_file_name = input_file_name.with_extension(format.to_string());
                    output_path.with_file_name(output_file_name)
                }
                false => output_path,
            };
            let output = match File::create(output) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create output file: {}", e);
                    return;
                }
            };
            let mut output = BufWriter::new(output);

            // Handle different output formats
            match format {
                Format::Jpeg => {
                    let mut encoder = JpegEncoder::new(output);
                    if let Err(e) = encoder.encode(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
                Format::Png => {
                    let encoder = PngEncoder::new(output);
                    if let Err(e) = encoder.write_image(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
                Format::WebP => {
                    let encoder = WebPEncoder::new_lossless(output);
                    if let Err(e) = encoder.encode(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
                Format::Bmp => {
                    let mut encoder = BmpEncoder::new(&mut output);
                    if let Err(e) = encoder.encode(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
                Format::Avif => {
                    let encoder = AvifEncoder::new(output);
                    if let Err(e) = encoder.write_image(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
                Format::Tiff => {
                    let encoder = TiffEncoder::new(output);
                    if let Err(e) = encoder.encode(img.as_bytes(), width, height, color_type) {
                        eprintln!("Failed to encode image: {}", e);
                        return;
                    }
                }
            }
            return;
        }
        // Flip image horizontally and/or vertically
        Command::Flip {
            horizontal,
            vertical,
        } => {
            img = match (horizontal, vertical) {
                (true, true) => img.fliph().flipv(),
                (true, false) => img.fliph(),
                (false, true) => img.flipv(),
                (false, false) => img,
            };
        }
        // Rotate image by fixed angles
        Command::Rotate { rotate } => {
            img = match rotate {
                Rotate::Rotate90 => img.rotate90(),
                Rotate::Rotate180 => img.rotate180(),
                Rotate::Rotate270 => img.rotate270(),
            };
        }
        // Resize image with optional exact dimensions
        Command::Resize {
            width,
            height,
            exact,
            filter,
        } => {
            img = match exact {
                true => img.resize_exact(width, height, filter.into()),
                false => img.resize(width, height, filter.into()),
            };
        }
        // Convert image to grayscale
        Command::Grayscale => {
            img = img.grayscale();
        }
        // Apply blur effect
        Command::Blur { sigma, fast } => {
            img = match fast {
                true => img.fast_blur(sigma),
                false => img.blur(sigma),
            };
        }
        // Adjust image brightness
        Command::Brighten { value } => {
            img = img.brighten(value);
        }
        // Rotate image hue
        Command::Huerotate { value } => {
            img = img.huerotate(value);
        }
        // Adjust image contrast
        Command::Contrast { value } => {
            img = img.adjust_contrast(value);
        }
        // Crop image with various positioning options
        Command::Crop { crop } => {
            let (x, y, w, h) = match crop {
                Crop::Center(w, h) => {
                    let x = (width - w) / 2;
                    let y = (height - h) / 2;
                    (x, y, w, h)
                }
                Crop::TopLeft(w, h) => (0, 0, w, h),
                Crop::TopCenter(w, h) => {
                    let x = (width - w) / 2;
                    (x, 0, w, h)
                }
                Crop::TopRight(w, h) => {
                    let x = width - w;
                    (x, 0, w, h)
                }
                Crop::MiddleLeft(w, h) => {
                    let y = (height - h) / 2;
                    (0, y, w, h)
                }
                Crop::MiddleRight(w, h) => {
                    let x = width - w;
                    let y = (height - h) / 2;
                    (x, y, w, h)
                }
                Crop::BottomLeft(w, h) => {
                    let y = height - h;
                    (0, y, w, h)
                }
                Crop::BottomCenter(w, h) => {
                    let x = (width - w) / 2;
                    let y = height - h;
                    (x, y, w, h)
                }
                Crop::BottomRight(w, h) => {
                    let x = width - w;
                    let y = height - h;
                    (x, y, w, h)
                }
                Crop::Custom(x, y, w, h) => (x, y, w, h),
            };

            img = img.crop_imm(x, y, w, h);
        }
        // Invert image colors
        Command::Invert => {
            img.invert();
        }
        // Apply unsharp mask
        Command::Unsharpen { sigma, threshold } => {
            img = img.unsharpen(sigma, threshold);
        }
        // Add watermark (text or image)
        Command::Watermark {
            position,
            rotate,
            margin,
            command,
        } => {
            // Validate rotation angle
            if !(0.0..=360.0).contains(&rotate) {
                eprintln!(
                    "Rotation value {} is out of valid range (0.0 to 360.0)",
                    rotate
                );
                return;
            }

            let rotate = rotate / 180.0 * PI;

            // Create watermark from text or image
            let watermark = match command {
                Watermark::Text {
                    text,
                    font,
                    scale,
                    color,
                } => {
                    // Load font data
                    let font_data = match font {
                        Some(f) => {
                            let font = match File::open(f) {
                                Ok(f) => f,
                                Err(e) => {
                                    eprintln!("Unable to open font file: {}", e);
                                    return;
                                }
                            };
                            match font.bytes().collect::<Result<Vec<u8>, _>>() {
                                Ok(fd) => fd,
                                Err(e) => {
                                    eprintln!("Unable to read font file: {}", e);
                                    return;
                                }
                            }
                        }
                        None => {
                            let font_data = include_bytes!("../data/仿宋_GB2312.ttf").as_slice();
                            font_data.to_vec()
                        }
                    };

                    // Parse font
                    let font = match FontRef::try_from_slice(&font_data) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("Unable to parse font file: {}", e);
                            return;
                        }
                    };

                    // Set text properties
                    let scale = PxScale::from(scale);
                    let color = match color {
                        Color::White => Rgba([255, 255, 255, 255]),
                        Color::Black => Rgba([0, 0, 0, 255]),
                        Color::Red => Rgba([255, 0, 0, 255]),
                        Color::Green => Rgba([0, 255, 0, 255]),
                        Color::Blue => Rgba([0, 0, 255, 255]),
                        Color::Rgba(r, g, b, a) => Rgba([r, g, b, a]),
                    };

                    // Create text watermark
                    let (text_w, text_h) = text_size(scale, &font, &text);
                    let diagonal = ((text_w.pow(2) + text_h.pow(2)) as f32).sqrt().ceil() as u32;
                    let mut watermark = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(diagonal, diagonal);
                    let center_x = diagonal / 2 - text_w / 2;
                    let center_y = diagonal / 2 - text_h / 2;
                    draw_text_mut(
                        &mut watermark,
                        color,
                        center_x as i32,
                        center_y as i32,
                        scale,
                        &font,
                        &text,
                    );
                    watermark
                }
                // Load image watermark
                Watermark::Image { image } => match ImageReader::open(&image) {
                    Ok(reader) => match reader.with_guessed_format() {
                        Ok(reader) => match reader.decode() {
                            Ok(img) => img.into_rgba8(),
                            Err(e) => {
                                eprintln!("Failed to decode watermark image: {}", e);
                                return;
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to read watermark image: {}", e);
                            return;
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to open watermark image: {}", e);
                        return;
                    }
                },
            };

            // Rotate watermark
            let rotated = rotate_about_center(
                &watermark,
                rotate,
                Interpolation::Nearest,
                Rgba([0, 0, 0, 0]),
            );

            let (w, h) = (rotated.width(), rotated.height());
            // Handle flat lay pattern
            if let Position::FlatLay(spacing) = position {
                for y in (0..height).step_by(spacing) {
                    for x in (0..width).step_by(spacing) {
                        overlay(&mut img, &rotated, x as i64, y as i64);
                    }
                }

                // Save the processed image
                let output = match output_path.is_dir() || output_path.as_os_str().is_empty() {
                    true => output_path.with_file_name(input_file_name),
                    false => output_path,
                };

                if let Err(e) = img.save(output) {
                    eprintln!("Failed to save image: {}", e);
                    return;
                }

                return;
            }

            // Position watermark
            let (x, y) = match position {
                Position::Center => ((width - w) / 2, (height - h) / 2),
                Position::TopLeft => (margin, margin),
                Position::TopCenter => ((width - w) / 2, margin),
                Position::TopRight => (width - w - margin, margin),
                Position::MiddleLeft => (margin, (height - h) / 2),
                Position::MiddleRight => (width - w - margin, (height - h) / 2),
                Position::BottomLeft => (margin, height - h - margin),
                Position::BottomCenter => ((width - w) / 2, height - h - margin),
                Position::BottomRight => (width - w - margin, height - h - margin),
                Position::Custom(x, y) => (x, y),
                Position::FlatLay(_) => unreachable!(),
            };

            overlay(&mut img, &rotated, x as i64, y as i64);
        }
    }

    // Save the processed image
    let output = match output_path.is_dir() || output_path.as_os_str().is_empty() {
        true => output_path.with_file_name(input_file_name),
        false => output_path,
    };

    if let Err(e) = img.save(output) {
        eprintln!("Failed to save image: {}", e);
        return;
    }
}
