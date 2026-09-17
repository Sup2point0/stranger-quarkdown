use super::*;
use crate::core::*;
use crate::log;
use crate::utils;
use crate::colours::*;

use std::fs::File;
use std::io::{ BufWriter, Write };


impl Renderer<'_>
{
	/// Render `+page.ts` for supplying page data to SvelteKit.
	pub(super) fn render_page_ts(&mut self) -> SquarkResult
	{
		log::info!(
			"rendering to: {GREY1}{}{GREY}/+page.ts",
			utils::display_rel(&self.dest_folder, &self.config.paths.root),
		);

		let file = File::create(self.dest_folder.join("+page.ts"))?;
		let mut f = BufWriter::new(file);

		f.write_all(b"import type { PageData } from 'squarkdown';\n\n")?;
		f.write_all(b"export default function load(): PageData {\n")?;
		f.write_all(b"\treturn {\n")?;

		let data = self.page.clone().serialise(self.config);
		let long = self.config.out.long_fields;

		writeln!(f, "\t\t{}: {:?},", data.filepath(long),    data.filepath)?;
		writeln!(f, "\t\t{}: {:?},", data.destination(long), data.destination)?;
		writeln!(f, "\t\t{}: {},",   data.flags(long),       data.flags)?;

		if let Some(title) = &data.title {
			writeln!(f, "\t\t{}: {:?},", data.title(long), title)?;
		}
		if let Some(description) = &data.description {
			writeln!(f, "\t\t{}: {:?},", data.description(long), description)?;
		}
		if let Some(heading) = &data.heading {
			writeln!(f, "\t\t{}: {:?},", data.heading(long), heading)?;
		}
		if let Some(caption) = &data.caption {
			writeln!(f, "\t\t{}: {:?},", data.caption(long), caption)?;
		}

		writeln!(f, "\t\ttags: {:?},", data.tags)?;

		if let Some(release_date) = data.release_date {
			writeln!(f, "\t\t{}: {:?},", data.release_date(long), release_date)?;
		}
		if let Some(last_updated) = data.last_updated {
			writeln!(f, "\t\t{}: {:?},", data.last_updated(long), last_updated)?;
		}

		writeln!(f, "\t\t{}: {:?},", data.other(long), data.other)?;

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
