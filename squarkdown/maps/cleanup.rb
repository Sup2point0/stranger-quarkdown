Cleanup =
{
  braces: ->(content) {
    content.gsub!(/(?<!\{)\{(?!\{)/, "&amp;lbrace;")
    content.gsub!(/(?<!\})\}(?!\})/, "&amp;rbrace;")
  },

  angles: ->(content) {
    content.gsub!(/</, "&amp;lt;")
    content.gsub!(/>/, "&amp;gt;")
  },
}
