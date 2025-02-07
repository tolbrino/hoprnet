#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct Lorem {
    // `default` is incompatible with `field.build`
    #[builder(
        default = "1",
        field(build = "self.ipsum.map(|v| v + 42).unwrap_or(100)")
    )]
    ipsum: usize,

    // Both `ty` and `type` are temporarily allowed to ease the transition
    // to syn 2.0, but they are mutually exclusive.
    #[builder(field(ty = "usize", type = "usize"))]
    dolor: usize,

    // `default` is incompatible with `field.ty`, even without `field.build`
    #[builder(default = "2", field(ty = "usize"))]
    sit: usize,

    // Both errors can occur on the same field
    #[builder(default = "3", field(ty = "usize", build = "self.ipsum + 42"))]
    amet: usize,
}

fn main() {}
