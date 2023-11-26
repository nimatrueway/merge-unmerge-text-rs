tart-rs
=======
tart-rs (tar text written in Rust) is a small tool to aggregate a bunch of utf-8 text files into one big text file.
Pretty much like good old `tar` but the output will be in utf-8 readable format, and the content of input files will be
copied line by line to the aggregated output.

Example:

```
file1.java
------------------------------------
System.out.println("Hello World 1")
------------------------------------


file2.scala
------------------------------------
println("Hello World 1")
------------------------------------

sh$ tart-rs --output=merged.txt *.java *.scala
sh$ tart-rs --output=merged.txt $(fd *.java *.scala)

output.txt
------------------------------------
// [tart-rs:file:start] ******************************** // file1.java
System.out.println("Hello World 1")
// [tart-rs:file:end]   ******************************** //
// [tart-rs:file:start] ******************************** // file2.scala
println("Hello World 1")
// [tart-rs:file:end]   ******************************** //
------------------------------------
```

Keywords
=======
utf-8 text file merger unmerged, utf8 text file imploder exploder
