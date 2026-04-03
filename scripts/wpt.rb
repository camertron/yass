# require "nokogiri"
# require "fileutils"

# Dir.chdir("wpt/css") do
#   Dir.glob("**/*.html") do |html_path|
#     doc = Nokogiri::HTML(File.read(html_path))

#     dirname = File.dirname(html_path)
#     basename = File.basename(html_path)

#     css_dirname = File.join("../../spec/wpt/fixtures", dirname)
#     css_basename = "#{basename.chomp(".html")}.css"
#     css_path = File.join(css_dirname, css_basename)

#     FileUtils.mkdir_p(css_dirname)
#     File.write(css_path, doc.css("style").map(&:inner_text).join("\n"))

#     puts "Wrote #{css_path}"
#   end
# end

require "yass"
require "json"

Dir.glob("spec/wpt/fixtures/**/*.css") do |css_path|
  puts "Parsing #{css_path}"
  sheet = Yass::Parser.parse(File.read(css_path))
  puts sheet.to_h.to_json
end
