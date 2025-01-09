;; This provides some useful functions (append, reverse, mapcar)
(load "builtin.l")

(print (append '(1 2 3) '(4 5 6)))
;; (1 2 3 4 5 6)

;; You can even write mergesort!
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

(defun split (L)
  (cond
    ((null L) (cons '() '()))
    (t (cons (cons (car L) (car (split (cdr (cdr L))))) (cons (car (cdr L)) (cdr (split (cdr (cdr L)))))))))

(defun mergesort (L)
  (cond
    ((null L) L)
    ((null (cdr L)) L)
    (t (merge (mergesort (car (split L))) (mergesort (cdr (split L)))))))

;; A reverse sorted list
(print (reverse (mergesort '(1 3 5 2 4 6))))