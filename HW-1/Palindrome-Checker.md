# Palindrome Checker - HW. #1

### Define Logic:
- Palindrome is a word, number, phrase, or other sequence of characters which reads the same backward as forward, such as madam, racecar, also in Thai; กาก, ยาย, งง, นอน, นาน, etc.
- Write a function called palindrome = string, <str>. If str is a palindrome, return true, otherwise return false
- Ignoring punctuation (non-Alphanumeric Characters) like commas, periods, question marks, exclamation points, and casing
Equalize all casing 
Do  lower case everything, every words
Compare String to Its Reverse

### Execution Steps:
Input the word that need to do “Palindrome Checker”
Define function to set all  Alphanumeric to Lowercase
Keep all Lowercase version to another one argument
Define function to match Alphanumeric character and finding as below by using regex pattern 
`[a-z0-9ก-๛]` 

to define matching character
	Note : This step can also ignore punctuation.