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
  }
}
