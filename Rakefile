task default: :squark
VERSION = "2.1.4"

require_relative "squarkdown/utils/log"
log "running Squarkdown v#{VERSION}"


## Tasks
task :test do
  Dir.glob("./tests/**/*.rb").each { |file| require file }
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
