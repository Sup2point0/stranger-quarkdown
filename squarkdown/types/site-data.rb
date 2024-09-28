require "json"
require "date"


class SiteData
  attr_accessor :pages, :index, :shards

  def initialize
    @pages = Hash.new
    @index = Hash.new
    @shard = Hash.new
  end

  def add_page(page)
    @pages[page.path] = page
  end

  def create_index(index:, page:)
    if !@index.include?(index)
      @index[index] = {"route" => page, "pages" => []}
    else
      @index[index]["route"] = page
    end

  def update_index(index:, page:)
    if !@index.include?(index)
      @index[index] = {"route" => nil, "pages" => [page]}
    else
      @index[index]["pages"].push(page)
    end
  end

  def update_shard(shard:, page:)
    if !@shard.include?(shard)
      @shard[shard] = [page]
    else
      @shard[shard].push(page)
    end
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
