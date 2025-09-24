#[derive(Debug, Clone, Copy)]
pub enum Handfuls {
    Simple,
    Double,
    Triple,
}

impl Handfuls {
    pub fn points(&self) -> usize {
        match self {
            Handfuls::Simple => 20,
            Handfuls::Double => 40,
            Handfuls::Triple => 60,
        }
    }

    pub fn trumps_required(&self) -> usize {
        match self {
            Handfuls::Simple => 10,
            Handfuls::Double => 13,
            Handfuls::Triple => 15,
        }
    }
}
