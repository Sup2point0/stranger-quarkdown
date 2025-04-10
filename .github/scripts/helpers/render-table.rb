def render_table(field:, props:)
  return "| " + render_props(field:, props:).join(" | ") + " |"
end


def render_props(field:, props:)
  return [
    "`#{field}`",
    render_type(props),
    render_values(props),
    if props["default"] then "`#{props["default"]}`" else nil end,
    props["description"].gsub(/\n/, "<br>"),
  ]
end


def render_type(props)
  type = props["type"]

  return (
    if type == "array"
      if props["enum"]
        "`option[]`"
      else
        "`" + (props["items"]["type"] || "string") + "[]`"
      end
    elsif type.is_a?(String)
      "`#{type}`"

    elsif type.is_a?(Array) and type[0].is_a?(String)
      type.map {|option| "`#{option}`"}.join(" ") 
    elsif props["enum"]
      "`option`"

    end
  )
end


def render_values(props)
  values = props["enum"] || (props["items"] && props["items"]["enum"])
  if values
    return values.map {|option| "`#{option}`"}.join(" ")
  else
    return ""
  end
end
