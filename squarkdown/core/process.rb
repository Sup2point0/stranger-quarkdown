require_relative "../types/file-data"


# Max lines parsed
PROCESSED_LINES = 20


def extract_data(lines:, data: nil, repo_config:, fill_defaults: true)
  if data.nil?
    data = FileData.new
  end

  data.slocs = lines.length
  data.chars = lines.join.length

  is_processing = false
  allow_arbitrary = false
  load = []
  head_line_idx = nil

  lines[0, PROCESSED_LINES].each_with_index do |line, i|
    if line.start_with?("-->")
      if !load.empty?
        data.update_fields(load.join(" "), repo_config:)
      end
      break
    end

    if data.head.nil? and line.start_with?("# ")
      data.head = line[2..-1].strip
      head_line_idx = i
    end

    if !is_processing
      if !line.upcase.include?("#SQUARK")
        next
      end

      is_processing = true

      begin
        data.update_flags(line)
      rescue FileData::Squarkless
        return
      end
    end

    if data.live
      if line.start_with?("|")
        data.update_fields(load.join(" "), repo_config:)
        load = [line.strip]
      elsif line.start_with?("---")
        allow_arbitrary = true
      else
        load.push(line.strip)
      end
    end 
  end

  # remove the line with head
  if !head_line_idx.nil?
    lines.delete_at(head_line_idx)
  end

  if data.live
    data.fill_fields(repo_config:) if fill_defaults
    return data
  else
    return
  end
end
