## Implements the `Vars` mixin for fetching the instance variables of an object as a hash.

module Vars
  def vars_str
    Hash[
      self.instance_variables.map { |var|
        [
          var.to_s.delete("@"),
          instance_variable_get(var)
        ]
      }
    ]
  end
  
  def vars_sym
    Hash[
      self.instance_variables.map { |var|
        [
          var.to_s.delete("@").to_sym,
          instance_variable_get(var)
        ]
      }
    ]
  end
end
