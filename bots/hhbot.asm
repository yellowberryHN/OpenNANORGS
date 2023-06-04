info: hhbot-01, Heinz Hemken
  
main:
    // select a random direction and distance to move
        rand    [dir], 4

//------------------------------------------------------------------------------
// main loop
loop:
        // check if I�m top of food and eat if so
        sense   r2
        cmp     r2, 32
        jg      noFood
        eat

noFood:

        // see if we're over a collection point and
        // release some energy
        energy  r0
        call    bigRelease
        call    medRelease
        call    smallRelease
        
        // move me
        call    newDir
        travel  [dir]

        jmp     loop
//------------------------------------------------------------------------------
// subroutine-like code

newDir:
        rand    [dir], 4
        ret
        
hugeRelease:
        cmp     r0, 30000
        sense   r5
        cmp     r5, 0xFFFF      // are we on a collection point?
        jne     hugeReleaseDone
        release 28000
        ret
hugeReleaseDone:
        ret
        
bigRelease:
        cmp     r0, 10000
        sense   r5
        cmp     r5, 0xFFFF      // are we on a collection point?
        jne     bigReleaseDone
        release 8000
        ret
bigReleaseDone:
        ret
        
medRelease:
        cmp     r0, 5000
        sense   r5
        cmp     r5, 0xFFFF      // are we on a collection point?
        jne     medReleaseDone
        release 4000
        ret
medReleaseDone:
        ret
        
smallRelease:
        cmp     r0, 2500
        sense   r5
        cmp     r5, 0xFFFF      // are we on a collection point?
        jne     smallReleaseDone
        release 500
        ret
smallReleaseDone:
        ret
        
dir:                
        data { 0 }       // our initial direction