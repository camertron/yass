use magnus::{Ruby, typed_data};
use style::values::specified::Percentage;

#[magnus::wrap(class = "Yass::Font::Metrics::Override")]
pub struct YFontMetricsOverride {
    percentage: Percentage
}

impl YFontMetricsOverride {
    pub fn new(percentage: Percentage) -> Self {
        Self { percentage }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.percentage.get()
    }
}

#[magnus::wrap(class = "Yass::Font::Metrics::OverrideNormal")]
pub struct YFontMetricsOverrideNormal {}

impl YFontMetricsOverrideNormal {
    pub fn new() -> Self {
        Self {}
    }
}
