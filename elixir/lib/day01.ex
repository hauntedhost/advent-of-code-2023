defmodule AdventOfCode2023.Day01 do
  @moduledoc """
  Day 1: Trebuchet?!

  https://adventofcode.com/2023/day/1
  """

  def main do
    input =
      "../data/01-input.txt"
      |> File.read!()
      |> String.split("\n", trim: true)

    IO.inspect({
      part_one(input),
      part_two(input)
    })
  end

  def part_one(lines) do
    lines
    |> Enum.reduce([], fn line, acc ->
      matches =
        line
        |> String.split("", trim: true)
        |> Enum.with_index()
        |> Enum.reduce(%{}, fn {char, index}, acc ->
          if Regex.match?(~r/\d/, char) do
            # char is a digit, put it in the map
            Map.put(acc, index, char)
          else
            acc
          end
        end)

      {min, max} =
        {matches
         |> Map.keys()
         |> Enum.min(),
         matches
         |> Map.keys()
         |> Enum.max()}

      calibration = String.to_integer(matches[min] <> matches[max])
      [calibration | acc]
    end)
    |> Enum.sum()
  end

  def part_two(lines) do
    lookup = %{
      "one" => "1",
      "two" => "2",
      "three" => "3",
      "four" => "4",
      "five" => "5",
      "six" => "6",
      "seven" => "7",
      "eight" => "8",
      "nine" => "9"
    }

    calibrations =
      Enum.reduce(lines, [], fn line, acc ->
        # build a map of matching indices to numbers, e.g.
        # "two1nine" => %{0 => "2", 3 => "1", 4 => "9"}
        matches =
          line
          |> String.split("", trim: true)
          |> Enum.with_index()
          |> Enum.reduce(%{}, fn {char, index}, acc ->
            if Regex.match?(~r/\d/, char) do
              # char is a digit, put it in the map
              Map.put(acc, index, char)
            else
              slice = String.slice(line, index..-1)

              Enum.reduce_while(lookup, acc, fn {word, number}, acc ->
                if String.starts_with?(slice, word) do
                  # we're at a numeric word, put number in the map
                  {:halt, Map.put(acc, index, number)}
                else
                  {:cont, acc}
                end
              end)
            end
          end)

        # take min/max indexes from matches map as first/last index
        {first_idx, last_idx} =
          {matches
           |> Map.keys()
           |> Enum.min(),
           matches
           |> Map.keys()
           |> Enum.max()}

        # concat first/last index vals and convert to integer for calibration
        calibration = String.to_integer(matches[min_idx] <> matches[max_idx])

        [calibration | acc]
      end)

    Enum.sum(calibrations)
  end
end
