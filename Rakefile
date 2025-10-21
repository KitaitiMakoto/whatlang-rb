# frozen_string_literal: true

require "rake/clean"
require "rubygems/ext"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "shellwords"

task default: :test

Gem::Tasks.new
YARD::Rake::YardocTask.new

CARGO_LOCK = "ext/Cargo.lock"
file CARGO_LOCK => "ext/Cargo.toml" do |t|
  system "cargo", "update", "--manifest-path", t.source, `cargo pkgid --manifest-path=#{t.source.shellescape}`.chomp, exception: true
end

EXTENSION = "lib/whatlang/whatlang.#{RbConfig::CONFIG["DLEXT"]}"
file EXTENSION => CARGO_LOCK do
  results = Rake.verbose == true ? $stdout : []
  Gem::Ext::CargoBuilder.new.build "ext/Cargo.toml", ".", results, [], "lib", File.expand_path("ext")
end
CLEAN.include "whatlang.bundle"
CLOBBER.include EXTENSION

Rake::TestTask.new
task test: EXTENSION

task doc: :yard do
  system "cargo", "doc", "--manifest-path", "ext/Cargo.toml", exception: true
end
