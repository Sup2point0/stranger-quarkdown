Replace = {
  /\n?<!-- ?#SQUARK leave\?.*?#SQUARK ?leave\. ?-->\n?/m =>
    "",

  /\n?<!-- ?#SQUARK only\?/ =>
    "",
  /#SQUARK only\. ?-->\n?/ =>
    "",
}
