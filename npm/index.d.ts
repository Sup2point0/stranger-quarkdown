/**
 * The metadata exported by Squarkdown for a Markdown page, returned from `load()` in `+page.ts`.
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
	
	/** Other arbitrary user-provided fields not intrinsic to Squarkdown. */
	other: Record<string, string[]>;
}


interface ShortFields
{
	/** The unique stable identifier for the page. */
	shard: string;

	/** The location of the original `.md` file this page represents. */
	filepath: string;

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
	
	/** Other arbitrary user-provided fields not intrinsic to Squarkdown. */
	other: Record<string, string[]>;
}
