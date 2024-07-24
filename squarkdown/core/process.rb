require_relative "../types/file-data"


# Max lines parsed
ProcessedLines = 20


def extract_data(lines:, data: nil, repo_config:, fill_defaults: true)
  if data.nil?
    data = FileData.new
  end

  processing = false
  idx = nil

  lines[0, ProcessedLines].each_with_index do |line, i|
    if data.head.nil?
      if line.start_with?("# ")
        data.head = line[2..-1].strip
        idx = i
      end
    end

    if !processing
      if line.upcase.include?("#SQUARK")
        processing = true
        begin
          data.update_flags(line)
        rescue FileData::Squarkless
          return
        end
      else
        next
      end
    end

    data.update_fields(line, repo_config:)
  end

  # remove the line with head
  if !idx.nil?
    lines.delete_at(idx)
  end

  if data.live
    data.fill_fields(repo_config:) if fill_defaults
    return data
  else
    return
  end
end
