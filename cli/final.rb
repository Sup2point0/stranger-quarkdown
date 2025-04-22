def export_json(data:)
  root = Pathname(__dir__).parent.parent
  
  route = root / ".squarkdown"
  route.mkpath()

  dest = route / "squarkup.json"
  File.write(dest, JSON.pretty_generate(data))
end
