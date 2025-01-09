# Lisp Interpreter
This is a fun Lisp interpreter in Rust. The interpreter has support for the pure Lisp functions below. Most other functions can be implemented directly in Lisp.

The supported functions are:
quote, car, cdr, cons, print, atom, listp, setq, defun, cond, eq, eval, equal, \+, \-, \*, \/, mod, floor, apply, load, and, \<=, \>=, \>, \<

There is also a `builtin.l` file you can load that provides some useful functions (append, reverse, mapcar).

# Example
Here's a fun mergesort example you can run (test.l):

```lisp
;; This provides some useful functions (append, reverse, mapcar)
(load "builtin.l")

(print (append '(1 2 3) '(4 5 6)))
;; (1 2 3 4 5 6)

;; Merge two sorted lists
(defun merge (L1 L2)
    (cond
        ( (null L1) L2 )
        ( (null L2) L1 )
        ( (null (car L1)) (merge (cdr L1) L2) )
        ( (null (car L2)) (merge L1 (cdr L2)) )
        ( (< (car L1) (car L2)) (cons (car L1) (merge (cdr L1) L2)) )
        ( T (cons (car L2) (merge L1 (cdr L2))) )
    )
)

;; Split a list into two halves
(defun split (L)
  (cond
    ((null L) (cons '() '()))
    (t (cons (cons (car L) (car (split (cdr (cdr L))))) (cons (car (cdr L)) (cdr (split (cdr (cdr L)))))))))

;; Do the divide and conquer
(defun mergesort (L)
  (cond
    ((null L) L)
    ((null (cdr L)) L)
    (t (merge (mergesort (car (split L))) (mergesort (cdr (split L)))))))

;; A reverse sorted list
(print (reverse (mergesort '(1 3 5 2 4 6))))
;; (6 5 4 3 2 1)
```

# Usage
Assuming you have Rust/Cargo installed, you can run the interpreter with a script file:
```
cargo run test.l
```
or run the interactive mode like below (note that the interactive mode doesn't support multi-line expressions).
```
cargo run
```

If you want to run the provided test cases, you can run:
```
cargo test
```