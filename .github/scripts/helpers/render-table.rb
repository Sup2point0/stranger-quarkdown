def render_table(field:, props:)
  return "| " + render_props(field:, props:).join(" | ") + " |"
end


def render_props(field:, props:)
  return [
    "`#{field}`",
    "#{render_type(props)}",
    props["description"].gsub(/\n/, ""),
  ]
end


def render_type(props)
  type = props["type"]

  return (
    if type == "array"
      if props["enum"]
        "option[]"
      else
        "`" + (props["items"]["type"] || "string") + "[]`"
      end
    elsif type.is_a?(String)
      "`#{type}`"

    elsif type.is_a?(Array) and type[0].is_a?(String)
      type.map {|option| "`#{option}`"}.join(" ") 
    elsif props["enum"]
      "option"

    end
  )
end
