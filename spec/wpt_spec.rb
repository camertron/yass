# frozen_string_literal: true

require "spec_helper"
require "json"

RSpec.describe Yass, slow: true do
  dirname = File.join(__dir__, "wpt", "fixtures")

  Dir.chdir(dirname) do
    Dir.glob(File.join("**", "*.css")) do |css_path|
      it "parses #{css_path}" do
        full_css_path = File.join(dirname, css_path)
        sheet = Yass::Parser.parse(File.read(full_css_path))
        expect(sheet.to_h.to_json.size).to be > 10
      end
    end
  end
end
