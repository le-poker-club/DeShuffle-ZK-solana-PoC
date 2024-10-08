pragma circom 2.0.2;

include "./sfmt.circom";
include "../node_modules/circomlib/circuits/poseidon.circom";

template Shuffle(cardNum) {
    signal input seed;
    signal input seedHash;
    signal input numbers[cardNum];
    signal cardsResult[cardNum];
    signal compareCards[cardNum];

    // Verify seed hash
    component poseidon = Poseidon(1);
    poseidon.inputs[0] <== seed;
    poseidon.out === seedHash;

    // Generate random numbers using SFMT
    component sfmt = SFMT(cardNum);
    sfmt.seed <== seed;

    var deck[cardNum];
    for (var i = 0; i < cardNum; i++) {
      deck[i] = i;
    }

    for (var i = 0; i < cardNum; i++) {
      var j = (sfmt.randomNumber[i] % (cardNum - i)) + i;
      var temp = deck[i];
      deck[i] = deck[j];
      deck[j] = temp;
    }

    var randomCut = sfmt.randomNumber[cardNum] % cardNum;
    for (var i = 0; i < cardNum; i++) {
      cardsResult[i] <-- i < cardNum - randomCut ? deck[i + randomCut] : deck[i - (cardNum - randomCut)];
    }
    
    // check if the numbers are the same
    for (var i = 0; i < cardNum; i++) {
        var temp = numbers[i] == 0 ? 0 : cardsResult[i]+1;
        compareCards[i] <-- temp;
        compareCards[i] === numbers[i];
    }
}

component main { public [numbers, seedHash] } = Shuffle(52);