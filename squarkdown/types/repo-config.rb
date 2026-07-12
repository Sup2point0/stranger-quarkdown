require_relative "../core/find"

require_relative "../utils/ansi"
require_relative "../utils/log"


SCHEMA_MAP = {
  "repo"                => [:@core,   :@repo],
  "paths / site"         => [:@paths,  :@site],
  "paths / sources"      => [:@paths,  :@sources],
  "paths / exclude"      => [:@paths,  :@exclude],
  "paths / dest"         => [:@paths,  :@dest],
  "out / file-name"      => [:@out,    :@file_name],
  "out / site-data"      => [:@out,    :@site_data],
  "opts / on-error"      => [:@opts,   :@on_error],
  "opts / on-no-dir"     => [:@opts,   :@on_no_dir],
  "bases / path"         => [:@bases,  :@path],
  "bases / page.svelte"  => [:@bases,  :@page_svelte],
  "bases / index.svelte" => [:@bases,  :@index_svelte],
  "bases / page.js"      => [:@bases,  :@page_js],
  "bases / index-view"   => [:@bases,  :@index_view],
  "styles / path"        => [:@styles, :@path],
  "styles / page-styles" => [:@styles, :@page_styles],
  "styles / base-style"  => [:@styles, :@base_style],
  "assets / path"        => [:@assets, :@path],
  "assets / site-assets" => [:@assets, :@site_assets],
  "assets / extensions"  => [:@assets, :@extensions],
  "fonts / queries"      => [:@fonts,  :@queries],
}


##
# The end user's configuration for Squarkdown, processed from `squarkup.json`.
class RepoConfigData

  attr_reader(
    :core,
    :paths,
    :out,
    :opts,
    :bases,
    :styles,
    :assets,
    :fonts
  )

  ##
  # Create a new `RepoConfig` with data extracted from `json:`.
  # 
  # Paths are resolved to `Pathname`s relative to `routes:`.
  # 
  # If `fill_defaults`, also load the Squarkup schema to find default values for missing fields. This requires `Routes` to be initialised.
  def initialize(json:, routes:, defaults: nil)

    @core   = Core.new 
    @paths  = Paths.new
    @out    = Out.new
    @opts   = Opts.new
    @bases  = Bases.new
    @styles = Styles.new
    @assets = Assets.new
    @fonts  = Fonts.new

    json.each do |key, value|
      next if key.start_with? "$"

      category_symbol, option_symbol = SCHEMA_MAP[key]

      category = instance_variable_get(category_symbol)

      category.instance_variable_set(
        option_symbol,
        (
            !value.nil? ? value
          : !defaults.nil? ? defaults[key]
          : nil
        )
      )
    end
  end

  class Core
    attr_reader(
      :repo
    )
  end

  class Paths
    attr_reader(
      :site,
      :sources,
      :exclude,
      :dest
    )
  end

  class Out
    attr_reader(
      :file_name,
      :site_data
    )
  end

  class Opts
    attr_reader(
      :on_error,
      :on_no_dir
    )
  end

  class Bases
    attr_reader(
      :path,
      :page_svelte,
      :page_js,
      :index_svelte
    )
  end

  class Styles
    attr_reader(
      :path,
      :page_styles,
      :base_style,
    )
  end

  class Assets
    attr_reader(
      :path,
      :site_assets,
      :extensions
    )
  end

  class Fonts
    attr_reader(
      :queries
    )
  end
end
