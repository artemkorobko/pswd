# Secret generator command line utility

Secret generator generates random secrets with configurable length and amount of segments to the standard output. Every password consists of lowercase, uppercase letters, digits, and special symbols. In case of multi-segment secrets, special symbols are not used and each segment is separated by `-` symbol. The amount of uppercase letters is around 10% and the amount of digits is around 5% of the total amount of symbols inside each segment.
