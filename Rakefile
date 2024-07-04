task default: :squark


## Tasks
task :test do
  Dir.glob("./tests/**/*.rb").each { |file| require file }
end


## Scripts
task :squark do
  ruby "squarkdown/squarkup.rb"
end

task :squarkup do
  ruby "squarkdown/squarkup.rb", "fonts", "assets", "scss"
end
