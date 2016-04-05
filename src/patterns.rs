
use utils::*;

#[derive(Copy, Clone)]
pub enum Token {E, S, Dhee, Dhes, Dhse, Dhss, Empty}

#[derive(Copy, Clone)]
pub enum HandshakePattern {N, X, K, NN, NK, NX, XN, XK, XX, XR, KN, KK, KX, IN, IK, IX, XXfallback}

use self::Token::*;
use self::HandshakePattern::*;

fn copy_tokens(input: &[Token], out: &mut [Token]) {
    for count in 0..input.len() {out[count] = input[count];}
}

pub fn resolve_handshake_pattern(
                            handshake_pattern: HandshakePattern,
                            name: &mut [u8], 
                            premsg_pattern_i: &mut [Token],
                            premsg_pattern_r: &mut [Token], 
                            msg_patterns: &mut [[Token; 10]; 10]) -> usize {
    match handshake_pattern {
        N => { 
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes], &mut msg_patterns[0]);
            copy_memory(b"N", name)
        },

        K => { 
            copy_tokens(&[S], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes, Dhss], &mut msg_patterns[0]);
            copy_memory(b"K", name)
        },

        X => { 
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes, S, Dhss], &mut msg_patterns[0]);
            copy_memory(b"X", name)
        },

        NN => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee], &mut msg_patterns[1]);
            copy_memory(b"NN", name)
        },

        NK => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee], &mut msg_patterns[1]);
            copy_memory(b"NK", name)
        },

        NX => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, S, Dhse], &mut msg_patterns[1]);
            copy_memory(b"NX", name)
        },

        XN => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee], &mut msg_patterns[1]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[2]);
            copy_memory(b"XN", name)
        },

        XK => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee], &mut msg_patterns[1]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[2]);
            copy_memory(b"XK", name)
        },

        XX => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, S, Dhse], &mut msg_patterns[1]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[2]);
            copy_memory(b"XX", name)
        },

        XR => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee], &mut msg_patterns[1]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[2]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[3]);
            copy_memory(b"XR", name)
        },

        KN => {
            copy_tokens(&[S], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes], &mut msg_patterns[1]);
            copy_memory(b"KN", name)
        }

        KK => {
            copy_tokens(&[S], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes, Dhss], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes], &mut msg_patterns[1]);
            copy_memory(b"KK", name)
        }

        KX => {
            copy_tokens(&[S], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes, S, Dhse], &mut msg_patterns[1]);
            copy_memory(b"KX", name)
        }

        IN => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E, S], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes], &mut msg_patterns[1]);
            copy_memory(b"IN", name)
        }

        IK => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[S], premsg_pattern_r);
            copy_tokens(&[E, Dhes, S, Dhss], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes], &mut msg_patterns[1]);
            copy_memory(b"IK", name)
        }

        IX => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[], premsg_pattern_r);
            copy_tokens(&[E, S], &mut msg_patterns[0]);
            copy_tokens(&[E, Dhee, Dhes, S, Dhse], &mut msg_patterns[1]);
            copy_memory(b"IX", name)
        }

        XXfallback => {
            copy_tokens(&[], premsg_pattern_i);
            copy_tokens(&[E], premsg_pattern_r);
            copy_tokens(&[E, Dhee, S, Dhse], &mut msg_patterns[0]);
            copy_tokens(&[S, Dhse], &mut msg_patterns[1]);
            copy_memory(b"XXfallback", name)
        }
    }
}
