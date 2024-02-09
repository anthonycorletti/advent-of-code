class EngineSchematic
    def initialize
        @input = File.readlines('input.txt').map(&:chomp)
    end

    def scan_for_numbers(i)
        @input[i].enum_for(:scan, /\d+/).map { |n| [n, Regexp.last_match.begin(0)] }
    end

    def asterisk_indices(i)
        @input[i].enum_for(:scan, /\*/).map { Regexp.last_match.begin(0) }
    end

    def part_one
        s = 0
        for i in 0..@input.length-1
            r = scan_for_numbers(i)
            for num in r
                for c in 0..num[0].length-1
                    n = num[1] + c
                    # look up
                    if i > 0
                        if @input[i-1][n] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                    # look down
                    if i < @input.length-1
                        if @input[i+1][n] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                    # look left
                    if n > 0
                        if @input[i][n-1] != '.' and c == 0
                            s += num[0].to_i
                            break
                        end
                    end
                    # look right
                    if n < @input[i].length-1
                        if @input[i][n+1] != '.' and c == num[0].length-1
                            s += num[0].to_i
                            break
                        end
                    end
                    # look up-left
                    if i > 0 and n > 0
                        if @input[i-1][n-1] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                    # look up-right
                    if i > 0 and n < @input[i].length-1
                        if @input[i-1][n+1] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                    # look down-left
                    if i < @input.length-1 and n > 0
                        if @input[i+1][n-1] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                    # look down-right
                    if i < @input.length-1 and n < @input[i].length-1
                        if @input[i+1][n+1] != '.'
                            s += num[0].to_i
                            break
                        end
                    end
                end
            end
        end
        s
    end

    def part_two
        cur_num = 0
        gears = {}
        gear_coordinates = []
        @input.each_with_index do |line, line_index|
            line.split('').each_with_index do |char, index|
                if char =~ /\d/
                    cur_num = cur_num * 10 + char.to_i
                    x, y = adjecent_symbol_coords(index, line_index) { |c| c == '*' }
                    gear_coordinates << { x:, y: } if x && y
                else
                    if cur_num.positive? && !gear_coordinates.empty?
                        gear_coordinates.uniq!
                        gear_coordinates.each do |coord|
                            if gears.key? coord
                                gears[coord] = { pair?: true, ratio: gears[coord][:ratio] * cur_num }
                            else
                                gears[coord] = { pair?: false, ratio: cur_num }
                            end
                        end
                    end
                    gear_coordinates = []
                    cur_num = 0
                end
            end
        end

        gears.values.select { |v| v[:pair?] }.sum { |v| v[:ratio] }
    end

    def adjecent_symbol_coords(x, y)
        (-1..1).each do |dx|
            (-1..1).each do |dy|
                next if dx.zero? && dy.zero?

                xi = x + dx
                yi = y + dy

                next if xi.negative? || xi > @input[y].length - 1 || yi.negative? || yi > @input.length - 1

                return [xi, yi] if yield @input[yi][xi]
            end
        end
        false
    end
end

es = EngineSchematic.new
puts es.part_one
puts es.part_two
