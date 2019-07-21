class Solution:
    def uniqueMorseRepresentations(self, words):
        self.morse_map = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
        self.letter_map = {}
        for i in range(26):
            self.letter_map[chr(i+97)] = i
        ascii_words = []
        for i in words:
            ascii_words.append(self.mappingWord(i))
        # print(ascii_words) 
        num_of_unique_words = 0
        while(len(ascii_words) > 0):
            counter = len(ascii_words) - 1
            for i in range(len(ascii_words)):
                if(i == 0):
                    to_compare_word = ascii_words.pop(counter)
                else:
                    if(self.compareTwoWords(to_compare_word, ascii_words[counter - i])):
                        ascii_words.pop(counter - i)
            num_of_unique_words += 1
        return num_of_unique_words

    def mappingWord(self, word):
        ascii_to_return = ''
        for i in word:
            ascii_to_return += self.morse_map[(self.letter_map[i])]
        return ascii_to_return
    # checks if two words are the same or not, if they are the same, returns true, else false
    def compareTwoWords(self, word_1, word_2):
        if(len(word_1) == len(word_2)):
            for i in range(len(word_1)):
                if(word_1[i] != word_2[i]):
                    return False
            return True
        else:
            return False



a = Solution()
print(a.uniqueMorseRepresentations(["gin", "zen", "gig", "msg"]))