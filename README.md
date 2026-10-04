<h1>Carb</h1>

# Syntax

## Variables
<p> let a = 10;     

creates a variable which`s value can be modified
<p>
<p> letf a = 10;     

creates a variable which`s value cannot be modified. It has to have a value

**letf a;** will break the program, beacause the values is `null`


---
{}   <- its called a `block`.
`Block`s are used for separating code into differend segments.  
## For Example

letf a = 10;

{

  print(a); 
  
}

**output:** 10, beacause the variable `a` is stored in the parent block.


### Another Example

{

letf a = 10;

}

  print(a); 

  **output** null, beacause the variable `a` is stored in the block, which is a child of block, which calls `print(a)`
