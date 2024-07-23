require "json"
require "date"


class SiteData
  attr_accessor :pages, :index, :shards

  def initialize
    @pages = Hash.new
    @index = []
    @shard = []
  end

  def add_page(page)
    @pages[page.path] = page
  end

  def add_index(index)
  end

  def add_shard(shard)
  end

  def export_json
    data = {
      exported: Date.today.to_s,
      index: @index,
      shard: @shard,
      pages: @pages.map {|page, data| [page, data.export_internal]}.to_h,
    }
    return JSON.pretty_generate(data)
  end
end
