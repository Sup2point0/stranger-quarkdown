# A lot of data is processed and collected during squarkup. It’d be a waste for it to all be discarded, so we store it all in a JSON file for access post-preprocessing.

require "json"


def save_site_data(data, repo_config:)
  route = Routes.site / repo_config["site-data"]
  File.write(route, data)
end
