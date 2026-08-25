#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctx {
	CODE_INLINE,
	CODE_BLOCK,
	COMMENT,
	SQUARK_LEAVE,
	SQUARK_SLASH,
	SQUARK_ONLY,
}
