use super::yarn::Yarn;



pub(crate) struct Chunk<'a> {
    start: Yarn<'a>,
    contents: Vec<Yarn<'a>>, // Lines within chunk,
    end: Yarn<'a>,
    id: usize
}

impl<'a> Chunk<'a> {

    pub(crate) fn from_raw(raw: Yarn<'a>, id: usize) -> Self {}

    pub(crate) fn from_parts(start: Yarn<'a>, contents: Yarn<'a>, end: Yarn<'a>, id: usize) -> Self {

    }    
}

