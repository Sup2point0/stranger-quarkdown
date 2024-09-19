Cleanup = {
  braces: ->(content) {
    content = content.gsub!(
      /(?<!\{)\{(?!\{)/,
      "&amp;lbrace;"
    )
    content = content.gsub!(
      /(?<!\})\}(?!\})/,
      "&amp;rbrace;"
    )
  },

  angles: ->(content) {
    content = content.gsub!(
      /</,
      "&amp;lt;"
    )
    content = content.gsub!(
      />/,
      "&amp;gt;"
    )
  },
}
