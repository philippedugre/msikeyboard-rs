#[derive(PartialEq, Eq, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_u32(rgb: u32) -> Color {
        Color::new(
            ((rgb >> 16) & 0xFF) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
        )
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn hue(mut hue: u32) -> Color {
        if hue >= 360 {
            hue %= 360;
        }

        let cu = ((hue % 60) * 255 / 60) as u8;
        let cd = 255 - cu;

        let (r, g, b);

        match hue / 60 {
            0 => {
                r = 255;
                g = cu;
                b = 0;
            }
            1 => {
                r = cd;
                g = 255;
                b = 0;
            }
            2 => {
                r = 0;
                g = 255;
                b = cu;
            }
            3 => {
                r = 0;
                g = cd;
                b = 255;
            }
            4 => {
                r = cu;
                g = 0;
                b = 255;
            }
            5 => {
                r = 255;
                g = 0;
                b = cd;
            }
            _ => {
                r = 0;
                g = 0;
                b = 0;
            }
        }

        return Color::new(r, g, b);
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    #[test]
    fn test_color_accessors() {
        let c = Color { r: 1, g: 2, b: 3 };
        assert_eq!(c.r(), 1);
        assert_eq!(c.g(), 2);
        assert_eq!(c.b(), 3);
    }

    #[test]
    fn test_new_color() {
        let c = Color::new(4, 5, 6);
        assert_eq!(c.r(), 4);
        assert_eq!(c.g(), 5);
        assert_eq!(c.b(), 6);
    }

    #[test]
    fn test_color_equals() {
        let a = Color::new(7, 8, 9);
        let b = Color::new(7, 8, 9);
        let c = Color::new(10, 12, 11);

        assert!(a == b);
        assert!(a != c);
    }
}
