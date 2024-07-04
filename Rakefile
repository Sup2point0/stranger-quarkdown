task default: :run


## Tasks
task :test do
  Dir.glob("./tests/**/*.rb").each { |file| require file }
end


## Scripts
task :run do
  ruby "squarkdown/squarkup.rb"
end

task :squarkup do
  ruby "squarkdown/squarkup.rb", "fonts"
end
