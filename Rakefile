# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

task default: :test

RUST_TARGET = "target/release/libwhatlang_rb.so"
RUST_SRC = FileList["src/**/*.rs"]

RUST_SRC.each do |path|
  file path
end

file RUST_TARGET => RUST_SRC do
  sh "cargo build --release"
end
task test: RUST_TARGET
