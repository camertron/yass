# Tries to find places where CachedValue or CachedValueList is used without an
# accompanying mark function

rust_files = Dir.glob("ext/yass/src/**/*.rs")

rust_files.flat_map do |rust_file|
  rust_code = File.read(rust_file)

  rust_code.scan(/#\[magnus(?:\:\:wrap)?\(class = "([\w:]+)"(, mark)?\)\]\s+pub struct (\w+)/).map do |ruby_class_name, mark, rust_struct_name|
    next unless ruby_class_name && rust_struct_name

    m = rust_code.match(/impl\s+#{rust_struct_name}\s+(\{)/)
    next unless m

    start = pos = m.end(1)
    count = 1

    while count > 0
      case rust_code[pos]
      when "{"
        count += 1
      when "}"
        count -= 1
      end

      pos += 1
    end

    impl_body = rust_code[start..pos]

    implements_data_type_functions = rust_code.include?("impl DataTypeFunctions for #{rust_struct_name}")

    if impl_body.include?("CachedValue")
      if !mark || !implements_data_type_functions
        puts rust_file
      end
    end
  end
end
