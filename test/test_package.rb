require "test/unit"
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
        dir = File.realpath(dir)
        system "gem", "install", "--install-dir", dir.shellescape, "--no-document", "pkg/#{gemspec.file_name.shellescape}", exception: true
        assert_installed dir, gemspec.version

        libdir = File.join(dir, "gems", "#{gemspec.name}-#{gemspec.version}", "lib")
        assert_match(/ita/, `ruby -I #{libdir.shellescape} -r whatlang -e 'print Whatlang.detect("Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo.", allowlist: ["eng", "ita"]).lang.code'`)
      end
    end

    private

    def assert_installed(dir, version)
      assert_path_exist File.join(dir, "gems/whatlang-#{version}/lib", "whatlang.#{RbConfig::CONFIG["DLEXT"]}")
    end
  end
end
