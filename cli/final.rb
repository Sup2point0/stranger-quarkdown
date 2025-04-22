def export_json(data:)
  text = JSON.pretty_generate(data)
  text.gsub!(/"\/\/ UNSET"/, "// UNSET")

  root = Pathname(__dir__).parent.parent
  route = root / ".squarkdown"
  route.mkpath()

  dest = route / "squarkup.json"
  File.write(dest, text)
end
