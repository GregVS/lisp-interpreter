(defun LENGTH (L)
    (COND
        ( (NULL L) 0 )
        ( T (+ 1 (LENGTH (CDR L))) )
    )
)

(defun MEMQ (A L)
    (COND
        ( (NULL L) NIL )
        ( (EQ A (CAR L)) L )
        ( T (MEMQ A (CDR L)) )
    )
)

(defun APPEND (L1 L2)
    (COND
        ( (NULL L1) L2 )
        ( T (CONS (CAR L1) (APPEND (CDR L1) L2)) )
    )
)

(defun REVERSE (L)
    (COND
        ( (NULL L) NIL )
        ( T (APPEND (REVERSE (CDR L)) (CONS (CAR L) NIL)) )
    )
)

(defun MAPCAR (F L)
    (COND
        ( (NULL L) NIL )
        ( T (CONS (APPLY F (CONS (CAR L) NIL)) (MAPCAR F (CDR L))) )
    )
)

(defun COPY (L)
    (COND
        ( (ATOM L) L )
        ( T (CONS (COPY (CAR L)) (COPY (CDR L))) )
    )
)

(defun NTH (L N)
    (COND
        ( (EQ '0 N) (CONS NIL L) )
        ( (EQ '1 N) L )
        ( T (NTH (CDR L) (- N 1)) )
    )
)

(defun REMOVE (X L)
    (COND
        ( (NULL L) NIL )
        ( (EQUAL X (CAR L)) (REMOVE X (CDR L)) )
        ( T (CONS (CAR L) (REMOVE X (CDR L))) )
    )
)

(defun SUBST (X Y Z)
    (COND
        ( (ATOM Z) (COND ((EQUAL Z X) Y) (T Z) ) )
        ( T (CONS (SUBST X Y (CAR Z)) (SUBST X Y (CDR Z)) ) )
    )
)

(defun SUBSET (FN L)
    (COND
        ( (NULL L) NIL )
        ( (EQUAL NIL (APPLY FN (CONS (CAR L) NIL))) (SUBSET FN (CDR L)) )
        ( T (CONS (CAR L) (SUBSET FN (CDR L))) )
    )
)

(defun SUBLIST (L1 L2)
    (COND
        ( (NULL L1) T )
        ( (NULL L2) NIL )
        ( (PREFIX-LIST L1 L2) T )
        ( T (SUBLIST L1 (CDR L2)))
    )
)
