defmodule TABIND do
  @moduledoc """
    Description: Given a word list of scrabble words,
    find all the words containing letters in the string "TABIND" that only appear once.

    In this process, the txt file will be read. Then, it will be split using ~r"(\r\n|\r|\n)"
    because of how txt files are formatted differently on machines. Lastly, check every word in
    the txt file to see if it matches the constraints above.

    To do the constraint above, we will transform the letters inside of the word into a set
    because of the property that there is no duplicates.
    Ex. ABA = {"A", "B"}

    After that we will perform an intersection between the word_letters set and the tabind_letters set.
    Ex. ABA = {"A", "B"} ∩ {"T","A","B","I","N","D"} = {"A","B"}

    Once we do the intersection, we can compare the length of the the intersection to the len of the word
    and see that they do not match.
    Ex. len({"A","B"}) = 2, len("ABA") = 3.

    Using Enum.filter, we will use this comparison for the last statement in the function to either keep or
    remove the word.
  """
  def is_legal_word?(word) do
    word_letters = MapSet.new(String.graphemes(word))
    tabind_letters = MapSet.new(["T","A","B","I","N","D"])
    intersection = MapSet.intersection(word_letters, tabind_letters)
    MapSet.size(intersection) == String.length(word)
  end

  def all_legal_words do
    words = File.read!("../Collins_Scrabble_Words_2019.txt") |> String.split(~r"(\r\n|\r|\n)")
    legal_words = Enum.filter(words, fn word -> TABIND.is_legal_word?(word) end)

    legal_words |> Enum.join(", ") |> IO.puts
    "Length of Legal Words: #{Kernel.length(legal_words)}" |> IO.puts
  end
end

TABIND.all_legal_words()
