require_relative "fields"


# Max lines parsed
ProcessedLines = 20


def extract_data(lines, repo_config:, fill_defaults: true)
  data = {live: false}

  lines[0, ProcessedLines].each do |line|
    if !data[:live]
      if line.include?("#SQUARK live!")
        data[:live] = true
      else
        next
      end
    end

    Fields.each do |field, props|
      if line.include?(String(field) + " = ")
        _, _, value = line.partition("=")
        value.strip!

        data[field] = handle(value, props:, data:, repo_config:)
      end
    end
  end

  # fill in unspecified defaults
  if fill_defaults
    Fields.each do |field, props|
      if !data.include?(field)
        if props["required"]
          raise "Required field not set"
        end

        value = props["default"]
        data[field] = handle(value, props:, data:, repo_config:)
      end
    end
  end

  return data
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
