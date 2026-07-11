##
# This file should be `require`d before using any other parts of Squarkdown.
# 
# It sets up routes, global variables and other things needed for Squarkdown to work nicely.
# 
# This module also pulls in all the `utils/` modules, so other files do not need to explicitly require them.

require_relative "../squark.version"
require_relative "./routes"

require_relative "utils/ansi"
require_relative "utils/log"
require_relative "utils/error"


## Should debug output be suppressed?
Silent = ARGV.include? "silent"

## Global paths for Squarkdown to operate relative to.
Routes = RoutesConfig.new()
