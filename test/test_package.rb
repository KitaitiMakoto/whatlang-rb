require_relative "helper"
require 'tempfile'
require 'tmpdir'
require 'shellwords'

class TestPackage < Test::Unit::TestCase
  def test_build
    Tempfile.create do |file|
      assert system("gem", "build", "whatlang.gemspec", "--output", file.to_path.shellescape, exception: true)
      assert file.size > 0
      assert_path_exist file.to_path
    end
  end

  class TestInstall < self
    def setup
      system "rake", "build", exception: true
    end

    def test_install
      gemspec = Gem::Specification.load("whatlang.gemspec")
      Dir.mktmpdir do |dir|
        system "gem", "install", "--install-dir", dir.shellescape, "--no-document", "pkg/#{gemspec.file_name.shellescape}", exception: true
        assert_installed dir, gemspec.version
      end
    end

    private

    def assert_installed(dir, version)
      assert_path_exist File.join(dir, "gems/whatlang-#{version}/lib", "whatlang_rb.#{RbConfig::CONFIG["DLEXT"]}")
    end
  end
end
