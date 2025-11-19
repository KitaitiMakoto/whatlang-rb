require "rake/clean"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "kar/dsl"

task default: :test

cargo "whatlang"

gem_tasks = Gem::Tasks.new
task build: "cargo:check"
CLOBBER.include(gem_tasks.build.gem.project.builds["whatlang"][:gem])

Rake::TestTask.new test: :cargo

YARD::Rake::YardocTask.new
desc "Generate Ruby and Rust documentation"
task doc: :yard do
  system "cargo", "doc", "--manifest-path", MANIFEST, exception: true
end
