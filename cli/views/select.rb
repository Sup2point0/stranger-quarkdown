require_relative "../../squarkdown/utils/ansi"

require_relative "out"


def select(
  text = nil,
  before: nil,
  after: nil,
  options: nil,
  multi: false
)
  $i = 0
  $t = options.length

  if multi
    options = options.map { |opt, desc| [opt, desc, false] }
  end

  selected = nil
  $cursor.invisible do
    out step: before || text
    selected = wait_select(options.to_a, multi:)
  end

  $t.times do
    print PREV, CLEAR
  end

  print PREV, CLEAR
  out success: after || text
  if multi
    if selected.empty?
      out GREY, "none"
    else
      out GREY, selected.join(", ")
    end
  else
    out GREY, selected[0]
  end

  return selected[0]
end


def wait_select(options, multi:)
  last_index = $i
  changed_selection = false

  render_options(options, multi:)

  loop do
    c = $reader.read_char

    case c
      when " "
        if multi
          options[$i][2] = !options[$i][2]
          changed_selection = true
        end
      when "\r"
        if multi
          return options.select { |opt| opt[2] }.map { |opt| opt[0] }
        else
          return options[$i]
        end
    end

    # only redraw if index changed
    if last_index != $i or changed_selection
      last_index = $i
      changed_selection = true

      $t.times do
        print PREV, CLEAR
      end
      render_options(options, multi:)
    end
  end
end


def render_options(options, multi:)
  options.each_with_index do |opt, idx|
    label, desc, state = opt

    if multi
      if idx == $i
        if state
          out CYAN, FILLED_DOT, " ", WHITE, label, GREY, " ", ITALIC, desc, ROMAN
        else
          out GREY, DOT, " ", WHITE, label, GREY, " ", ITALIC, desc, ROMAN
        end
      
      else
        if state
          out CYAN, FILLED_DOT, " ", WHITE, label
        else
          out GREY, DOT, " ", label
        end
      end
    
    else
      if idx == $i
        out CYAN, FILLED_DOT, " ", WHITE, label, GREY, " ", ITALIC, desc, ROMAN
      else
        out GREY, DOT, " ", label
      end
    end
  end
end
