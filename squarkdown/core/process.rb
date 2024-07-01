require_relative "fields"


# Max lines parsed
ProcessedLines = 20


def extract_data(lines, repo_config:)
  data = {"live": true}

  lines[0, ProcessedLines].each do |line|
    if !data["live"]
      if line.include?("#SQUARK live!")
        data["live"] = true
      else
        next
      end
    end

    Fields.each do |field, props|
      if line.include?(field)
        _, _, value = line.partition("=")
        value.strip!

        if handler = props["handle-all"]
          data[field] = handler.call(value, data:, repo_config:)
        elsif handler = props["handle-val"]
          data[field] = handler.call(value)
        else
          data[field] = value
        end

      end
    end
  end

  return data
end
