# Max lines parsed
ProcessedLines = 20

Fields = {
  "title": {
    "required": true
  },
  "dest": {
    "required": true
  },
  "style": {
    "required": false,
    "default": "essence",
    "handle-val": -> { |val|
      val.downcase!
      val.sub!(Inject["pattern"], Fields["style"]["default"])
      return val.split(" / ")
    }
  },
  "duality": {
    "required": false,
    "default": "light",
    "handle-val": -> { |val| val.downcase }
  },
  "index": {
    "required": false,
    "default": "",
    "handle-val": -> { |val| val.downcase.split(" / ") }
  }
  "shard": {
    "required": false,
    "default": "",
    "handle-all": -> { |val, data, repo_config|
      val.sub!("#index", data["index"].join(" / ")
      return val.downcase.split
    }
  ],
}

Inject = {
  "pattern": /#(auto|default)/,
  "repl": {
    "style": "essence"
  }
}


def extract_data(lines, data:, repo_config:)
  data = {"live": true}

  lines[:ProcessedLines].each do |line|
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

        handler = props["handle-all"]
        if handler
          data[field] = handler(value, data:, repo_config:)
        end
        
        handler = props["handle-val"]
        if handler
          data[field] = handler(value)
        else
          data[field] = value
        end

      end
    end
  end

  return data
end
