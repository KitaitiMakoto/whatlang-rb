require "rutie"

Rutie.new(:whatlang_rb).init "Init_whatlang", __dir__

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

  class Info
    attr_reader :lang, :script, :confidence

    def initialize(lang, script, is_reliable, confidence)
      @lang = lang
      @script = script
      @is_reliable = is_reliable
      @confidence = confidence
    end

    def reliable?
      @is_reliable
    end
  end

  class Lang
    attr_reader :code, :name, :eng_name

    def initialize(code, name, eng_name)
      @code = code
      @name = name
      @eng_name = eng_name
    end
  end

  private

  # Class returnned by detect when no lang detected.
  # 
  # Needed because Rutie cause segmentation fault when it returns NilClass
  NO_INFO = nil
end
