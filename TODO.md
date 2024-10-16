 -  parse cli parameters
 -  read from stdin
 -  split stdin into columns 
 -  handle different separator types
 - handle commands with args
   - detect if cmd has more than one arg
 -  map (execute) commands to fields
     -  pass full column to yarg
     - spawn one yarg cmd per row (xargs style)
 - print resulting concatenated columns

 - delimiter: use regex or literal char/str 
 - add delimiter shortcuts as args
 - handle input stream
    - no preloading of input
    - handle input as a stream

