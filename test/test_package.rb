require "test/unit"
require "test/unit/notify"
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
    def test_install
      gemspec = Gem::Specification.load("whatlang.gemspec")
      Dir.mktmpdir do |dir|
        dir = File.realpath(dir)
        gemfile = File.join(dir, gemspec.full_name + ".gem")
        system "gem", "build", "whatlang.gemspec", "--output", gemfile, exception: true
        install_dir = File.join(dir, "install")
        FileUtils.mkdir install_dir
        system "gem", "install", "--install-dir", install_dir, "--no-document", gemfile, exception: true
        assert_installed install_dir, gemspec.version

        libdir = File.join(install_dir, "gems", "#{gemspec.name}-#{gemspec.version}", "lib")
        assert_match(/ita/, `ruby -I #{libdir.shellescape} -r whatlang -e 'print Whatlang.detect("Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo.", allowlist: ["eng", "ita"]).lang.code'`)
      end
    end

    private

    def assert_installed(dir, version)
      assert_path_exist File.join(dir, "gems/whatlang-#{version}/lib", "whatlang.#{RbConfig::CONFIG["DLEXT"]}")
    end
  end
end
