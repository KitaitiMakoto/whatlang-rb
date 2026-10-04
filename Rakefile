require "rake/clean"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "rutie/rake_task"
require "kar/dsl"

TARGET = "target/release/libwhatlang.#{RbConfig::CONFIG['SOEXT']}"
RUST_SRC = FileList["src/**/*.rs"]

task default: :test

gem_tasks = Gem::Tasks.new

file TARGET => RUST_SRC + ["Cargo.toml", "Cargo.lock", "ext/Rakefile"] do |t|
  chdir "ext" do
    sh "rake"
  end
end

Rake::TestTask.new test: TARGET
Rutie::RakeTask.new

task clean: "rutie:clean"

YARD::Rake::YardocTask.new
desc "Generate Ruby and Rust documentation"
task doc: :yard do
  system "cargo", "doc", "--manifest-path", MANIFEST, exception: true
end
