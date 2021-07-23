//shrsi t,n,k-1 Form the integer
//shri t,t,32-k 2**k – 1 if n < 0, else 0.
//add t,n,t     Add it to n,
//shrsi q,t,k   and shift right (signed).
//bge n,label      Branch if n >= 0.
//addi n,n,2**k-1  Add 2**k - 1 to n,
//shrsi n,n,k      and shift right (signed).