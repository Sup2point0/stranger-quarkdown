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

		writeln!(f, "\t\tfilepath: {:?},", data.filepath)?;
		writeln!(f, "\t\tdestination: {:?},", data.destination)?;
		writeln!(f, "\t\tflags: {:?},", data.flags.into_vec())?;

		if let Some(title) = data.title {
			writeln!(f, "\t\ttitle: {:?},", title)?;
		}
		if let Some(description) = data.description {
			writeln!(f, "\t\tdescription: {:?},", description)?;
		}
		if let Some(heading) = data.heading {
			writeln!(f, "\t\theading: {:?},", heading)?;
		}
		if let Some(caption) = data.caption {
			writeln!(f, "\t\tcaption: {:?},", caption)?;
		}

		writeln!(f, "\t\ttags: {:?},", data.tags)?;

		if let Some(release_date) = data.release_date {
			writeln!(f, "\t\trelease_date: {:?},", release_date)?;
		}
		if let Some(last_updated) = data.last_updated {
			writeln!(f, "\t\tlast_updated: {:?},", last_updated)?;
		}

		writeln!(f, "\t\tother: {:?},", data.other)?;

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
