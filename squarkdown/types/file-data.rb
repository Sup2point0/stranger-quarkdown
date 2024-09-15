require "json"
require "yaml"

require_relative "../config"
require_relative "../utils/vars"


class FileData
  include Vars

  class Squarkless < StandardError
  end
  
  class ValidationError < StandardError
  end
  
  # sentinel for unset required fields
  Unset = Object.new

  attr_accessor :live, :slocs, :chars, :head
  attr_reader :path, :isIndex, :isFeatured, :isWoozy, :dest, :title, :capt, :desc, :style, :duality, :index, :shard, :date, :date_display, :clean

  Fields = [:dest, :title, :capt, :desc, :style, :duality, :index, :shard, :date, :clean]


  def initialize(source = nil)
    @path = source && source.relative_path_from(Routes.repo).to_s
    @slocs = 0
    @chars = 0
    @head = nil
    @live = false
    @isIndex = false
    @isFeatured = false
    @isWoozy = false

    # For all fields, `nil` indicates default or skipped handling
    @dest = Unset
    @title = nil
    @capt = nil
    @desc = nil  # capt
    @style = ["article"]
    @duality = nil
    @index = []
    @shard = nil  # index
    @date = nil
    @date_display = nil
    @clean = []
  end

  def _split_(value)
    value.downcase.split(" / ")
  end


  def update_flags(text)
    if text.include?("dead!") then raise Squarkless end
    if text.include?("live!") then @live = true end
    if text.include?("index!") then @isIndex = true end
    if text.include?("feat") then @isFeatured = true end
    if text.include?("woozy") then @isWoozy = true end
  end


  def update_fields(text, repo_config:)
    _, _, value = text.partition("=")
    value.strip!

    Fields.each do |field|
      if text.match("#{field} ?=")
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
    when :capt then @capt = value
    when :desc then @desc = value

    when :duality then @duality = value.downcase
    when :index then @index = _split_(value)
    when :clean then @clean = _split_(value)

    when :style
      styles = _split_(value)

      if styles.delete("#auto")
        styles.unshift("article")
      elsif !styles.include?("article")
        styles.unshift "article"
      end

      @style = styles

    when :shard
      @shard = _split_(value)
      if @shard.delete("#index")
        @shard.unshift(*@index)
      end

    when :date
      @date_display = value
      
      begin
        @date = Date.strptime(value, "%Y %B %d")
      rescue Date::Error
        begin
          @date = Date.strptime(value, "%Y %B")
        rescue Date::Error
          begin
            @date = Date.strptime(value, "%Y")
          rescue Date::Error
            return
          end
        end
      end

    else
      return

    end

    return self
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
    
    when :title then @title = @head
    when :desc then @desc = @capt
    when :duality then @duality = "light"
    when :shard then @shard = @index

    end
  end


  def export_internal
    return self.vars_sym.slice(
      :path, :slocs, :chars, :isIndex, :isFeatured, :isWoozy,
      :dest, :title, :head, :capt, :desc, :index, :shard, :date, :date_display
    )
  end

  def export_external
    return self.vars_str.slice(
      "path", "isFeatured",
      "title", "head", "capt", "desc", "index", "shard", "date_display"
    )
  end


  def to_yaml
    return self.export_external.to_yaml + "---\n\n"
  end
end
