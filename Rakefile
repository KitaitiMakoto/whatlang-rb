require "rake/clean"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "kar/dsl"

TARGET = "target/release/libwhatlang.#{RbConfig::CONFIG['SOEXT']}"

task default: :test

gem_tasks = Gem::Tasks.new

file TARGET do |t|
  chdir "ext" do
    sh "rake"
  end
end

Rake::TestTask.new test: TARGET

YARD::Rake::YardocTask.new
desc "Generate Ruby and Rust documentation"
task doc: :yard do
  system "cargo", "doc", "--manifest-path", MANIFEST, exception: true
end
