#[derive(Clone, PartialEq, Eq)]
pub enum Context {
    Static(CtxStatic),
    Stream(CtxStream),
}

#[derive(Clone, PartialEq, Eq)]
pub struct CtxStatic;

impl CtxStatic {

}

#[derive(Clone, PartialEq, Eq)]
pub struct CtxStream;