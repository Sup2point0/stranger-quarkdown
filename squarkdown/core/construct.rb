# A lot of data is processed and collected during squarkup. It’d be a waste for it to all be discarded, so we store it all in a JSON file for access post-preprocessing.

require "json"


def build_site_data(
  pages:
)
  return {
    "pages" => pages
  }
end


def save_site_data(data)
  repr = JSON.generate(data)
  repr.sub!(/"(.*?)":/, /\1:/)

  route = Routes.site / "src/site-config.js"
  content = File.read(route)
  content.sub!(
    /siteData = .*export/m,
    "siteData = #{repr}export"
  )

  update = Date.now.strftime("%d %B %y")
  content.sub!(/generated: .*/, "generated: " + update)

  File.write(route, content)
end
