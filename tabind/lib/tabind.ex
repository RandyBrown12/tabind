defmodule TABIND do

  def is_legal_word?(word) do
    word_letters = MapSet.new(String.graphemes(word))
    tabind_letters = MapSet.new(["T","A","B","I","N","D"])
    intersection = MapSet.intersection(word_letters, tabind_letters)
    MapSet.size(intersection) == String.length(word)
  end

  def all_legal_words do
    words = File.read!("../Collins Scrabble Words (2019).txt") |> String.split("\r\n")
    legal_words = Enum.filter(words, fn word -> TABIND.is_legal_word?(word) end)

    legal_words |> Enum.join(", ") |> IO.puts
    "Length of Legal Words: #{Kernel.length(legal_words)}" |> IO.puts
  end
end

TABIND.all_legal_words()
