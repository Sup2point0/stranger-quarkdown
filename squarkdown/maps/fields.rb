## File Fields
# Defines the core flags for a file undergoing squarkup.

require "date"


Fields = {
  dest: {
    "required" => true,
  },
  title: {
    "required" => false,
    "default": nil,
  },
  capt: {
    "required" => false,
    "default": nil,
  },
  desc: {
    "required" => false,
    "default" => ""
  },
  style: {
    "required" => false,
    "default" => "default",
    "handle-val" => ->(val) {
      val.downcase.sub(
        Inject[:pattern],
        Inject[:repl]["style"]
      ).split(" / ")
    }
  },
  duality: {
    "required" => false,
    "default" => "light",
    "handle-val" => ->(val) { val.downcase }
  },
  index: {
    "required" => false,
    "default" => "",
    "handle-val" => ->(val) { val.downcase.split(" / ") }
  },
  shard: {
    "required" => false,
    "default" => "#index",
    "handle-all" => ->(val, data:, repo_config:) {
      val.downcase.sub(
        "#index",
        data[:index].join(" / ")
      ).split(" / ")
    }
  },
  date: {
    "required" => false,
    "default" => nil,
    "handle-val" => ->(val) {
      Date.strptime(val, "%Y %B %d") if !val.nil?
    }
  },
  clean: {
    "required" => false,
    "default" => "",
    "handle-val" => ->(val) { val.downcase.split(" / ") }
  }
}

Inject = {
  pattern: /#(auto|default)/,
  repl: {
    "style" => Fields[:style]["default"]
  },
}
