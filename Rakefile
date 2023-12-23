# frozen_string_literal: true

require "rake/clean"
require "rubygems/tasks"
require "rake/testtask"
require "yard"

Rake::TestTask.new

task default: :test

Gem::Tasks.new
YARD::Rake::YardocTask.new

RUST_TARGET = "target/release/libwhatlang_rb.so"
RUST_SRC = FileList["src/**/*.rs"]

RUST_SRC.each do |path|
  file path
end

file RUST_TARGET => RUST_SRC + ["Cargo.toml", "Cargo.lock"] do
  sh "cargo build --release"
end
CLEAN.include RUST_TARGET
task test: RUST_TARGET
