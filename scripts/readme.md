# Scripts

## Formatting function names

```bash
perl -pi -e 's/(pub fn )([A-Za-z][a-zA-Z0-9_]*)(?=\()/$1\L$2/g; s/(?<=\w)__+(?=\w)/_/g' src/icons/*.rs
```
