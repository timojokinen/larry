use rand::Rng;

use crate::{attacks::*, util::*};

/// 
pub const SLIDER_TABLE_SIZE: usize = 107648;

#[derive(Debug, Copy, Clone)]
pub struct Magic {
    pub(super) magic: u64,
    pub(super) mask: Bitboard,
    pub(super) shift: u8,
    pub(super) offset: usize,
}

/// Calculates a magic number
pub fn init_magic(
    square: u8,
    bishop: bool,
    offset: &mut usize,
    table: &mut [Bitboard; SLIDER_TABLE_SIZE],
) -> Magic {
    let slider_attacks = if bishop {
        attacks::bishop_att
    } else {
        attacks::rook_att
    };
    let attack_mask = slider_attacks(square, 0) & !edges(square);

    let relevant_bits = attack_mask.count_ones() as u8;
    let permutations_count: usize = 1 << relevant_bits;
    let shift = 64 - relevant_bits;

    let mut blocker_configurations = vec![0u64; permutations_count];
    let mut attack_table = vec![0u64; permutations_count];

    enumerate_subsets(attack_mask, |subset, idx| {
        blocker_configurations[idx] = subset;
        attack_table[idx] = slider_attacks(square, subset);
    });

    let mut magic = (if bishop { BISHOP_MAGICS } else { ROOK_MAGICS })[square as usize];
    let mut rng = rand::thread_rng();

    loop {
        if magic == 0 {
            let candidate = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
            if !verify_candidate(candidate, attack_mask) {
                continue;
            }
            magic = candidate;
        }

        let mut failure = false;
        let mut used_attacks = vec![0u64; permutations_count];

        for idx in 0..permutations_count {
            let hash_index = calculate_hash_index(magic, blocker_configurations[idx], shift);

            if used_attacks[hash_index] == 0 {
                used_attacks[hash_index] = attack_table[idx];
            } else if used_attacks[hash_index] != attack_table[idx] {
                failure = true;
                break;
            }
        }

        if !failure {
            let magic = Magic {
                magic,
                mask: attack_mask,
                shift,
                offset: offset.clone(),
            };

            for idx in 0..permutations_count {
                table[*offset + idx] = used_attacks[idx];
            }

            *offset += permutations_count;
            return magic;
        } else {
            magic = 0;
        }
    }
}

/// Uses a magic number to calculate the index for the given occupancy in the precomputed table
pub fn calculate_hash_index(magic: u64, occupancy: Bitboard, shift: u8) -> usize {
    (((occupancy.wrapping_mul(magic)) >> shift) as u64) as usize
}

/// Verifies if a magic number candidate is suitable by checking if the multiplication
/// of the attack mask and the magic number results in at least 6 set bits in the upper byte.
pub fn verify_candidate(magic: u64, attack_mask: Bitboard) -> bool {
    (attack_mask.wrapping_mul(magic) & 0xff00000000000000).count_ones() >= 6
}

/// Uses the Carry-Rippler trick to enumerate all different subset possibilities of a given binary number.
/// Formula: (n - d) & d
pub fn enumerate_subsets<F>(bitboard: Bitboard, mut func: F)
where
    F: FnMut(u64, usize),
{
    let mut subset: u64 = 0;
    let mut idx: usize = 0;
    loop {
        subset = subset.wrapping_sub(bitboard) & bitboard;
        func(subset, idx);
        idx += 1;

        if subset == 0 {
            break;
        }
    }
}

/// Pre-calculated magic numbers for bishops to speed up initialising attack tables
pub static BISHOP_MAGICS: [u64; 64] = [
    18041904302786592,
    4620698732480989544,
    40532948567719936,
    288804329828123648,
    11530359381048688640,
    11052116097647247360,
    2306696256646287360,
    1157473491470059520,
    162134023556104704,
    932391245910528,
    614745764698015746,
    11682412217646448652,
    2306125657790522368,
    4616261663719424004,
    1369094724844011520,
    276019808288,
    9268452051308857376,
    2255167340707976,
    4769453052113469954,
    9230132386279342083,
    73183502553650380,
    140741917622274,
    9232538674455977985,
    4611756939079864320,
    4613973071510896913,
    73185695187862529,
    6775191254426114,
    1134696001970240,
    9241667979144347648,
    9300076172417696268,
    1153062800729575424,
    81637106286168064,
    19159024474071104,
    4630264811077505024,
    180302624007653380,
    4615068133374820640,
    563516906799136,
    5066828820906112,
    577052298174529731,
    4611866477930037768,
    11619595185607950336,
    2306979915242734080,
    11029543158272,
    11530983344342829056,
    162448514754839552,
    9042453428617536,
    2342436990139572480,
    1445657825510424608,
    11547515661265930372,
    285875240183808,
    426894382334500,
    9009402640664768,
    3602879851215978504,
    10379112826781369344,
    4922438826204602624,
    2330999149184128,
    9029228276745221,
    92930899159031808,
    2323857961845608480,
    7207033188017897984,
    36310340782588928,
    18295895229628928,
    11540544423311180048,
    577026244912120065,
];

/// Pre-calculated magic numbers for rooks to speed up initialising attack tables
pub static ROOK_MAGICS: [u64; 64] = [
    9259401384170618904,
    2684154449420619777,
    1188959648011522049,
    72063092133466112,
    144117387369646084,
    144132832762528256,
    1441152983558654000,
    36029346779408640,
    3518473720299552,
    9368683507004473412,
    73324368873128064,
    900861212986705920,
    145381842659313666,
    281492173357576,
    2308236096556761600,
    1155454788159574272,
    4926361855213568,
    94580265101295682,
    4332465590578020384,
    72096077095899138,
    9512728862736777346,
    141287277724672,
    5769115520978815041,
    5782632917066875905,
    70370891694112,
    72092779484807232,
    35186527965184,
    720595740182710304,
    290490974206165632,
    562967200403488,
    36319085278003777,
    2252083283656964,
    10203555440886912,
    72128031505784836,
    4611704728395456513,
    141905744631808,
    79167026627584,
    39547242826171392,
    306253576190054434,
    3694926038564929,
    9376564795079163908,
    9095160722374656,
    141012668383264,
    864708724938309640,
    9223935330542288928,
    288234774231810176,
    13979454795644731396,
    3096516810113025,
    1154188147421111552,
    4684376948896597248,
    54113635240388096,
    17596483111168,
    2533825622052866,
    9512165536101081600,
    281483566907648,
    704513115648,
    563229127713410,
    36065081976455701,
    12691178939671449859,
    598173248652545,
    563018740539394,
    77124147986045026,
    72058762302621700,
    11547229584232611906,
];
