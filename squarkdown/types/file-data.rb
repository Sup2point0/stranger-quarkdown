require "json"

require_relative "../config"


class FileData
  attr_reader
    :path,
    :live,

  RequiredFields = [
    :dest
  ]

  def initialize(source)
    @path = source.relative_path_from(Routes.root)
    @live = false
    @isIndex = false
    @isFeatured = false
    @isWoozy = false

    # For all fields, `nil` indicates default or skipped handling
    @title = nil
    @head = nil # title
    @capt = nil
    @desc = nil # capt
    @style = ["article"]
    @duality = nil
    @index = []
    @shard = nil # index
    @date = nil
    @date_display = nil
    @clean = []
  end

  def update(field, value, repo_config:)
    case field

    when "title"
      @title = value

    when "head"
      @head = value[2..]

    when "style"
      @style = value.downcase.split(" / ").unshift("article")

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
