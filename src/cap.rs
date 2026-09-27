/// How the ends of a stroked spinner's arc are drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Cap {
    Flat,
    #[default]
    Round,
}

pub const DEFAULT_CAP: Cap = Cap::Round;

impl Cap {
    pub const ALL: [Cap; 2] = [Cap::Flat, Cap::Round];
}
