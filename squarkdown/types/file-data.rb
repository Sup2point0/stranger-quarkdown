# FileData
# Represents all the data of a file processed and exported by Squarkdown
# 
# The class exports to 2 kinds of representations: INTERNAL and EXTERNAL
# 
# INTERNAL
# - for JSON
# - stores a lot more data
# - exports as symbols
# 
# EXTERNAL
# - for YAML
# - stored as frontmatter in .svx files
# - mostly properties relevant to page rendering
#   (used through $page.data in SvelteKit)
# - exports as strings

require "json"
require "yaml"

require_relative "../config"
require_relative "../utils/vars"
require_relative "../maps/seasons"


class FileData
  include Vars

  class Squarkless < StandardError
  end
  
  class ValidationError < StandardError
  end
  
  # sentinel for unset required fields
  Unset = Object.new

  attr_accessor :live, :slocs, :chars, :head
  attr_reader :path, :last_deploy, :isIndex, :flags, :dest, :title, :capt, :desc, :style, :duality, :index, :tags, :date, :date_display, :update, :update_display, :clean

  Fields = [
    :dest, :title, :desc, :head, :capt,
    :style, :duality,
    :index, :tags,
    :date, :update,
    :clean,
    :data,
  ]


  def initialize(source = nil, repo_config: nil)
    ## Meta
    @path = source && source.relative_path_from(Routes.repo).to_s
    @last_deploy = source && source.mtime
    @slocs = 0
    @chars = 0

    ## Flags
    @live = false
    @isIndex = false
    @flags = []

    ## Fields
    # For all fields, `nil` indicates default or skipped handling
    @dest = Unset
    @title = nil  # head
    @desc = nil  # capt
    @head = nil
    @capt = nil

    if repo_config and repo_config["styles / base-style"]
      @style = [repo_config["styles / base-style"]]
    else
      @style = []
    end

    @duality = nil
    @index = []
    @tags = nil  # index
    @date = nil
    @date_display = nil
    @update = nil
    @update_display = nil
    @clean = []
  end


  def _split_(value)
    value.downcase.split(" / ")
  end


  ##
  # Given a line of text, detect the flags present and update the file data.
  def update_flags(text)
    if text.include?("dead!") then raise Squarkless end
    if text.include?("live!") then @live = true end
    if text.include?("index!") then @isIndex = true end

    _, _, data = text.partition "#SQUARK"
    flags = data.split
    @flags = flags.map { |flag| flag.gsub("!", "").strip }
  end


  def update_fields(text, repo_config:)
    _, _, value = text.partition("=")
    value.strip!

    Fields.each do |field|
      # NOTE: line start, `|field`, ` field` are valid
      if text.match("(?:[ |]|^)#{field} ?=")
        if _parse_(field:, value:, repo_config:)
          break
        end
      end
    end

    return self
  end


  def _parse_(field:, value:, repo_config:)
    case field

    when :dest then @dest = value
    when :title then @title = value
    when :desc then @desc = value
    when :head then @head = value
    when :capt then @capt = value

    when :duality then @duality = value.downcase
    when :index then @index = _split_(value)
    when :clean then @clean = _split_(value)

    when :style
      styles = _split_(value)
      base = repo_config["styles / base-style"]

      if styles.delete("#auto")
        if base
          styles.unshift(base)
        end
      elsif base
        if !styles.include?(base)
          styles.unshift(base)
        end
      end

      @style = styles

    when :tags
      @tags = _split_(value)
      if @tags.delete("#index")
        @tags.unshift(*@index)
      end

    when :date
      @date_display = value
      @date = _parse_date_(value)
    
    when :update
      @update_display = value
      @update = _parse_date_(value)
    
    else
      return

    end

    return self
  end


  def _parse_date_(value)
    begin
      return Date.strptime(value, "%Y %B %d")
    rescue Date::Error
    end

    begin
      return Date.strptime(value, "%Y %B")
    rescue Date::Error
    end

    begin
      @date = Date.strptime(value, "%Y")
    rescue Date::Error
    end

    # 20XX season
    begin
      year, season = value.downcase.split(" ")
      dec = Seasons[season.downcase] + 1
      return Date.civil(year.to_i, dec+1, -1)
    rescue Date::Error
    end
  end


  def fill_fields(repo_config:)
    Fields.each do |field|
      if self.vars_sym[field].nil?
        _parse_default_(field:, repo_config:)
      end
    end

    if self.vars_str.values.include?(Unset)
      raise ValidationError
    end
  end

  def _parse_default_(field:, repo_config:)
    case field
      when :title   then @title = @head
      when :desc    then @desc = @capt
      when :duality then @duality = "light"
      when :tags    then @tags = @index
      when :update  then @update = @date
    end
  end


  def export_internal
    return self.vars_sym.slice(
      :path, :last_deploy, :slocs, :chars, :isIndex, :flags,
      :dest, :title, :head, :capt, :desc, :index, :tags, :date, :date_display, :update, :update_display
    )
  end


  def export_external
    return self.vars_str.slice(
      "path", "last_deploy", "flags",
      "title", "head", "capt", "desc", "index", "tags", "date_display", "update_display"
    )
  end


  def to_yaml
    return self.export_external.to_yaml + "---\n\n"
  end
end
