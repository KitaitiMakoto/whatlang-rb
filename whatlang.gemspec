require "tomlrb"

Gem::Specification.new do |spec|
  spec.name          = "whatlang"
  spec.version       = Tomlrb.load_file("Cargo.toml")["package"]["version"]
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
  spec.extensions = ["ext/Rakefile"]

  spec.add_runtime_dependency "rutie"

  spec.add_development_dependency "tomlrb"
  spec.add_development_dependency "test-unit"
  spec.add_development_dependency "rake"
  spec.add_development_dependency "yard"
  spec.add_development_dependency "rubygems-tasks"
end
