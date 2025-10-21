# frozen_string_literal: true

require "rake/clean"
require "rubygems/tasks"
require "rake/testtask"
require "rb_sys/extensiontask"
require "yard"

task default: :test

Gem::Tasks.new
YARD::Rake::YardocTask.new

GEMSPEC = Gem::Specification.load("whatlang.gemspec")
RbSys::ExtensionTask.new("whatlang-rb", GEMSPEC)
task build: :compile

Rake::TestTask.new
task test: :compile
