class CardPoints
    def initialize
        @input = File.readlines("input.txt").map(&:chomp)
    end

    def num_matches(winning_numbers, current_values)
        result = -1
        current_values.each do |num|
            result += 1 if winning_numbers.include?(num)
        end
        result
    end

    def part_one
        total = 0
        @input.map do |line|
            winning_numbers = line.split(":").last.split("|").first.split(" ").map(&:to_i)
            current_values = line.split("|").last.split(" ").map(&:to_i)
            num_matches = num_matches(winning_numbers, current_values)
            if num_matches > -1
                total += 2 ** num_matches
            end
        end
        total
    end

    def part_two
        @input.each_with_object(Hash.new(0)) { |line, counts|
            id, winners, numbers = line.split(/[:|]/)

            id = id.scan(/\d+/).first.to_i
            winners = winners.scan(/\d+/)
            numbers = numbers.scan(/\d+/)
            matches = (winners & numbers).count

            counts[id] += 1
            matches.times do |m|
                card_id = id + m + 1
                counts[card_id] += counts[id]
            end
        }.values.sum
    end
end


card = CardPoints.new
puts card.part_one
puts card.part_two
