module Squarkdown

require_relative "../types/file-data"


# Max lines parsed
PROCESSED_LINES = 20


## :: [String] -> *mut FileData -> *RepoConfig -> Bool -> Option FileData
def self.extract_file_data!(lines:, file_data: nil, repo_config:, fill_defaults: true)
  if file_data.nil?
    file_data = FileData.new
  end

  file_data.slocs = lines.length
  file_data.chars = lines.join.length

  is_processing = false
  allow_arbitrary = false
  
  load = []
  head_line_idx = nil

  lines[0, PROCESSED_LINES].each_with_index do |line, i|
    if line.start_with?("-->")
      if !load.empty?
        file_data.update_fields(load.join(" "), repo_config:, allow_arbitrary:)
      end
      break
    end

    if file_data.head.nil? and line.start_with?("# ")
      file_data.head = line[2..-1].strip
      head_line_idx = i
    end

    if !is_processing
      if !line.upcase.include?("#SQUARK")
        next
      end

      is_processing = true

      begin
        file_data.update_flags(line)
      rescue FileData::Squarkless
        return
      end
    end

    if file_data.live
      if line.start_with?("|")
        file_data.update_fields(load.join(" "), repo_config:, allow_arbitrary:)
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

  if file_data.live
    file_data.fill_fields(repo_config:) if fill_defaults
    return file_data
  else
    return
  end
end


end
