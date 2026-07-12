$started_logging = false


## :: String -> IO ()
#
# Print a prettified message from Squarkdown.
def log(
  text = nil,
  success: nil,
  error: nil,
  hint: nil,
  done: false,
  **kwargs
)
  return if (SILENT and error.nil?)
  
  puts "#{GREY}#{
    if !$started_logging then ">>> #{PINK}Squark#{GREY} / "
    elsif           done then ">>> #{PINK}Squark#{GREY} / "
    elsif        success then "           #{CYAN}✓ "
    elsif          error then "           #{RED}⨯ "
    elsif           hint then "           #{WHITE}= "
    else                      "           / "
    end
  }#{
    if  success then "#{CYAN}#{success}"
    elsif error then "#{RED}#{error}"
    elsif  hint then "hint: #{GREEN}#{hint}"
    elsif  done then "#{CYAN}done!"
    else             "#{YELLOW}#{text}"
    end
  }#{WHITE}"

  $started_logging = true
end


## :: String -> IO ()
#
# Print a message from Squarkdown counting a number of processed files.
# 
# If `sparse`, only print a message every 10 files.
def log_count(n, total:, file:, sparse: false)
  return if SILENT

  log "#{n} #{GREY}of #{total}: #{WHITE}#{file.parent.basename}#{GREY}/#{BLUE}#{file.basename}"
end
