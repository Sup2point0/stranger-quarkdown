task default: :squark


## Tasks

task :test do
  Dir.glob("./tests/**/*.rb").each { |file| require file }
end

task :schema do
  require_relative ".github/scripts/schema-doc.rb"
  ruby ".github/scripts/schema-sync.rb", get_version
  log done: true
end


## Scripts

task :squark, :fonts, :assets, :scss do |task, args|
  args.with_defaults({
    fonts: nil,
    assets: nil,
    scss: nil
  })
  ruby "squarkdown/squarkup.rb", *args
end

task :init do
  ruby "cli/init.rb"
end
