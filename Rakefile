EXTENSION_GEMS = %w[c_extension magnus_extension rb_sys_extension]

task :compile_extensions do
  EXTENSION_GEMS.each do |dir|
    Dir.chdir(dir) do
      system("rake compile")
    end
  end
end

task :clean do
  EXTENSION_GEMS.each do |dir|
    Dir.chdir(dir) do
      system("rake clean clobber")
    end
  end
end

task :benchmark => :compile_extensions do
  system("bundle exec ruby benchmark.rb")
end
