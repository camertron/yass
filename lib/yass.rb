# frozen_string_literal: true

require_relative "yass/version"

# Tries to require the precompiled extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "yass/#{Regexp.last_match(1)}/yass"
rescue LoadError
  require_relative "yass/yass"
end

module Yass
end
