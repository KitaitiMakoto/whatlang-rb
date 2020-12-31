require "rutie"

Rutie.new(:whatlang_rb).init "Init_whatlang", __dir__

module Whatlang
  class << self
    def detect(text, whitelist: nil, blacklist: nil)
      if blacklist && whitelist
        raise ArgumentError, "Couldn't specify `whitelist' and `blacklist' at a time. Choose one."
      end

      case
      when whitelist
        detect_with_whitelist(text, whitelist)
      when blacklist
        detect_with_blacklist(text, blacklist)
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
end
