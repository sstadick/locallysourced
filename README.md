# locallysourced

A repo for home grown scripts for little everyday problems. 


## Structure

Things that need to compile, or other require their own project dirs, can be found in src.
One off scripts just live in src/scripts
A make file might be used to make a bin dir or something. TBD

## Tools

### transpose

Flip a file from rows X columns to columns X rows.  Ex:

head1   head2   head3           head1   val1    val5
val1     val2    val3     =>    head2   val2    val6
val5     val6    val7           head3   val3    val7


