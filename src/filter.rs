pub struct Filter {
    fil_val: f32,
    k: f32,
}

impl Filter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fil_val: 0.0,
            k: 0.5,
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn filter_value(&mut self, val: u16) -> u16 {
        let abs_diff = val.abs_diff(self.fil_val as u16);

        let val: f32 = val.into();
        let abs_diff: f32 = abs_diff.into();

        self.k = if abs_diff > 1.5 { 0.9 } else { 0.03 };
        let a = (val - self.fil_val) * self.k;
        self.fil_val += a;
        self.fil_val as u16
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
