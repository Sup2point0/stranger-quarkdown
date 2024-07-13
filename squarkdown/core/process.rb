require_relative "../types/file-data"
require_relative "../maps/fields"
require_relative "../maps/flags"


# Max lines parsed
ProcessedLines = 20


def extract_data(lines:, data: nil, repo_config:, fill_defaults: true)
  if data.nil?
    data = FileData.new
  end

  lines[0, ProcessedLines].each do |line|
    if data.head.nil?
      if line.start_with?("# ")
        data.head = line[2..]
      end
    end

    if !data.live
      if line.include?("#SQUARK live!")
        data.live = true
      elsif line.include?("#SQUARK dead!")
        return
      else
        next
      end
    end

    data.update(line, repo_config:)
  end

  if data.live
    data.fill()
    return data
  else
    return
  end
end


def handle(value, props:, data:, repo_config:)
  if handler = props["handle-all"]
    return handler.call(value, data:, repo_config:)
  elsif handler = props["handle-val"]
    return handler.call(value)
  else
    return value
  end
end
