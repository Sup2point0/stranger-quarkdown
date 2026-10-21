use super::super::ui::*;

use enum_display::EnumDisplay;

use std::io;


#[derive(EnumDisplay)] #[enum_display(case = "Title")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Features
{
	PAGE_TS,
	SITE_DATA,
	ASSETS,
}


pub fn enable_features() -> io::Result<()>
{
	ask!("What features would you like to use?");

	let choices = pick_many! {
		[true] Features::PAGE_TS => "Render +page.ts files alongside +page.svx files";
	};

	Ok(())
}
