require "json"
require "date"


## All the metadata Squarkdown collected while processing files.
class SiteData
  attr_accessor(
    ## :: Hash Any
    :meta,
    
    ## :: Hash FileData
    :pages,
    
    ## :: Hash { "route": FileData, "pages": [String] }
    :index,
    
    ## :: Hash [String]
    :tags
  )


  def initialize()
    @meta = {
      exported: Date.today.to_s,
      file_count: 0,
      page_count: 0,
    }
    @pages = {}
    @index = {}
    @tags = {}
  end


  ## :: *FileData -> ()
  def add_page(page)
    @pages[page.path] = page
  end


  ## :: String -> *FileData -> ()
  def create_index(index:, page:)
    if !@index.include?(index)
      @index[index] = {"route" => page, "pages" => []}
    else
      @index[index]["route"] = page
    end
  end


  ## :: String -> *FileData -> ()
  def update_index(index:, page:)
    if !@index.include?(index)
      @index[index] = {"route" => nil, "pages" => [page]}
    else
      @index[index]["pages"].push(page)
    end
  end


  ## :: String -> *FileData -> ()
  def update_tags(tag:, page:)
    if !@tags.include?(tag)
      @tags[tag] = [page]
    else
      @tags[tag].push(page)
    end
  end


  ## :: -> String
  def export_json()
    data = {
      meta: @meta,
      index: @index,
      tags: @tags,
      pages: @pages.map {|page, data| [page, data.export_internal]}.to_h,
    }
    return JSON.pretty_generate(data)
  end

end
