task default: :run


task :run do
  ruby "squarkdown/squarkup.rb"
end


task :test do
  Dir.glob("./tests/**/*.rb").each { |file| require file }
end
