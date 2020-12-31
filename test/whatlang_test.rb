# frozen_string_literal: true

require_relative "helper"

class WhatlangTest < Test::Unit::TestCase
  def test_detect
    text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu!"
    list = ["eng", "ita"]

    assert_equal "epo", Whatlang.detect(text).lang.code
    assert_equal "ita", Whatlang.detect(text, whitelist: list).lang.code
    assert_equal "epo", Whatlang.detect(text, blacklist: list).lang.code
    assert_raise ArgumentError do
      Whatlang.detect(text, whitelist: list, blacklist: list)
    end
  end

  def test_detect_without_options
    text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu!"
    info = Whatlang.detect_without_options(text)
    assert_equal "epo", info.lang.code
    assert_equal "Latin", info.script
  end

  def test_detect_with_whitelist
    text = "Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo."
    list = ["eng", "ita"]
    assert_equal "ita", Whatlang.detect_with_whitelist(text, list).lang.code
  end

  def test_detect_with_blacklist
    text = "Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo."
    list = ["eng", "ita"]
    assert_equal "epo", Whatlang.detect_with_blacklist(text, list).lang.code
  end

  def test_detect_lang
    text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"
    assert_equal "epo", Whatlang.detect_lang(text).code
  end

  def test_detect_script
    text = "Благодаря Эсперанто вы обрётете друзей по всему миру!"
    assert_equal "Cyrillic", Whatlang.detect_script(text)
  end
end
