def error(e, repo_config:)
  if repo_config["opts / on-error"] == "kill"
    raise e
  else
    log error: e.to_s
  end
end
