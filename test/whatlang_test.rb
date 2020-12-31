# frozen_string_literal: true

require_relative "helper"

class WhatlangTest < Test::Unit::TestCase
  test "VERSION" do
    assert do
      ::Whatlang.const_defined?(:VERSION)
    end
  end

  test "something useful" do
    assert_equal("expected", "actual")
  end
end
