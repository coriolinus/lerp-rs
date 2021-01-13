use std::fmt::Debug;

// Helper when working with floats to "round" them, so we can compare them better
pub fn round(d: &dyn Debug) -> String {
    format!("{:.1?}", d)
}
