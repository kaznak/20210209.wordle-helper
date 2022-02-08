# wordle helper

[wordle](https://www.powerlanguage.co.uk/wordle/) helper

## usage

```
$ bin/wordlist s 0 a 0 l 0 e 0 t 0 c 0 r 0 o 0 n 0 y 0            
s 0 a 0 l 0 e 0 t 0
c 0 r 0 o 0 n 0 y 0
humid
humph
vivid
whiff
$ bin/wordlist s 0 a 0 l 0 e 0 t 0 c 0 r 0 o 0 n 0 y 0 | bin/stats
s 0 a 0 l 0 e 0 t 0
c 0 r 0 o 0 n 0 y 0
4
0.750000 h
0.750000 i
0.500000 d
0.500000 m
0.500000 u
0.250000 f
0.250000 p
0.250000 v
0.250000 w
0.50000 h 0.50000 u 0.50000 m 0.50000 i 0.50000 d
0.25000 v 0.25000 h 0.25000 i 0.25000 f 0.25000 f
0.25000 w 0.25000 i 0.25000 v 0.25000 p 0.25000 h
$  % bin/wordlist s 0 a 0 l 0 e 0 t 0 c 0 r 0 o 0 n 0 y 0 h 1 u 0 m 0 i 1 d 0
s 0 a 0 l 0 e 0 t 0
c 0 r 0 o 0 n 0 y 0
h 1 u 0 m 0 i 1 d 0
whiff
```

## reference

- [the site](https://freshman.dev/wordle)
- [the answer](https://gist.github.com/cfreshman/a03ef2cba789d8cf00c08f767e0fad7b)
