require_relative "../squarkdown/core/process"


def extract(content, fill_defaults: false)
  return extract_data(
    lines: content.split("\n"),
    repo_config: RepoConfig,
    fill_defaults:
  )
end


class Test_Process_Flags_ < Minitest::Test

  def test_live
    content =
"""
# Testing
<!-- #SQUARK live! -->
""".lstrip

    data = extract(content)

    assert_equal true, data.live
  end


  def test_dead
    content =
"""
# Testing
<!-- #SQUARK dead! -->
""".lstrip

    data = extract(content)

    assert_nil data
  end


  def test_flags
    content =
"""
# Testing
<!-- #SQUARK live! index! feat! woozy! dev! -->
""".lstrip

    data = extract(content)

    assert_includes data.flags, "live"
    assert_includes data.flags, "index"
    assert_includes data.flags, "feat"
    assert_includes data.flags, "woozy"
    assert_includes data.flags, "dev"
    assert_equal 5, data.flags.length  # FIXME `-->` being picked up as a flag 💀
  end

end


class Test_Process_Fields_ < Minitest::Test

  def test_fields
    content =
"""
# Testing
<!-- #SQUARK live!
| dest    = testing/fields
| capt    = A unit test
| title   = Squarkdown Tests
| desc    = Does it work?
| style   = #AUTO / test
| duality = dark
| index   = tests
| tags    = #INDEX / testing
| date    = 1984 April 1
-->
""".lstrip

    data = extract(content, fill_defaults: true)

    assert_equal "testing/fields",     data.dest
    assert_equal "Testing",            data.head
    assert_equal "A unit test",        data.capt
    assert_equal "Squarkdown Tests",   data.title
    assert_equal "Does it work?",      data.desc
    assert_equal ["test"],             data.style
    assert_equal "dark",               data.duality
    assert_equal ["tests"],            data.index
    assert_equal ["tests", "testing"], data.tags
    assert !data.date.nil?
  end


  def test_fields_multiline
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/fields-expanded
| capt = A
    rather long caption
| title =
    Squarkdown is very awesome
| desc = Never
         gonna
          give
           you
            up
| style =
  / #AUTO
  / test
| tags =
    tests /
    testing /
-->
""".lstrip

    data = extract(content, fill_defaults: true)

    assert_equal "testing/fields-expanded",    data.dest
    assert_equal "A rather long caption",      data.capt
    assert_equal "Squarkdown is very awesome", data.title
    assert_equal "Never gonna give you up",    data.desc
    assert_equal ["test"],                     data.style
    assert_equal ["tests", "testing"],         data.tags
  end


  def test_fields_default
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = test/defaults
-->
""".lstrip

    data = extract(content, fill_defaults: true)

    assert_equal "test/defaults", data.dest
    assert_equal "Testing",       data.head
    assert_equal "Testing",       data.title
    assert_equal [],              data.style
    assert_equal "light",         data.duality
    assert_equal [],              data.index
    assert_equal [],              data.tags
    assert_nil data.date
    assert_nil data.update
  end


  def test_head
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/head
-->
"""
    
    lines = content.split("\n")

    data = extract_data(
      lines:,
      repo_config: RepoConfig,
      fill_defaults: false
    )

    assert_equal "Testing", data.head
    assert !lines.include?("# Testing")
    assert !lines.include?("Testing")
  end


  def test_date
    # year + month + day
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| date = 1984 April 2
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(1984, 4, 2), data.date
    assert_equal Date.new(1984, 4, 2), data.update
    
    # year + month
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| date = 1984 April
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(1984, 4, 1), data.date
    assert_equal Date.new(1984, 4, 1), data.update
    
    # year + season
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| date = 1984 winter
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(1984, 12, 31), data.date
    assert_equal Date.new(1984, 12, 31), data.update
    
    # year
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| date = 1984
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(1984, 1, 1), data.date
    assert_equal Date.new(1984, 1, 1), data.update

    # error
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| date = winter 2077
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_nil data.date
    assert_nil data.update
  end


  def test_update
    # year + month + day
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| update = 2069 April 2
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(2069, 4, 2), data.update
    assert_nil data.date
    
    # year + month
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| update = 2069 April
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(2069, 4, 1), data.update
    assert_nil data.date
    
    # year
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| update = 2069
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_equal Date.new(2069, 1, 1), data.update
    assert_nil data.date
    
    # error
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/dates
| update = future
-->
"""
    
    data = extract(content, fill_defaults: true)

    assert_nil data.update
    assert_nil data.date
  end


  def test_excess
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/excess
-->

<!-- #SQUARK live!
| dest = testing/wrong
-->
"""

    data = extract(content)

    assert_equal "testing/excess", data.dest
  end


  def test_arbitrary
    content =
"""
# Testing
<!-- #SQUARK live!
| dest = testing/arbitrary
---
| arbitrary = sup
| arbitrary-long = / a singleton collection
| arbitrary-poly = many / arbitrary / values
-->
"""

    data = extract(content)

    assert_equal "sup",                           data.rest["arbitrary"]
    assert_equal ["a singleton collection"],      data.rest["arbitrary_long"]
    assert_equal ["many", "arbitrary", "values"], data.rest["arbitrary_poly"]
  end

end
