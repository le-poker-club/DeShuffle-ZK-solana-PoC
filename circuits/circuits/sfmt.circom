pragma circom 2.0.2;

include "../node_modules/circomlib/circuits/bitify.circom";

template Lshift128(shift) {
    signal input ain[4];
    signal output r[4];

    var N = 32;

    component ainBits[4];
    for (var i = 0; i < 4; i++) {
        ainBits[i] = Num2Bits(N);
        ainBits[i].in <== ain[i];
    }

    signal totalBits[128];
    for (var i = 0; i < 128; i++) {
        if (i < 32) {
            totalBits[i] <== ainBits[0].out[i];
        } else if (i < 64) {
            totalBits[i] <== ainBits[1].out[i - 32];
        } else if (i < 96) {
            totalBits[i] <== ainBits[2].out[i - 64];
        } else {
            totalBits[i] <== ainBits[3].out[i - 96];
        }
    }

    signal shiftedBits[128];
    var bound = shift * 8;
    for (var i = 0; i < 128; i++) {
        if (i < bound) {
            shiftedBits[i] <== 0;
            shiftedBits[i] <-- totalBits[i - bound];
        }
    }

    component shiftedAout[4];
    for (var i = 0; i < 4; i++) {
        shiftedAout[i] = Bits2Num(N);
        for (var j = 0; j < N; j++) {
            shiftedAout[i].in[j] <-- shiftedBits[i * 32 + j];
        }
        r[i] <== shiftedAout[i].out;
    }
}

template Rshift128(shift) {
    signal input ain[4];
    signal output r[4];

    var N = 32;

    component ainBits[4];
    for (var i = 0; i < 4; i++) {
        ainBits[i] = Num2Bits(N);
        ainBits[i].in <== ain[i];
    }

    signal totalBits[128];
    for (var i = 0; i < 128; i++) {
        if (i < 32) {
            totalBits[i] <== ainBits[0].out[i];
        } else if (i < 64) {
            totalBits[i] <== ainBits[1].out[i - 32];
        } else if (i < 96) {
            totalBits[i] <== ainBits[2].out[i - 64];
        } else {
            totalBits[i] <== ainBits[3].out[i - 96];
        }
    }

    signal shiftedBits[128];
    for (var i = 0; i < 128; i++) {
        if (i < 128 - shift * 8) {
            shiftedBits[i] <== totalBits[i + shift * 8];
        } else {
            shiftedBits[i] <== 0;
        }
    }

    signal tempBits[128];
    for (var i = 0; i < 128; i++) {
        if (i >= 64) {
            tempBits[i] <== shiftedBits[i];
        } else {
            tempBits[i] <-- shiftedBits[i] | (totalBits[i + 64] << (64 - shift * 8));
        }
    }

    component shiftedAout[4];
    for (var i = 0; i < 4; i++) {
        shiftedAout[i] = Bits2Num(N);
        for (var j = 0; j < N; j++) {
            shiftedAout[i].in[j] <-- tempBits[i * 32 + j];
        }
        r[i] <== shiftedAout[i].out;
    }
}

template DoRecursion() {
    signal input a[4];
    signal input b[4];
    signal input r1[4];
    signal input r2[4];
    signal x[4];
    signal y[4];
    signal output r[4];

    var SFMT_SL1 = 11;
    var SFMT_SL2 = 3;
    var SFMT_SR1 = 10;
    var SFMT_SR2 = 1;
    var SFMT_MSK1 = 3220684791; // 0xbff7bff7
    var SFMT_MSK2 = 3221225471; // 0xbfffffff
    var SFMT_MSK3 = 3221224063; // 0xbffffa7f
    var SFMT_MSK4 = 4292738043; // 0xffddfbfb

    component lshift128 = Lshift128(SFMT_SL2);
    lshift128.ain <== a;
    x <== lshift128.r;

    component rshift128 = Rshift128(SFMT_SR2);
    rshift128.ain <== r1;
    y <== rshift128.r;

    r[0] <-- int32(a[0] ^ x[0] ^ ((b[0] >> SFMT_SR1) & SFMT_MSK1) ^ y[0] ^ (r2[0] << SFMT_SL1));
    r[1] <-- int32(a[1] ^ x[1] ^ ((b[1] >> SFMT_SR1) & SFMT_MSK2) ^ y[1] ^ (r2[1] << SFMT_SL1));
    r[2] <-- int32(a[2] ^ x[2] ^ ((b[2] >> SFMT_SR1) & SFMT_MSK3) ^ y[2] ^ (r2[2] << SFMT_SL1));
    r[3] <-- int32(a[3] ^ x[3] ^ ((b[3] >> SFMT_SR1) & SFMT_MSK4) ^ y[3] ^ (r2[3] << SFMT_SL1));
}


function int32 (y) {
    return y % 4294967296;
}

template SFMT(number) {
    var SFMT_N = 1689;
    var SFMT_N32 = SFMT_N * 4;
    var SFMT_POS1 = 627;
    var SFMT_PARITY1 = 4160749569; // 0xf8000001
    var SFMT_PARITY2 = 2313684745; // 0x89e80709
    var SFMT_PARITY3 = 1003664971; // 0x3bd2b64b
    var SFMT_PARITY4 = 207925732;  // 0x0c64b1e4

    signal input seed;
    signal output randomNumber[number+1];

    // Initialize state
    var state[SFMT_N32];
    state[0] = seed;

    // Initialize rest of the state
    for (var i = 1; i < SFMT_N32; i++) {
        var y = 1812433253 * (state[i-1] ^ (state[i-1] >> 30)) + i;
        state[i] = int32(y);
    }

    // period certification
    var inner = 0;
    var partity[4];
    partity[0] = SFMT_PARITY1;
    partity[1] = SFMT_PARITY2;
    partity[2] = SFMT_PARITY3;
    partity[3] = SFMT_PARITY4;

    for (var i = 0; i < 4; i++) {
        inner = int32(inner ^ state[i] & partity[i]);
    }
    inner = int32(inner ^ (inner >> 16));
    inner = int32(inner ^ (inner >> 8));
    inner = int32(inner ^ (inner >> 4));
    inner = int32(inner ^ (inner >> 2));
    inner = int32(inner ^ (inner >> 1));
    inner = int32(inner & 1);

    var flag = 0;
    if (inner != 1) {
        for (var i = 0; i < 4; i++) {
            var work = 1;
            for (var j = 0; j < 32; j++) {
                if (flag == 0 && int32(work & partity[i]) != 0) {
                    state[i] = int32(state[i] ^ work);
                    flag = 1;
                }
                work = int32(work << 1);
            }
        }
    }

    var r1[4];
    var r2[4];

    for (var i = 0; i < 4; i++) {
        r1[i] = state[(SFMT_N - 2) * 4 + i];
        r2[i] = state[(SFMT_N - 1) * 4 + i];
    }

    var a[4];
    var b[4];

    var n = number/4+1;
    component doRecursions[n];
    for (var i = 0; i < n; i++) {
        for (var k = 0; k < 4; k++) {
            a[k] = state[i * 4 + k];
            b[k] = state[(i + SFMT_POS1) * 4 + k];
        }
        doRecursions[i] = DoRecursion();
        doRecursions[i].a <-- a;
        doRecursions[i].b <-- b;
        doRecursions[i].r1 <-- r1;
        doRecursions[i].r2 <-- r2;

        for (var k = 0; k < 4; k++) {
            state[i * 4 + k] = doRecursions[i].r[k];
        }

        r1 = r2;
        for (var k = 0; k < 4; k++) {
            r2[k] = state[i * 4 + k];
        }
    }

    // handle result
    for (var i = 0; i < (number+1); i++) {
        randomNumber[i] <-- state[i];
    }
}
