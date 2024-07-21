# A lot of data is processed and collected during squarkup. It’d be a waste for it to all be discarded, so we store it all in a JSON file for access post-preprocessing.

require "json"


def build_site_data(
  pages:
)
  return {
    "pages" => pages
  }
end


def save_site_data(data, repo_config:)
  repr = JSON.generate(data)
  repr.gsub!(/"(.*?)":/, '\1:')

  route = Routes.site / repo_config["site-data"]
  content = File.read(route)
  content.sub!(
    /Site = .*?export default Site;/m,
    "Site = #{repr}export default Site;"
  )

  # update = Date.now.strftime("%d %B %y")
  # content.sub!(/generated: .*/, "generated: " + update)

  File.write(route, content)
end
