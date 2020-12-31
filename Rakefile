# frozen_string_literal: true

require "rubygems/tasks"
require "rake/testtask"
require "yard"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

task default: :test

Gem::Tasks.new
YARD::Rake::YardocTask.new

RUST_TARGET = "target/release/libwhatlang_rb.so"
RUST_SRC = FileList["src/**/*.rs"]

RUST_SRC.each do |path|
  file path
end

file RUST_TARGET => RUST_SRC do
  sh "cargo build --release"
end
task test: RUST_TARGET
