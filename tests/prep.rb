require "pathname"

require_relative "../scripts/prep-fonts"
require_relative "../scripts/prep-assets"
require_relative "../scripts/prep-scss"


Current = Pathname(__dir__)


def test_prep_fonts
  fonts = load_fonts()
  assert fonts.is_a?(String)
  assert fonts.include("Testing")
end


def test_prep_assets
  prep_assets()

  route = Current / "static"
  assert route.exist?
  assert (route / "test.png").exist?
  assert (route / "test.jpeg").exist?
  assert (route / "test.svg").exist?
  assert (route / ".include/test.png").exist?
end


def test_prep_scss
  scss = load_scss()
  assert scss.is_a?(Array)
  assert scss.length == 2
  assert scss.include?(Current / "resources" / "~test.scss")
  assert scss.include?(Current / "resources" / "~testing.scss")
  assert !scss.include?(Current / "resources" / "ignore.scss")
end
