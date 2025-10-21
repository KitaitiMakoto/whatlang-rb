# frozen_string_literal: true

require "rake/clean"
require "rubygems/ext"
require "rubygems/tasks"
require "rake/testtask"
require "yard"

task default: :test

Gem::Tasks.new
YARD::Rake::YardocTask.new

CARGO_LOCK = "ext/whatlang-rb/Cargo.lock"
file CARGO_LOCK => "ext/whatlang-rb/Cargo.toml" do |t|
  system "cargo", "update", "--manifest-path", t.source, "whatlang-rb", exception: true
end

EXTENSION = "lib/whatlang-rb/whatlang_rb.#{RbConfig::CONFIG["DLEXT"]}"
file EXTENSION => CARGO_LOCK do
  results = Rake.verbose == true ? $stdout : []
  Gem::Ext::CargoBuilder.new.build "ext/whatlang-rb/Cargo.toml", ".", results, [], "lib", File.expand_path("ext/whatlang-rb")
end
CLEAN.include "whatlang-rb"
CLOBBER.include EXTENSION

Rake::TestTask.new
task test: EXTENSION
