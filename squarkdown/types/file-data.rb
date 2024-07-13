require "json"

require_relative "../config"
require_relative "../utils/vars"


class ValidationError < StandardError
end


class FileData
  include Vars
  
  # sentinel for unset required fields
  Unset = Object.new

  attr_reader :path, :live, :isIndex, :isFeatured, :isWoozy, :title, :head, :capt, :desc, :style, :duality, :index, :shard, :date, :date_display, :clean,

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

    when "title"
      @title = value

    when "head"
      @head = value[2..]

    when "style"
      @style = value.downcase.split(" / ").unshift("article")

    end
  end

  def validate
    if self.vars_str.values.include?(Unset)
      raise ValidationError
    end
  end


  def to_json
    return (self
      .instance_variables
      .slice(:head, :capt, :desc, :index, :shard, :date)
      .to_json
    )
  end

  def to_yaml
    return (self
      .instance_variables
      .slice(:title, :head, :capt, :desc, :index, :shard, :date_displayed)
      .to_yaml
    )
  end
end
