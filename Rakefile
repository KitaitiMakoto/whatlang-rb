require "rake/clean"
require "rubygems/ext"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "shellwords"
require "kar/dsl"

task default: :test

GEMSPEC = Gem::Specification.load("whatlang.gemspec")

cargo "whatlang"

Gem::Tasks.new
task build: "cargo:validate"
CLOBBER.include("pkg/#{GEMSPEC.file_name}")

Rake::TestTask.new
task test: :cargo

YARD::Rake::YardocTask.new
desc "Generate Ruby and Rust documentation"
task doc: :yard do
  system "cargo", "doc", "--manifest-path", MANIFEST, exception: true
end
