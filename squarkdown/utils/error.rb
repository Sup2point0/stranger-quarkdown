## :: Error -> *RepoConfig -> ()
#
# Raise an error for Squarkdown to handle.
def squark_error(e, repo_config:)
  if repo_config.opts.on_error == "kill"
    raise e
  else
    log error: e.to_s
  end
end
