require "whatlang_rb"

module Whatlang
  class << self
    def detect(text, allowlist: nil, denylist: nil)
      if denylist && allowlist
        raise ArgumentError, "Couldn't specify `allowlist' and `denylist' at a time. Choose one."
      end

      text = text.to_s

      case
      when allowlist
        detect_with_allowlist(text, allowlist)
      when denylist
        detect_with_denylist(text, denylist)
      else
        detect_without_options(text)
      end
    end
  end
end
