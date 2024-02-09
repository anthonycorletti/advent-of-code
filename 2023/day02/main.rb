class Possibility
    def initialize(red_count=12, green_count=13, blue_count=14)
        @input = File.readlines("input.txt")
        @red_count = red_count
        @green_count = green_count
        @blue_count = blue_count
    end

    def part_one
        @input.map { |game| game_id_if_possible(game) } .sum
    end

    def game_id_if_possible(game)
        g, sets = game.chomp().split(":")
        game_id = g.split(" ").last
        sets = sets.gsub(/^\s/,"").split("; ")
        for set in sets
            for count_color in set.split(", ")
                count, color = count_color.split(" ")
                if (color == "red" && count.to_i > @red_count) || (color == "green" && count.to_i > @green_count) || (color == "blue" && count.to_i > @blue_count)
                    return 0
                end
            end
        end
        return game_id.to_i
    end

    def part_two
        @input.map { |game| min_count_game_power(game) } .sum
    end

    def min_count_game_power(game)
        totals = [0,0,0]
        g, sets = game.chomp().split(":")
        sets = sets.gsub(/^\s/,"").split("; ")
        for set in sets
            for count_color in set.split(", ")
                count, color = count_color.split(" ")
                if (color == "red" && count.to_i > totals[0])
                    totals[0] = count.to_i
                elsif (color == "green" && count.to_i > totals[1])
                    totals[1] = count.to_i
                elsif (color == "blue" && count.to_i > totals[2])
                    totals[2] = count.to_i
                end
            end
        end
        return multiply_array(totals)
    end

    def multiply_array(arr)
        arr.inject(:*)
    end
end

p = Possibility.new
puts p.part_one
puts p.part_two
