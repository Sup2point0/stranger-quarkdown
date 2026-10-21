use super::super::ui::*;

use crate::colours::*;

use enum_display::EnumDisplay;

use std::io;


#[derive(EnumDisplay)] #[enum_display(case = "Title")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Features
{
	#[display("Render +page.ts")]
	PAGE_TS,
	SITE_DATA,
	ASSETS,
}


pub fn enable_features() -> io::Result<Vec<Features>>
{
	ask!("What features would you like to use?");

	let choices = pick_many!
	{
		[true] Features::PAGE_TS
			=> "Render +page.ts files alongside +page.svx files";
		[true] Features::SITE_DATA
			=> "Produce a JSON file containing aggregated site data";
		[true] Features::ASSETS
			=> "Copy assets from anywhere in your repo to static/";
	}?;

	Ok(choices)
}
