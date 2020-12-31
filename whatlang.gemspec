# frozen_string_literal: true

require "tomlrb"

Gem::Specification.new do |spec|
  spec.name          = "whatlang"
  spec.version       = Tomlrb.load_file("Cargo.toml")["package"]["version"]
  spec.license       = "AGPL-3.0-or-later"
  spec.authors       = ["Kitaiti Makoto"]
  spec.email         = ["KitaitiMakoto@gmail.com"]

  spec.summary       = "Natural language detection."
  spec.description   = "Ruby bindings for whatlang, Natural language detection for Rust."
  spec.homepage      = "https://gitlab.com/KitaitiMakoto/whatlang-rb"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://gitlab.com/KitaitiMakoto/whatlang-rb"
  spec.metadata["changelog_uri"] = "https://gitlab.com/KitaitiMakoto/whatlang-rb/-/blob/master/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{\A(?:test|spec|features)/}) }
  end
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/Rakefile"]

  spec.add_runtime_dependency "rutie"

  spec.add_development_dependency "tomlrb"
  spec.add_development_dependency "test-unit"
  spec.add_development_dependency "rake"

  # For more information and examples about making a new gem, checkout our
  # guide at: https://bundler.io/guides/creating_gem.html
end
