#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Segment {
    a,    //  aaa
    b,    // b   c
    c,    // b   c
    d,    //  ddd
    e,    // e   f
    f,    // e   f
    g,    //  ggg
}

impl Segment {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'a' => Segment::a,
            'b' => Segment::b,
            'c' => Segment::c,
            'd' => Segment::d,
            'e' => Segment::e,
            'f' => Segment::f,
            'g' => Segment::g,
            _ => panic!(),
        }
    }
}
