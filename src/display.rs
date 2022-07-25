use core::fmt::{Display, Formatter, Result};

use crate::{gradient::Gradient as GradientGenerator, RGB};

/// Methods to colorize string with a gradient.
pub trait GradientStr {
    /// This function takes a list of colors, which represent a gradient,
    /// and colorizes the string.
    fn gradient<I>(&self, colors: I) -> GradientDisplay<'_, I>
    where
        I: IntoIterator<Item = RGB> + Clone,
        I::IntoIter: ExactSizeIterator + Clone;
}

impl GradientStr for str {
    fn gradient<I>(&self, colors: I) -> GradientDisplay<'_, I>
    where
        I: IntoIterator<Item = RGB>,
        I::IntoIter: ExactSizeIterator,
    {
        GradientDisplay::new(self, colors, ColorType::FOREGROUND)
    }
}

impl GradientStr for &str {
    fn gradient<I>(&self, colors: I) -> GradientDisplay<'_, I>
    where
        I: IntoIterator<Item = RGB>,
        I::IntoIter: ExactSizeIterator,
    {
        GradientDisplay::new(self, colors, ColorType::FOREGROUND)
    }
}

#[cfg(feature = "std")]
impl GradientStr for String {
    fn gradient<I>(&self, colors: I) -> GradientDisplay<'_, I>
    where
        I: IntoIterator<Item = RGB>,
        I::IntoIter: ExactSizeIterator,
    {
        GradientDisplay::new(self, colors, ColorType::FOREGROUND)
    }
}

/// A gradient string representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GradientDisplay<'a, I> {
    text: &'a str,
    colors: I,
    color_type: ColorType,
}

impl<'a, I> GradientDisplay<'a, I> {
    const fn new(text: &'a str, colors: I, color_type: ColorType) -> Self {
        Self {
            text,
            colors,
            color_type,
        }
    }

    /// Colorize background.
    /// 
    /// Default is foreground.
    pub const fn background(mut self) -> Self {
        self.color_type = ColorType::BACKGROUND;
        self
    }

    /// Colorize foreground.
    /// 
    /// It's a default option.
    pub const fn foreground(mut self) -> Self {
        self.color_type = ColorType::FOREGROUND;
        self
    }
}

impl<I> Display for GradientDisplay<'_, I>
where
    I: IntoIterator<Item = RGB> + Clone,
    I::IntoIter: ExactSizeIterator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        display_gradient(
            self.text,
            self.colors.clone().into_iter(),
            self.color_type,
            f,
        )
    }
}

fn display_gradient<I>(
    text: &str,
    colors: I,
    color_type: ColorType,
    f: &mut Formatter<'_>,
) -> Result
where
    I: Iterator<Item = RGB> + Clone + ExactSizeIterator,
{
    if colors.len() == 0 || text.is_empty() {
        return text.fmt(f);
    }

    let line_width = text.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    if line_width == 0 {
        return text.fmt(f);
    }

    let mut gradient_chunk = line_width;
    if colors.len() > 2 {
        gradient_chunk /= colors.len() - 1;
    }

    let mut gradient = None;
    let mut next_color = RGB::new(0, 0, 0);
    let mut _colores = colors.clone();
    let mut i = 1;
    for c in text.chars() {
        if c == '\n' {
            gradient = None;
            i = 1;
            c.fmt(f)?;
            continue;
        }

        // happens only on 1st iteration
        if gradient.is_none() {
            _colores = colors.clone();
            let c1: RGB = _colores.next().unwrap();
            let c2: RGB = _colores.next().unwrap_or(c1);
            next_color = c2;

            gradient = Some(GradientGenerator::new(c1, c2, gradient_chunk).into_iter());
        }

        let color = match gradient.as_mut().unwrap().next() {
            Some(color) => color,
            None => {
                i += 1;

                let mut gradient_length = gradient_chunk;
                let is_last_gradient = i + 1 == colors.len();
                if is_last_gradient {
                    gradient_length += line_width - gradient_chunk * i
                };

                let c1 = next_color;
                let c2 = _colores.next().unwrap();
                next_color = c2;

                if gradient_length == 0 {
                    gradient = Some(GradientGenerator::new(c1, c2, 0).into_iter());

                    c2
                } else {
                    // we make +1 to be able to skip
                    // 1st color because it was the last on prev iteration
                    gradient =
                        Some(GradientGenerator::new(c1, c2, gradient_length + 1).into_iter());
                    let gradient = gradient.as_mut().unwrap();
                    gradient.next().unwrap();
                    gradient.next().unwrap()
                }
            }
        };

        colorize_char(c, color, color_type, f)?;
    }

    Ok(())
}

fn colorize_char(
    c: char,
    RGB { r, g, b }: RGB,
    color_type: ColorType,
    f: &mut Formatter<'_>,
) -> Result {
    f.write_fmt(format_args!(
        "\x1b[{};2;{};{};{}m{}\x1b[0m",
        color_type.0, r, g, b, c
    ))
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ColorType(usize);

impl ColorType {
    const BACKGROUND: ColorType = ColorType(48);
    const FOREGROUND: ColorType = ColorType(38);
}
