# Password generator command line utility

Password generator generates random password with configurable length and tokens to the standard output. Every password contains of lower case, upper case letters and digits. If special symbols required it can generate tokenized password by separating each token with `-` symbol. The amount of upper case letters is 10% and 5% of digits depending on the password length.

## Examples

Generate password with default options (3 tokens with size of 10 symbols in each token):
```bash
> ./pswd
GeriwePuTo-fa1uvilice-m3pogaoruh
```

Generate 10 symbols password with single token:
```bash
> ./pswd -t 1
qoMusi4simIh
```
Generate 20 symbols password with single token:
```bash
> ./pswd -t 1 -l l
WulegumookeK7qubu5aaho
```
