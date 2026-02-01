task default: :squark


## Tasks

task :test do
  ARGV.append("root")
  Dir.glob("./tests/**/*.rb").each { |file| require file }
end

task :schema do
  ARGV.append("root")
  require_relative "squarkdown/config.rb"

  require_relative ".github/scripts/schema-doc"
  version = doc_schema

  require_relative ".github/scripts/schema-sync.rb"
  sync_schema version

  log done: true
end


## Scripts

task :squark, [:fonts, :assets, :scss, :root] do |task, args|
  args.with_defaults({
    fonts: nil,
    assets: nil,
    scss: nil,
    root: nil,  # internal flag for squarking up Squarkdown itself instead of the parent project
  })
  ruby "squarkdown/squarkup.rb", *args
end

task :init do
  ruby "cli/init.rb", "--silent"
end
