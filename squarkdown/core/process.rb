# Max lines parsed
ProcessedLines = 20

Fields = {
  "title": nil,
  "dest": nil,
  "style": -> [
    lambda do |val|
      val.downcase!
      val.sub!(Defaults["pattern"], Defaults["repl"]["style"])
      return val.split(" / ")
    end
  ],
  "duality": -> { |val| val.downcase },
  "index": -> { |val| val.downcase.split(" / ") },

  "shard": [
    lambda do |val, data, repo_config|
      val.sub!("#index", data["index"].join(" / ")
      return val.downcase.split
    end
  ],
}

Defaults = {
  "pattern": /#(auto|default)/,
  "repl": {
    "style": "essence"
  }
}


def extract_data(lines)
  data = {"live": true}

  lines[:ProcessedLines].each do |line|
    if !data["live"]
      if line.include?("#SQUARK live!")
        data["live"] = true
      else
        next
      end
    end

    Fields.each do |field, handler|
      if line.include?(field)
        _, _, value = line.partition("=")
        value.strip!

        if handler.is_a?(Array)
          handler.each do |handle|
            data[field] = handle(value, data:, repo_config:)
          end
        else
          data[field] = handler ? handler(value) : value
        end

      end
    end
  end

  return data
end
