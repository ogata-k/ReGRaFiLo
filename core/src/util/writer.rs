pub trait WriteAsJson {
    fn write_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
