pub struct Filter {
    fil_val: f32,
    k: f32,
}

impl Filter {
    #[must_use]
    pub fn new(k: f32) -> Self {
        Self { fil_val: 0.0, k }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn filter_value(&mut self, val: u16) -> u16 {
        let val: f32 = val.into();
        let a = (val - self.fil_val) * self.k;
        self.fil_val += a;
        self.fil_val as u16
    }
}
