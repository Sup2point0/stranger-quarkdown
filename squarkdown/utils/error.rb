## :: Error -> &RepoConfig -> Result IO Error
#
# Raise an error for Squarkdown to handle.
def squark_error(e, repo_config:)
  if repo_config.opts.on_error == "kill"
    raise e
  else
    log error: e.to_s
    log error: e.backtrace[0]
  end
end
