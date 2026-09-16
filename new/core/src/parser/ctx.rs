use enum_display::EnumDisplay;


#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, EnumDisplay)]
pub enum ParseCtx
{
	#[default]
	#[display("while parsing charm squark")]
	CHARM_SQUARK,

	#[display("while extracting <h1> heading")]
	HEADING,

	#[display("while parsing flags")]
	FLAGS,

	#[display("while parsing fields")]
	FIELDS,

	#[display("while parsing a field")]
	FIELD,

	#[display("while parsing identifier")]
	IDENT,

	#[display("while parsing values")]
	VALUES,
}
