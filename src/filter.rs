const K: f32 = 0.1;

pub struct Filter {
    val: f32,
}

impl Filter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            val: f32::default(),
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn filter_value(&mut self, val: u16) -> u16 {
        let val: f32 = val.into();
        let new_val = self.val + (val - self.val) * K;
        self.val = new_val;
        self.val as u16
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
