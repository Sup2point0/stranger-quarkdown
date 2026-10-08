/**
 * The metadata exported by Squarkdown for a Markdown page, returned from `load()` in `+page.ts`.
 * 
 * The generic type parameter describes what format the field names use. `long` means full, clear identifiers like `description` and `last_update_raw`. `short` means more compact identifiers like `desc` and `update_raw`.
 * 
 * Which format is used in `+page.ts` and `site-data.json` is configured by `out.shorter-fields` in your squarkup config.
 */
export type PageData<
	FieldLength extends "long" | "short"
		= "long" | "short"
> = (
	FieldLength extends "long" ?
	  LongFields
	: ShortFields
);


interface LongFields
{
	/** The unique stable identifier for the page. */
	shard: string;

	/** The location of the original `.md` file this page represents. */
	filepath: string;

	/** The folder to render this page's `+page.svx` and `+page.ts` to, relative to `out.folder`. */
	destination:  string;

	/** The flags provided in this page's charm squark, excluding `live!`. */
	flags: string[];

	title?: string;
	description?: string;
	heading?: string;
	caption?: string;

	tags: string[];

	release_date?: Date;
	release_date_raw?: string;
	last_update?: Date;
	last_update_raw?:  string;
}


interface ShortFields
{
	/** The unique stable identifier for the page. */
	shard: string;

	/** The location of the original `.md` file this page represents. */
	path: string;

	/** The folder to render this page's `+page.svx` and `+page.ts` to, relative to `out.folder`. */
	dest:  string;

	/** The flags provided in this page's charm squark, excluding `live!`. */
	flags: string[];

	title?: string;
	desc?: string;
	head?: string;
	capt?: string;

	tags: string[];

	date?: Date;
	date_raw?: string;
	update?: Date;
	update_raw?:  string;
}
