use enum_display::EnumDisplay;


#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, EnumDisplay)]
pub enum ParseCtx
{
	#[default]
	#[display("while parsing charm squark")]
	CHARM_SQUARK,

	#[display("while parsing identifier")]
	IDENT,
}
