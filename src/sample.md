---
author: test 
coil: 
  options: 
    keep_indention: true 
  files: 
    Alias1: ./path/to/file/file.cs 
    alias2: ./path/to/file/file2.txt
title: unaffect tag 
---


Normal text that should not be parsed!

Test code is not verified to be actual C# code. It shouldn't have to be.
```c# coil.files.Alias1
    Method(){
      Console.writeline("test");
    }
```

```
Code block with no meta data
```

Normal text
`An inline that should also be ignored`

```js coil.files.alias2 some,other.meta
  Hightlighting doesn't have to match the file type.
  A didifferent set of hightlighting
```

Normal text `with inline` this should be handled nicely.

``` coil: { options.keep-indention: false, files.target: "./path/to/file/file3.py" }
  This should not be indented
    this should not be indented
```

This should also be ignored
