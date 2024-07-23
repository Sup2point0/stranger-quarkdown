require_relative "../types/file-data"


# Max lines parsed
ProcessedLines = 20


def extract_data(lines:, data: nil, repo_config:, fill_defaults: true)
  if data.nil?
    data = FileData.new
  end

  idx = nil

  lines[0, ProcessedLines].each_with_index do |line, i|
    if data.head.nil?
      if line.start_with?("# ")
        data.head = line[2..-1].strip
        idx = i
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

  # remove the line with head
  if !idx.nil?
    lines.delete_at(idx)
  end

  if data.live
    if fill_defaults
      data.fill(repo_config:)
    end
    return data
  else
    return
  end
end
