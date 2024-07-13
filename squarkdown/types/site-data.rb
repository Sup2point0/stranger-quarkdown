require "json"


class SiteData
  attr_accessor :pages, :index, :shards

  def initialize
    @pages = []
    @index = []
    @shards = []
  end

  def add_page(page)
  end

  def add_index(index)
  end

  def add_shard(shard)
  end

  def save
    route = Routes.site / "src/site-data.js"
    existing = File.read(route)

    pattern = /(?<=siteData = ){.*\n};\n/m
    load = existing.match(pattern)
    data = JSON.parse(load)

    data["pages"] = @pages
    # TODO ...

    content = existing.sub(pattern, dump)
    File.write(route, content)
  end
end
