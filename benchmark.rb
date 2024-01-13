require 'c_extension'
require 'magnus_extension'
require 'benchmark/ips'

PAYLOAD = "ABC"*100

def build_tree(depth)
  if depth == 1
    return {label: PAYLOAD.dup , children: []}
  end
  return {label: PAYLOAD.dup, children: [build_tree(depth-1), build_tree(depth-1)]}
end

def build_big_tree
  build_tree(13)
end

# Sanity check.
a = build_big_tree
b = CExtension.build_big_tree
c = MagnusExtension.build_big_tree
raise "This shouldn't happen" unless a == b && b == c

r = Benchmark.ips(format: :raw) do |x|
  x.report('Plain Ruby') { build_big_tree }
  x.report('C extension') { CExtension.build_big_tree }
  x.report('Magnus extension') { MagnusExtension.build_big_tree }
  x.compare!
end

r.data.each do |entry|
  iterations = entry.fetch(:iterations)
  time = entry.fetch(:microseconds)
  puts "#{entry.fetch(:name)} = #{format("%.2f", time/iterations/1000)} ms"
end

