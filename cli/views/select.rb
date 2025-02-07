require_relative "../../squarkdown/utils/ansi"

require_relative "out"


def select(options)
  i = 0
  t = options.length
  
  loop do
    options.each_with_index do |opt, idx|
      label, desc = opt

      if idx == i
        out WHITE, FILLED_DOT, " ", label, GREY, " ", desc
      else
        out GREY, DOT, " ", label
      end
    end

    c = $reader.read_char
    
    case c
      when UP
        idx -= 1
        if idx < 0
          idx = t - 1
        end

      when DOWN
        idx += 1
        if idx > t - 1
          idx = 0
        end
        
    end

    t.times do
      print PREV, CLEAR
    end
  end
end
