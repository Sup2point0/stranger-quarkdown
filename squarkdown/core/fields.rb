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
    "handle-val": ->(val) {
      val.downcase!
      val.sub!(Inject["pattern"], Inject["style"]["default"])
      return val.split(" / ")
    }
  },
  "duality": {
    "required": false,
    "default": "light",
    "handle-val": ->(val) { val.downcase }
  },
  "index": {
    "required": false,
    "default": "",
    "handle-val": ->(val) { val.downcase.split(" / ") }
  },
  "shard": {
    "required": false,
    "default": "",
    "handle-all": ->(val, data:, repo_config:) {
      val.sub!("#index", data["index"].join(" / "))
      return val.downcase.split(" / ")
    }
  }
}

Inject = {
  "pattern": /#(auto|default)/,
  "repl": {
    "style": "essence"
  }
}
