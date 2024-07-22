require "json"


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

  def to_json
    data = {
      pages: @pages,
      index: @index,
      shard: @shard,
    }
    return JSON.pretty_generate(data)
  end
end
