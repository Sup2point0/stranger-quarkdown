$started_logging = false


## Print a prettified message from Squarkdown.
def log(
  text = nil,
  success: nil,
  error: nil,
  done: false,
  **kwargs
)
  return if (SILENT and error.nil?)
  
  puts "#{GREY}#{
    if !$started_logging then ">>> #{PINK}Squark#{GREY} / "
    elsif           done then ">>> #{PINK}Squark#{GREY} / "
    elsif        success then "           #{CYAN}✓ "
    elsif          error then "           #{RED}⨯ "
    else                      "           / "
    end
  }#{
    if  success then "#{CYAN}#{success}"
    elsif error then "#{RED}#{error}"
    elsif  done then "#{CYAN}done!"
    else             "#{YELLOW}#{text}"
    end
  }#{WHITE}"

  $started_logging = true
end
