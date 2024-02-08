class Calibrator
    REGEXP = Regexp.new(/\d|one|two|three|four|five|six|seven|eight|nine/)
    REGEXP_LAST = Regexp.new(/.*(\d|one|two|three|four|five|six|seven|eight|nine).*$/)
    @@words_to_numbers = {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9
    }

    def initialize(file_path="input.txt")
        @input = File.readlines(file_path)
    end

    def connect_first_and_last_digits_as_int(s)
        digits = s.split("").select { |c| c.match(/\d/) }
        "#{digits.first}#{digits.last}".to_i
    end

    def part_one
        @input.map { |s| connect_first_and_last_digits_as_int(s) } .sum
    end

    def map_words_to_numbers(s)
        # find the first number that appears from the left of the string it can be a number or a word that matches REGEXP
        # find the first number that appears from the right of the string it can be a number or a word that matches REGEXP_LAST

        left_num = s[REGEXP]
        right_num = s[REGEXP_LAST, 1]

        # if the left number is a word, map it to a number
        left_num = @@words_to_numbers[left_num] if @@words_to_numbers[left_num]

        # if the right number is a word, map it to a number
        right_num = @@words_to_numbers[right_num] if @@words_to_numbers[right_num]

        # if the left number is nil, then the right number is the only number in the string
        left_num = right_num if left_num.nil?

        "#{left_num}#{right_num}".to_i
    end

    def part_two
        @input.map { |s| map_words_to_numbers(s) } .sum
    end
end

puts Calibrator.new(file_path="input.txt").part_one
puts Calibrator.new(file_path="input.txt").part_two
