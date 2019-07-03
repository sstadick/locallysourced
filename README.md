# locallysourced

A repo for home grown scripts for little everyday problems. 


## Structure

Things that need to compile, or other require their own project dirs, can be found in src.
One off scripts just live in src/scripts
A make file might be used to make a bin dir or something. TBD

## Build

In top level, just run

```
make
```

## Deps
- cargo / rust toolchain

## Tools

### transpose

Flip a file from rows X columns to columns X rows.  Ex:

|head1|head2|head3|       
|-----|-----|-----|         
|val1|val2|val3|     
|val5|val6|val7| 

to:

|head1|val1|val5|
|-----|----|----| 
|head2|val2|val6|
|head3|val3|val7|
