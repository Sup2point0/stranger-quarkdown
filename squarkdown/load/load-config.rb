module Load

require "pathname"
require "json"
require "json-schema"


$schema = nil


## :: *mut Routes -> RepoConfig
#
# Load the user's repo config.
# 
# Finds `.squarkdown/squarkup.json` in their project repo, validates it with the squarkup JSON schema, and fills in absent values with defaults.
# 
# Also extracts `paths / site` and updates `routes.site`.
def self.load_repo_config!(routes:)

  $schema = load_squarkup_schema(routes:)
  log success: "found Squarkdown's #{BLUE}squarkup-schema.json"

  json = load_user_repo_config(routes:)
  log success: "found your #{BLUE}squarkup.json"

  JSON::Validator.validate!($schema, json)
  log success: "validated your #{BLUE}squarkup.json #{CYAN}against the schema"

  defaults = load_repo_config_defaults(routes:)
  out = RepoConfigData.new(json:, routes:, defaults:)
  log success: "processed your config"

  routes.resolve_site(repo_config: out)
  log success: "found your project site: #{BLUE}#{routes.site.expand_path}"
  
  return out
end


private

## :: *Routes -> JSON
def self.load_squarkup_schema(routes:)

  route = routes.root / "squarkdown/resources/squarkup-schema.json"
  raise "Internal Error: failed to find #{BLUE}squarkup-schema.json" unless route.exist?
        
  content = File.read(route)
  raise "Internal Error: read no content from #{BLUE}squarkup-schema.json" if content.empty?

  out = JSON.parse(content)
  
  return out
end


## :: *Routes -> JSON
def self.load_user_repo_config(routes:)

  route = routes.repo / ".squarkdown/squarkup.json"
  raise "could not find #{BLUE}squarkup.json" unless route.exist?

  content = File.read(route)
  raise "read no content from #{BLUE}squarkup.json" if content.empty?
  
  out = JSON.parse(content)

  return out
end


## :: *Routes -> Hash String Any
def self.load_repo_config_defaults(routes:)

  $schema = load_squarkup_schema(routes:) if $schema.nil?
  raise "could not access `properties` field on #{BLUE}squarkup-schema.json" if $schema["properties"].nil?

  out = $schema["properties"].map do |key, data|
    [key, data["default"]]
  end.to_h

  return out
end


end
