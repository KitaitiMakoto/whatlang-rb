require "json"

Gem::Specification.new do |spec|
  spec.name          = "whatlang"
  spec.version       = JSON.parse(`cargo metadata --no-deps --format-version=1 --manifest-path=ext/Cargo.toml`)["packages"][0]["version"]
  spec.license       = "Ruby"
  spec.authors       = ["Kitaiti Makoto"]
  spec.email         = ["KitaitiMakoto@gmail.com"]

  spec.summary       = "Fast natural language detection library."
  spec.description   = "Ruby bindings for Whatlang, a natural language detection for Rust."
  spec.homepage      = "https://gitlab.com/KitaitiMakoto/whatlang-rb"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://gitlab.com/KitaitiMakoto/whatlang-rb"
  spec.metadata["changelog_uri"] = "https://gitlab.com/KitaitiMakoto/whatlang-rb/-/blob/master/CHANGELOG.md"

  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0")
  end
  spec.extensions = ["ext/Cargo.toml"]

  spec.add_development_dependency "test-unit"
  spec.add_development_dependency "test-unit-notify"
  spec.add_development_dependency "rake"
  spec.add_development_dependency "yard"
  spec.add_development_dependency "rubygems-tasks"
  spec.add_development_dependency "kar"
  spec.add_development_dependency "terminal-notifier"
end
