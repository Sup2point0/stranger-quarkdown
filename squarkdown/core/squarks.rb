Replace = {
  /\n?<!-- ?#SQUARK leave\?.*?#SQUARK ?leave\. -->\n?/m =>
    "",

  # Constants
  # /#SQUARK font-(.*?) / =>
  #   styleMixins['\1'].join(", "),

  # Cleanup
  /\n?<!-- ?#SQUARK only\?/ =>
    "",
  /#SQUARK only\. ?-->\n?/ =>
    "",

  /\n?<!-- ?#SQUARK.*?-->\n?/m =>
    "",
}
