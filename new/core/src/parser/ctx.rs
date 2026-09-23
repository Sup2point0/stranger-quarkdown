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


macro_rules! ctx
{
	($self:ident, $ctx:expr => $body:block) => {
		{
			$self.ctx.push($ctx);
			let r = { $body };
			$self.ctx.try_pop($ctx)?;
			r
		}
	};
}
pub(crate) use ctx;
