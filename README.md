```
                   __    _____  _  _ 
                  (  )  (  _  )( \/ )
  /\-/\            )(__  )(_)(  )  ( 
 /a a  \          (____)(_____)(_/\_)  _
=\ Y  =/-~~~~~~-,_____________________/ )
  '^--'          ______________________/
    \           /
    ||  |---'\  \   
   (_(__|   ((__| 
```

A tree-walk interpreter for a dialect of the
[Lox](http://craftinginterpreters.com/the-lox-language.html) programming
language, as described in part two of Bob Nystrom's excellent book [Crafting
Interpreters](https://http://craftinginterpreters.com/). Implemented in
[rust](https://www.rust-lang.org/en-US/). The code is somewhat rough, this is
my first rust project. It's also severely under-documented, but then again
one can always read the [book](https://http://craftinginterpreters.com/) 😉.

This is not and was never intended to be an industrial strength language
implementation, it's a hobby project implemented for fun and learning. Just
in case it's not obvious—don't use this for anything you want to work well.

## Running

### Repl
```sh
cargo run --features="cli"
```

### Repl In Debug Mode
For debugging the language implementation. Example :
```
> print(1+1);
Tokens ----
Ident("print")
LeftParentheses
Number(1.0)
Plus
Number(1.0)
RightParentheses
Semicolon
AST ----
(Expression Statement (Call (Variable Ident("print")) (Binary Plus (Literal Number(1.0)) (Literal Number(1.0)))))
Output ----
2
```

```sh
cargo run --features="cli" debug
```

### Running A Program From A File

```sh
cargo run --features="cli" fixtures/fibonacci.cbox
```

### Running A Program From A File In Debug Mode 

```sh
cargo run --features="cli" debug fixtures/fibonacci.cbox
```

## Examples

### Input: fixtures/fibonacci.cbox
```js
fn fib(n) {
  let i = 0;
  let j = 1;
  
  for (let c = 0; c < n; c = c + 1) {
    let temp = i + j;
    i = j;
    j = temp;
  }
  return i;
}

fn r_fib(n) {
  if (n <= 1) return n;
  return r_fib(n - 2) + r_fib(n - 1);
}

// ============

let n = 13;

let tick = clock();
for (let i = 0; i < n; i = i + 1) {
  print(fib(i));
}
let tock = clock();

print("non recursive fibonacci completed in " + (tock - tick) + " (s)");

tick = clock();
for (let i = 0; i < n; i = i + 1) {
  print(r_fib(i));
}
tock = clock();

print("recursive fibonacci completed in " + (tock - tick) + " (s)");

```

### Output: fixtures/fibonacci.cbox
```sh
0
1
1
2
3
5
8
13
21
34
55
89
144
"non recursive fibonacci completed in 0 (s)"
0
1
1
2
3
5
8
13
21
34
55
89
144
"recursive fibonacci completed in 0 (s)"
```

### Input: fixtures/classes_7.cbox
```js
class A {
  method() {
    print("A method");
  }
}

class B < A {
  method() {
    print("B method");
  }

  test() {
    super.method();
  }
}

class C < B {}

C().test();
```

### Output: fixtures/classes_7.cbox
```sh
"A method"
```