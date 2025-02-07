require_relative "../../squarkdown/utils/ansi"

require_relative "out"


def select(text, options, multi: false)
  $i = 0
  $t = options.length

  selected = nil
  $cursor.invisible do
    out step: text
    selected = wait_select(options.to_a, multi:)
  end

  $t.times do
    print PREV, CLEAR
  end

  print PREV
  out success: text
  out GREY, selected[0]

  return selected[0]
end


def wait_select(options, multi:)
  i = $i
  render_options(options, multi:)

  loop do
    c = $reader.read_char

    case c
      when " "
        if multi
          options[$i][0] = !options[$i][0]
        end
      when "\r"
        if multi
          return options.select { |opt| opt[0] }.map { |opt| opt[1] }
        else
          return options[$i]
        end
    end

    # only redraw if index changed
    if $i != i
      i = $i

      $t.times do
        print PREV, CLEAR
      end
      render_options(options, multi:)
    end
  end
end


def render_options(options, multi:)
  options.each_with_index do |opt, idx|
    label, desc = opt

    if multi
      if opt[1][0]
        out CYAN, FILLED_DOT, " ", WHITE, label, GREY, " ", desc[1]
      else
        out GREY, DOT, " ", label
      end
    
    else
      if idx == $i
        out CYAN, FILLED_DOT, " ", WHITE, label, GREY, " ", desc
      else
        out GREY, DOT, " ", label
      end
    end
  end
end
