use clap::{Parser, Subcommand};
use image::imageops::FilterType;
use std::path::PathBuf;
use std::str::FromStr;

/// Image Processing
#[derive(Parser, Debug)]
pub struct Cli {
    /// Input image file path
    #[arg(long, short = 'i')]
    pub input: PathBuf,
    /// Output image file path (optional)
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Available image processing commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert image format
    Convert {
        /// Target format
        #[arg(long, short = 'f')]
        format: Format,
    },
    /// Flip image
    Flip {
        /// Whether to flip horizontally
        #[arg(long, short = 'h')]
        horizontal: bool,
        /// Whether to flip vertically
        #[arg(long, short = 'v')]
        vertical: bool,
    },
    /// Rotate image
    Rotate {
        /// Rotation angle
        #[arg(long, short = 'r')]
        rotate: Rotate,
    },
    /// Resize image
    Resize {
        /// Target width
        #[arg(long, short = 'w')]
        width: u32,
        /// Target height
        #[arg(long, short = 'h')]
        height: u32,
        /// Whether to force exact size
        #[arg(long, short = 'e')]
        exact: bool,
        /// Scaling filter type
        #[arg(long, short = 'f')]
        filter: Filter,
    },
    /// Convert to grayscale
    Grayscale,
    /// Blur processing
    Blur {
        /// Standard deviation for Gaussian blur
        #[arg(long, short = 's')]
        sigma: f32,
        /// Whether to use fast blur algorithm
        #[arg(long, short = 'f')]
        fast: bool,
    },
    /// Adjust brightness
    Brighten {
        /// Brightness adjustment value (positive increases brightness, negative decreases brightness)
        #[arg(long, short = 'v')]
        value: i32,
    },
    /// Adjust hue
    Huerotate {
        /// Hue rotation angle
        #[arg(long, short = 'v')]
        value: i32,
    },
    /// Adjust contrast
    Contrast {
        /// Contrast adjustment value
        #[arg(long, short = 'v')]
        value: f32,
    },
    /// Crop image
    Crop {
        /// Crop parameters
        #[arg(long, short = 'c')]
        crop: Crop,
    },
    /// Invert image colors
    Invert,
    /// Sharpen processing
    Unsharpen {
        /// Standard deviation for Gaussian blur
        #[arg(long, short = 's')]
        sigma: f32,
        /// Sharpening threshold
        #[arg(long, short = 't')]
        threshold: i32,
    },
    /// Add watermark
    Watermark {
        /// Watermark position
        ///
        /// Supports the following position options:
        /// - center: Center (default)
        /// - top-left: Top left corner
        /// - top-center: Top center
        /// - top-right: Top right corner
        /// - middle-left: Middle left
        /// - middle-right: Middle right
        /// - bottom-left: Bottom left corner
        /// - bottom-center: Bottom center
        /// - bottom-right: Bottom right corner
        /// - custom(x,y): Custom coordinate position
        /// - flat-lay(spacing): Tiled mode (spacing between watermarks)
        #[arg(long, short = 'p', default_value = "center")]
        position: Position,
        /// Watermark rotation angle
        ///
        /// Rotation angle of the watermark, default is 0.0, range (0.0 ~ 360.0)
        #[arg(long, short = 'r', default_value_t = 0.0)]
        rotate: f32,
        /// Watermark margin
        ///
        /// Pixel distance from the watermark to the edge, default is 20 pixels
        #[arg(long, short = 'm', default_value_t = 20)]
        margin: u32,
        /// Watermark mode
        #[command(subcommand)]
        command: Watermark,
    },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[default]
    Png,
    Jpeg,
    WebP,
    Bmp,
    Avif,
    Tiff,
}

impl FromStr for Format {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(Format::Png),
            "jpeg" | "jpg" => Ok(Format::Jpeg),
            "webp" => Ok(Format::WebP),
            "bmp" => Ok(Format::Bmp),
            "avif" => Ok(Format::Avif),
            "tiff" => Ok(Format::Tiff),
            _ => Err("Unsupported image formats"),
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Png => "png".to_string(),
            Format::Jpeg => "jpeg".to_string(),
            Format::WebP => "webp".to_string(),
            Format::Bmp => "bmp".to_string(),
            Format::Avif => "avif".to_string(),
            Format::Tiff => "tiff".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Rotate {
    #[default]
    Rotate90,
    Rotate180,
    Rotate270,
}

impl FromStr for Rotate {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "90" | "rotate-90" => Ok(Rotate::Rotate90),
            "180" | "rotate-180" => Ok(Rotate::Rotate180),
            "270" | "rotate-270" => Ok(Rotate::Rotate270),
            _ => Err("Unsupported rotation angle, only supports 90/180/270 degree rotation"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    #[default]
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl Into<FilterType> for Filter {
    fn into(self) -> FilterType {
        match self {
            Filter::Nearest => FilterType::Nearest,
            Filter::Triangle => FilterType::Triangle,
            Filter::CatmullRom => FilterType::CatmullRom,
            Filter::Gaussian => FilterType::Gaussian,
            Filter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

impl FromStr for Filter {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nearest" => Ok(Filter::Nearest),
            "triangle" => Ok(Filter::Triangle),
            "catmullrom" | "catmull-rom" => Ok(Filter::CatmullRom),
            "gaussian" => Ok(Filter::Gaussian),
            "lanczos3" => Ok(Filter::Lanczos3),
            _ => {
                Err("Unsupported filter types, only nearest/triangle/catmullrom/gaussian/lanczos3")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Crop {
    Center(u32, u32),
    TopLeft(u32, u32),
    TopCenter(u32, u32),
    TopRight(u32, u32),
    MiddleLeft(u32, u32),
    MiddleRight(u32, u32),
    BottomLeft(u32, u32),
    BottomCenter(u32, u32),
    BottomRight(u32, u32),
    Custom(u32, u32, u32, u32),
}

impl FromStr for Crop {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        if let Some((name, nums)) = s.split_once('(') {
            if !nums.ends_with(')') {
                return Err("Format error: Need to end with ')'");
            }

            let nums = nums.trim_end_matches(')');
            let nums: Vec<u32> = nums
                .split(',')
                .map(|n| n.trim().parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "The parameter must be a non-negative integer")?;

            match (name.trim(), nums.as_slice()) {
                ("center", &[w, h]) => Ok(Crop::Center(w, h)),
                ("topleft", &[w, h]) => Ok(Crop::TopLeft(w, h)),
                ("topcenter", &[w, h]) => Ok(Crop::TopCenter(w, h)),
                ("topright", &[w, h]) => Ok(Crop::TopRight(w, h)),
                ("middleleft", &[w, h]) => Ok(Crop::MiddleLeft(w, h)),
                ("middleright", &[w, h]) => Ok(Crop::MiddleRight(w, h)),
                ("bottomleft", &[w, h]) => Ok(Crop::BottomLeft(w, h)),
                ("bottomcenter", &[w, h]) => Ok(Crop::BottomCenter(w, h)),
                ("bottomright", &[w, h]) => Ok(Crop::BottomRight(w, h)),
                ("custom", &[x, y, w, h]) => Ok(Crop::Custom(x, y, w, h)),
                _ => Err("Format error: The number of parameters does not match"),
            }
        } else {
            Err("Format error: Missing '('")
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Watermark {
    /// Add watermark
    Text {
        /// Text content to add
        ///
        /// Supports both English and Chinese characters, recommend using short text
        #[arg(long, short = 't')]
        text: String,

        /// Font file path
        ///
        /// Supports ttf/otf font files. If not specified, will use built-in FangSong font
        #[arg(long, short = 'f')]
        font: Option<PathBuf>,

        /// Font scale ratio
        ///
        /// Controls text size, default is 50.0. Larger value means bigger text
        #[arg(long, short = 's', default_value_t = 50.0)]
        scale: f32,

        /// Text color
        ///
        /// Supports following color options:
        /// - Preset colors: white(default), black, red, green, blue
        /// - Custom color: rgba(r,g,b,a) - r,g,b range 0-255, a range 0-255 for transparency
        #[arg(long, short = 'c', default_value = "white")]
        color: Color,
    },
    Image {
        /// Watermark image file path
        image: PathBuf,
    },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    #[default]
    Center,
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom(u32, u32),
    FlatLay(usize),
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "center" => Ok(Position::Center),
            "top-left" => Ok(Position::TopLeft),
            "top-center" => Ok(Position::TopCenter),
            "top-right" => Ok(Position::TopRight),
            "middle-left" => Ok(Position::MiddleLeft),
            "middle-right" => Ok(Position::MiddleRight),
            "bottom-left" => Ok(Position::BottomLeft),
            "bottom-center" => Ok(Position::BottomCenter),
            "bottom-right" => Ok(Position::BottomRight),
            _ => {
                // Handle custom(x,y) format
                if s.starts_with("custom(") && s.ends_with(")") {
                    // Extract content within parentheses
                    let coords = s[7..s.len() - 1].split(',').collect::<Vec<&str>>();

                    if coords.len() == 2 {
                        // Try to parse coordinate values
                        match (
                            coords[0].trim().parse::<u32>(),
                            coords[1].trim().parse::<u32>(),
                        ) {
                            (Ok(x), Ok(y)) => Ok(Position::Custom(x, y)),
                            _ => Err(
                                "Invalid custom position format. Expected numbers for coordinates."
                                    .to_string(),
                            ),
                        }
                    } else {
                        Err(
                            "Invalid custom position format. Expected format: custom(x,y)"
                                .to_string(),
                        )
                    }
                }
                // Handle flat-lay(n) format
                else if s.starts_with("flat-lay(") && s.ends_with(")") {
                    // Extract content within parentheses
                    let n = s[9..s.len() - 1].trim();
                    match n.parse::<usize>() {
                        Ok(value) => Ok(Position::FlatLay(value)),
                        Err(_) => Err("Invalid flat-lay format. Expected a number.".to_string()),
                    }
                } else {
                    Err(format!("Unknown position: {}", s))
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    White,
    Black,
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, u8),
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "white" => Ok(Color::White),
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            s if s.starts_with("rgba(") && s.ends_with(')') => {
                let content = &s[5..s.len() - 1];
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

                if parts.len() != 4 {
                    return Err(format!(
                        "Invalid RGBA format: {}. Expected rgba(r,g,b,a)",
                        s
                    ));
                }

                let parse_component = |s: &str| -> Result<u8, String> {
                    s.parse()
                        .map_err(|_| format!("Invalid color component: {}", s))
                };

                let r = parse_component(parts[0])?;
                let g = parse_component(parts[1])?;
                let b = parse_component(parts[2])?;
                let a = parse_component(parts[3])?;

                Ok(Color::Rgba(r, g, b, a))
            }
            _ => Err(format!(
                "Invalid color: {}. Expected one of: white, black, red, green, blue, rgba(r,g,b,a)",
                s
            )),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_crop_formats() {
        // Test center crop
        assert_eq!(
            "center(100,200)".parse::<Crop>().unwrap(),
            Crop::Center(100, 200)
        );

        // Test corner crops
        assert_eq!(
            "topleft(100,200)".parse::<Crop>().unwrap(),
            Crop::TopLeft(100, 200)
        );
        assert_eq!(
            "topcenter(100,200)".parse::<Crop>().unwrap(),
            Crop::TopCenter(100, 200)
        );
        assert_eq!(
            "topright(100,200)".parse::<Crop>().unwrap(),
            Crop::TopRight(100, 200)
        );

        // Test middle positions
        assert_eq!(
            "middleleft(100,200)".parse::<Crop>().unwrap(),
            Crop::MiddleLeft(100, 200)
        );
        assert_eq!(
            "middleright(100,200)".parse::<Crop>().unwrap(),
            Crop::MiddleRight(100, 200)
        );

        // Test bottom positions
        assert_eq!(
            "bottomleft(100,200)".parse::<Crop>().unwrap(),
            Crop::BottomLeft(100, 200)
        );
        assert_eq!(
            "bottomcenter(100,200)".parse::<Crop>().unwrap(),
            Crop::BottomCenter(100, 200)
        );
        assert_eq!(
            "bottomright(100,200)".parse::<Crop>().unwrap(),
            Crop::BottomRight(100, 200)
        );

        // Test custom position crop
        assert_eq!(
            "custom(10,20,100,200)".parse::<Crop>().unwrap(),
            Crop::Custom(10, 20, 100, 200)
        );

        // Test case insensitivity
        assert_eq!(
            "CENTER(100,200)".parse::<Crop>().unwrap(),
            Crop::Center(100, 200)
        );
        assert_eq!(
            "TopLeft(100,200)".parse::<Crop>().unwrap(),
            Crop::TopLeft(100, 200)
        );
    }

    #[test]
    fn test_invalid_crop_formats() {
        // Test missing parentheses
        assert!("center100,200".parse::<Crop>().is_err());

        // Test unmatched parentheses
        assert!("center(100,200".parse::<Crop>().is_err());

        // Test incorrect number of parameters
        assert!("center(100)".parse::<Crop>().is_err());
        assert!("center(100,200,300)".parse::<Crop>().is_err());
        assert!("custom(10,20,100)".parse::<Crop>().is_err());

        // Test invalid numbers
        assert!("center(abc,200)".parse::<Crop>().is_err());
        assert!("center(-100,200)".parse::<Crop>().is_err());

        // Test unknown crop position
        assert!("unknown(100,200)".parse::<Crop>().is_err());

        // Test format errors
        assert!("center(100;200)".parse::<Crop>().is_err());
        assert!("center(100, )".parse::<Crop>().is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        // Test extra whitespace
        assert_eq!(
            "center ( 100 , 200 )".parse::<Crop>().unwrap(),
            Crop::Center(100, 200)
        );
        assert_eq!(
            "custom( 10 , 20 , 100 , 200 )".parse::<Crop>().unwrap(),
            Crop::Custom(10, 20, 100, 200)
        );
    }

    #[test]
    fn test_color_parsing() {
        assert!(matches!("white".parse::<Color>().unwrap(), Color::White));
        assert!(matches!("Black".parse::<Color>().unwrap(), Color::Black));
        assert!(matches!("RED".parse::<Color>().unwrap(), Color::Red));
        assert!(matches!(
            "rgba(255,0,128,64)".parse::<Color>().unwrap(),
            Color::Rgba(255, 0, 128, 64)
        ));
        assert!(matches!(
            "RGBA(10, 20, 30, 40)".parse::<Color>().unwrap(),
            Color::Rgba(10, 20, 30, 40)
        ));

        assert!("invalid".parse::<Color>().is_err());
        assert!("rgba(256,0,0,0)".parse::<Color>().is_err());
        assert!("rgba(1,2,3)".parse::<Color>().is_err());
    }
    #[test]
    fn test_position_from_str_fixed_positions() {
        // Test all fixed positions
        assert_eq!(Position::from_str("center").unwrap(), Position::Center);
        assert_eq!(Position::from_str("top-left").unwrap(), Position::TopLeft);
        assert_eq!(
            Position::from_str("top-center").unwrap(),
            Position::TopCenter
        );
        assert_eq!(Position::from_str("top-right").unwrap(), Position::TopRight);
        assert_eq!(
            Position::from_str("middle-left").unwrap(),
            Position::MiddleLeft
        );
        assert_eq!(
            Position::from_str("middle-right").unwrap(),
            Position::MiddleRight
        );
        assert_eq!(
            Position::from_str("bottom-left").unwrap(),
            Position::BottomLeft
        );
        assert_eq!(
            Position::from_str("bottom-center").unwrap(),
            Position::BottomCenter
        );
        assert_eq!(
            Position::from_str("bottom-right").unwrap(),
            Position::BottomRight
        );
    }

    #[test]
    fn test_position_from_str_case_insensitive() {
        // Test case insensitivity
        assert_eq!(Position::from_str("CENTER").unwrap(), Position::Center);
        assert_eq!(Position::from_str("Top-Left").unwrap(), Position::TopLeft);
        assert_eq!(
            Position::from_str("BOTTOM-RIGHT").unwrap(),
            Position::BottomRight
        );
    }

    #[test]
    fn test_position_from_str_custom() {
        // Test custom coordinates
        assert_eq!(
            Position::from_str("custom(0,0)").unwrap(),
            Position::Custom(0, 0)
        );
        assert_eq!(
            Position::from_str("custom(1,2)").unwrap(),
            Position::Custom(1, 2)
        );
        assert_eq!(
            Position::from_str("custom(10,20)").unwrap(),
            Position::Custom(10, 20)
        );

        // Test custom coordinates with whitespace
        assert_eq!(
            Position::from_str("custom(1, 2)").unwrap(),
            Position::Custom(1, 2)
        );
        assert_eq!(
            Position::from_str("custom( 1 , 2 )").unwrap(),
            Position::Custom(1, 2)
        );
    }

    #[test]
    fn test_position_from_str_errors() {
        // Test invalid inputs
        assert!(Position::from_str("invalid").is_err());
        assert!(Position::from_str("custom").is_err());
        assert!(Position::from_str("custom()").is_err());
        assert!(Position::from_str("custom(1)").is_err());
        assert!(Position::from_str("custom(1,)").is_err());
        assert!(Position::from_str("custom(,1)").is_err());
        assert!(Position::from_str("custom(a,1)").is_err());
        assert!(Position::from_str("custom(1,b)").is_err());
        assert!(Position::from_str("custom(1,2,3)").is_err());
    }
}
