require "rake/clean"
require "rubygems/ext"
require "rubygems/tasks"
require "rake/testtask"
require "yard"
require "shellwords"

task default: :test

GEMSPEC = Gem::Specification.load("whatlang.gemspec")
MANIFEST = GEMSPEC.extensions.first

CARGO_LOCK = "ext/Cargo.lock"
file CARGO_LOCK => MANIFEST do |t|
  pkgid = `cargo pkgid --manifest-path=#{t.source.shellescape}`.chomp
  system "cargo", "update", "--manifest-path", t.source, pkgid, exception: true
end

DL_NAME = "#{GEMSPEC.name}.#{RbConfig::CONFIG["DLEXT"]}"
DL_PATH = File.join("lib", DL_NAME)
SRC = FileList["ext/src/**/*.rs"]
file DL_PATH => [CARGO_LOCK] + SRC do
  results = Rake.verbose == true ? $stdout : []
  begin
    Gem::Ext::CargoBuilder.new.build MANIFEST, ".", results, [], "lib", File.expand_path("ext")
  rescue => error
    $stderr.puts results unless Rake.verbose == true
    fail
  end
end
CLEAN.include DL_NAME
CLOBBER.include DL_PATH

Gem::Tasks.new
task build: CARGO_LOCK
CLOBBER.include("pkg/#{GEMSPEC.file_name}")

Rake::TestTask.new
task test: DL_PATH

YARD::Rake::YardocTask.new
desc "Generate Ruby and Rust documentation"
task doc: :yard do
  system "cargo", "doc", "--manifest-path", MANIFEST, exception: true
end
