require "json"
require "yaml"

require_relative "../config"
require_relative "../utils/vars"


class FileData
  include Vars
  
  class ValidationError < StandardError
  end
  
  # sentinel for unset required fields
  Unset = Object.new


  attr_reader :path, :live, :isIndex, :isFeatured, :isWoozy, :title, :head, :capt, :desc, :style, :duality, :index, :shard, :date, :date_display, :clean

  Fields = [:dest, :title, :capt, :desc, :style, :duality, :index, :shard, :date, :clean]


  def initialize(source = nil)
    @path = source and source.relative_path_from(Routes.root)
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


  def update(text, repo_config:)
    _, _, value = line.partition("=")
    value.strip!

    Fields.each do |field|
      _parse_(field:, value:, repo_config:)
    end
  end
  

  def _parse_(field:, value:, repo_config:)
    case field

    when :dest then @dest = value
    when :title then @title = value
    when :capt then @capt = value
    when :desc then @desc = value

    when :duality then @duality = value.downcase
    when :index then @index = _split_(value)
    when :shard then @shard = _split_(value)
    when :clean then @clean = _split_(value)

    when :style
      styles = _split_(value)

      if !styles.include?("article")
        styles.unshift "article"
      end

      @style = styles

    when :date
      begin
        @date = Date.strptime(value, "%Y %B %d")
      rescue Date::Error
      end

    end
  end

  def _split_(value)
    value.downcase.split(" / ")
  end


  def fill
    Files.each do |field|
    end

    if self.vars_str.values.include?(Unset)
      raise ValidationError
    end
  end


  def export_internal
    return self.vars_sym.slice(:head, :capt, :desc, :index, :shard, :date)
  end

  def export_external
    return self.vars_sym.slice(:title, :head, :capt, :desc, :index, :shard, :date_displayed)
  end


  def to_json
    return self.export_internal.to_json
  end

  def to_pretty_json
    return JSON.pretty_generate(self.export_internal)
  end

  def to_yaml
    return self.export_external.to_yaml
  end
end
