task default: :squark


## == User Scripts ==

## Run Squarkdown on a project.
task :squark, [:fonts, :assets, :scss, :root] do |task, args|
  args.with_defaults({
    fonts: nil,
    assets: nil,
    scss: nil,
    root: nil,  # internal flag for squarking up Squarkdown itself instead of the parent project
  })
  ruby "squarkdown/__main__.rb", *args
end

## Setup Squarkdown for a project.
task :init do
  ruby "cli/init.rb", "--silent"
end

## Run unit tests.
task :test do
  ARGV.append("root")
  ARGV.append("silent")

  require_relative "tests/__init__"

  Dir.glob("./tests/**/test-*.rb").each { |file| require file }
end


## == Developer Scripts ==

## Update documentation for the latest version of the Squarkup schema.
task :schema do
  ARGV.append("root")

  require_relative ".github/scripts/schema-doc"
  version = doc_schema

  require_relative ".github/scripts/schema-sync"
  sync_schema version

  log done: true
end
