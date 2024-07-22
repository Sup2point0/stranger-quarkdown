def render_table(field:, props:)
 return "| " + render_props(field:, props:).join(" | ") + " |"
end


def render_props(field:, props:)
 return [
   "`#{field}`",
   "#{render_type(props)}",
   props["description"],
 ]
end


def render_type(props)
 type = props["type"]

 return case
   when type == "array"
     type = props["items"]
     "`#{type}`[]"
   when type.is_a?(String)
     "`#{type}`"
   when type.is_a?(Array) and type[0].is_a?(String)
     type.map {|option| "`#{option}`"}.join(" ")
 end
end
