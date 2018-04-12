use color::Color;
use hidapi::{HidApi};

struct Keyboard {
    api: HidApi,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        // TODO: Use the ? operator to send a result instead of unwraping.
        // TODO: Change the HidApi in the struct to a HidDevice
        let api = HidApi::new().unwrap();
        Keyboard { api }
    }

    fn write_color(&self, function: u8, zone: u8, color: &Color) {
        let data = [
            0x01,
            0x02,
            function,
            zone,
            color.r(),
            color.g(),
            color.b(),
            0x00,
        ];

        // TODO: Use the ? operator to send a result instead of unwraping
        self.api
            .open(0x1770, 0xFF00)
            .unwrap()
            .send_feature_report(&data);
    }

    fn write_mode(&self, mode: u8) {
        let data = [0x01, 0x02, 0x41, mode, 0x00, 0x00, 0x00, 0x00];

        // TODO: Use the ? operator to send a result instead of unwraping
        self.api
            .open(0x1770, 0xFF00)
            .unwrap()
            .send_feature_report(&data);
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use keyboard::Keyboard;
    #[test]
    fn test_write_color() {
        // Not a unit test. Should change keyboard to normal mode
        // with Red/Green/Blue pattern in this order
        let c1 = Color::new(255, 0, 0);
        let c2 = Color::new(0, 255, 0);
        let c3 = Color::new(0, 0, 255);
        let k = Keyboard::new();
        k.write_mode(0x01);
        k.write_color(0x40, 1, &c1);
        k.write_color(0x40, 2, &c2);
        k.write_color(0x40, 3, &c3);
    }
}
