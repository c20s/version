#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Change {
    Fix,
    Feature,
    Breaking,
}
