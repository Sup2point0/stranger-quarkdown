require "json"
require "date"


class SiteData
  attr_accessor :meta, :pages, :index, :tagss

  def initialize
    @meta = {
      exported: Date.today.to_s,
      file_count: 0,
      page_count: 0,
    }
    @pages = Hash.new
    @index = Hash.new
    @tags = Hash.new
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
  end

  def update_index(index:, page:)
    if !@index.include?(index)
      @index[index] = {"route" => nil, "pages" => [page]}
    else
      @index[index]["pages"].push(page)
    end
  end

  def update_tags(tags:, page:)
    if !@tags.include?(tags)
      @tags[tags] = [page]
    else
      @tags[tags].push(page)
    end
  end

  def export_json
    data = {
      meta: @meta,
      index: @index,
      tags: @tags,
      pages: @pages.map {|page, data| [page, data.export_internal]}.to_h,
    }
    return JSON.pretty_generate(data)
  end
end
