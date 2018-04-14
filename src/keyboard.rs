use color::Color;
use hidapi::{HidApi, HidDevice, HidError};

mod functions {
    pub const SET: u8 = 0x40;
    pub const GRAD: u8 = 0x44;
}

mod modes {
    pub const NORMAL: u8 = 0x01;
    pub const GAMING: u8 = 0x02;
    pub const BREATHING: u8 = 0x03;
    //pub const AUDIO: u8 = 0x04; //TODO
    pub const WAVE: u8 = 0x05;
    //pub const DUAL_COLOR: u8 = 0x06; //TODO
}

mod regions {
    pub const LEFT: u8 = 0x01;
    pub const MIDDLE: u8 = 0x02;
    pub const RIGHT: u8 = 0x03;
}

pub struct Keyboard{
    device: HidDevice
}

impl<'a> Keyboard {
    pub fn new() -> Result<Keyboard, HidError> {
        let device = HidApi::new()?.open(0x1770, 0xFF00)?;
        Ok(Keyboard { device })
    }

    pub fn normal(&self, left: Color, middle: Color, right: Color) -> Result<(), HidError> {
        self.write_color(functions::SET, regions::LEFT, left)?;
        self.write_color(functions::SET, regions::MIDDLE, middle)?;
        self.write_color(functions::SET, regions::RIGHT, right)?;
        self.write_mode(modes::NORMAL)?;
        Ok(())
    }

    pub fn gaming(&self, left: Color) -> Result<(), HidError> {
        self.write_color(functions::SET, regions::LEFT, left)?;
        self.write_mode(modes::GAMING)?;
        Ok(())
    }

    pub fn breathing<T: IntoColorTuple, U: IntoColorTuple, V: IntoColorTuple>(
        &self,
        left: T,
        middle: U,
        right: V,
    ) -> Result<(), HidError> {
        let l = left.into();
        let m = middle.into();
        let r = right.into();
        self.write_gradient(regions::LEFT, l.0, l.1)?;
        self.write_gradient(regions::MIDDLE, m.0, m.1)?;
        self.write_gradient(regions::RIGHT, r.0, r.1)?;
        self.write_mode(modes::BREATHING)?;
        Ok(())
    }

    pub fn wave<T: IntoColorTuple, U: IntoColorTuple, V: IntoColorTuple>(
        &self,
        left: T,
        middle: U,
        right: V,
    ) -> Result<(), HidError> {
        let l = left.into();
        let m = middle.into();
        let r = right.into();
        self.write_gradient(regions::LEFT, l.0, l.1)?;
        self.write_gradient(regions::MIDDLE, m.0, m.1)?;
        self.write_gradient(regions::RIGHT, r.0, r.1)?;
        self.write_mode(modes::WAVE)?;
        Ok(())
    }

    fn write_color(&self, function: u8, zone: u8, color: Color) -> Result<(), HidError> {
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

        self.device.send_feature_report(&data)?;
        Ok(())
    }

    fn write_gradient(&self, region: u8, first: Color, second: Color) -> Result<(), HidError> {
        let index = (region - 1) * 3 + 1;
        self.write_color(functions::GRAD, index, first)?;
        self.write_color(functions::GRAD, index + 1, second)?;
        self.write_color(functions::GRAD, index + 2, Color::new(0x03, 0x03, 0x03))?;
        Ok(())
    }

    fn write_mode(&self, mode: u8) -> Result<(), HidError> {
        let data = [0x01, 0x02, 0x41, mode, 0x00, 0x00, 0x00, 0x00];

        self.device.send_feature_report(&data)?;
        Ok(())
    }
}

pub trait IntoColorTuple {
    fn into(self) -> (Color, Color);
}

impl IntoColorTuple for Color {
    fn into(self) -> (Color, Color) {
        (self, Color::new(0, 0, 0))
    }
}

impl IntoColorTuple for (Color, Color) {
    fn into(self) -> (Color, Color) {
        self
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use keyboard::Keyboard;
    #[test]
    fn test_write_color() {
        // Uncomment line to test a method.
        let _c1 = Color::new(255, 0, 0);
        let _c2 = Color::new(0, 255, 0);
        let _c3 = Color::new(0, 0, 255);

        let _c4 = Color::new(100, 200, 100);
        let _c5 = Color::new(100, 100, 200);
        let _c6 = Color::new(200, 200, 200);

        let _k = Keyboard::new().unwrap();

        //_k.normal(_c1, _c2, _c3).unwrap();
        //_k.gaming(_c2).unwrap();
        //_k.breathing(_c1, _c2, _c3).unwrap();
        //_k.wave(_c1, _c2, _c3).unwrap();

        //_k.breathing((_c1, _c4), (_c2, _c5), (_c3, _c6)).unwrap();
        //_k.wave((_c1, _c4), (_c2, _c5), (_c3, _c6)).unwrap();

        //_k.wave((_c1, _c4), _c5, (_c3, _c6)).unwrap();
    }
}
