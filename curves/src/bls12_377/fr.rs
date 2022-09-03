// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_fields::{
    FftParameters,
    FieldParameters,
    Fp256,
    Fp256Parameters,
    PoseidonDefaultParameters,
    PoseidonDefaultParametersEntry,
};
use snarkvm_utilities::biginteger::BigInteger256 as BigInteger;

/// BLS12-377 scalar field.
///
/// Roots of unity computed from modulus and R using this sage code:
///
/// ```ignore
/// q = 8444461749428370424248824938781546531375899335154063827935233455917409239041
/// R = 6014086494747379908336260804527802945383293308637734276299549080986809532403 # Montgomery R
/// s = 47
/// o = q - 1
/// F = GF(q)
/// g = F.multiplicative_generator()
/// assert g.multiplicative_order() == o
/// g2 = g ** (o/2**s)
/// assert g2.multiplicative_order() == 2**s
/// def into_chunks(val, width, n):
///     return [int(int(val) // (2 ** (width * i)) % 2 ** width) for i in range(n)]
/// print("Gen (g % q): ", g % q)
/// print("Gen (g * R % q): ", g * R % q)
/// print("Gen into_chunks(g * R % q): ", into_chunks(g * R % q, 64, 4))
/// print("2-adic gen (g2 % q): ", g2 % q)
/// print("2-adic gen (g2 * R % q): ", g2 * R % q)
/// print("2-adic gen into_chunks(g2 * R % q): ", into_chunks(g2 * R % q, 64, 4))
/// ```
pub type Fr = Fp256<FrParameters>;

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}

impl FftParameters for FrParameters {
    type BigInteger = BigInteger;

    #[rustfmt::skip]
    const POWERS_OF_G: &'static [BigInteger] = &[
        BigInteger([10328383819076220041, 14191591508846097093, 12501134559794337047, 86910565973793808]),
        BigInteger([8860509688463634423, 1987637956085362503, 13747388016533910673, 576114928986390863]),
        BigInteger([17832370440353462201, 18333106810824263321, 592081709184052627, 1124009604792067551]),
        BigInteger([1367222650021696822, 1672322518064107775, 10078960202391419581, 986487607474991126]),
        BigInteger([7801994457662355801, 1836364555646277854, 5564941731559901330, 137071134080593267]),
        BigInteger([1627872368810536665, 7561244056243702343, 7398253825307092333, 492579031612513421]),
        BigInteger([2174016681181728162, 10505838485506869491, 16610720311795037309, 1098422320772082154]),
        BigInteger([14337306730171611076, 12442514747166259275, 17248308783327290044, 823884278180763119]),
        BigInteger([15696010754873740657, 13141345562885611284, 9680036141109855441, 887505112900742727]),
        BigInteger([14117995369707848741, 2676544868778200817, 15381510150427907877, 257498018850372776]),
        BigInteger([8075389433991515579, 16916339557505807697, 11298360946530060764, 1122123836554287848]),
        BigInteger([10342888282698021604, 12403284485547111431, 14696235823030357764, 838681669075417381]),
        BigInteger([3300847203551844483, 3339409470471498824, 13654830630444755558, 707677572403360065]),
        BigInteger([3808737952027303658, 8968531274175594265, 8106831138537735386, 95940273114358519]),
        BigInteger([15286725442987323933, 2007106475361142923, 2083317292372149883, 180974583549619563]),
        BigInteger([4397199371149633379, 2642875832121143872, 10413799076104686281, 825642426606926497]),
        BigInteger([8774224414660521854, 18365829392473779644, 8655911966341339463, 684365345904928200]),
        BigInteger([10806033142992442180, 3075668854507278742, 8979520784635433375, 850542396786602444]),
        BigInteger([9597527489463497832, 2333761960726968730, 16746112055805176011, 720722047058011984]),
        BigInteger([11665299291901935400, 17182857005450655341, 4636328355928986639, 367772762952845550]),
        BigInteger([8069337352849774732, 6810387472871013908, 12557276942623517253, 1071767776870672597]),
        BigInteger([289374981952860296, 10127025435713098654, 2208729361928340935, 1150687475564240568]),
        BigInteger([15093304075973936383, 9776219015851785266, 11568887383937677886, 1006739033513003763]),
        BigInteger([1150486120695289571, 3791543346167510731, 8613032656732926571, 813839133560734466]),
        BigInteger([16839302334653569483, 7806350610692982781, 9943056458881994391, 929704394376454922]),
        BigInteger([10144789091081869880, 5276676660904173699, 11321786778118072766, 828040235651688951]),
        BigInteger([14026679459268555543, 17614867463469069595, 3844728541758442592, 1281214418221255993]),
        BigInteger([5610429035363378761, 5506683230076607326, 15435736246316713183, 630199525257939000]),
        BigInteger([2091185604097662362, 17127246870944143349, 5486761434399919491, 1279919528069553426]),
        BigInteger([13424026958577531300, 15479473384426506037, 17001359710774047057, 625354898110673901]),
        BigInteger([13755737776659122100, 7812729316127440458, 9523938547420537112, 648137986191484461]),
        BigInteger([1165812272695901547, 12477727142090267391, 2985602208663590731, 1045963240577762725]),
        BigInteger([11386823563897388805, 8996428867051358375, 17407367155104854965, 902542683321490548]),
        BigInteger([14207807180461846907, 2977462516520395126, 8289854015104591870, 838036922954485384]),
        BigInteger([8702429858031186294, 9896991137985631698, 11452212551647075484, 1112923889600286310]),
        BigInteger([11868334807536044594, 11338331136471586738, 8983503108370834649, 144895927838607475]),
        BigInteger([14325615367965009586, 651112296219242760, 8607802202904885190, 487010021762658902]),
        BigInteger([14813624617275070509, 2882077349404717420, 5680560839221873456, 18496147629218790]),
        BigInteger([3860813008363005562, 13070289317452546120, 16177474974163429734, 905589341506001309]),
        BigInteger([10635157538079540293, 14558775325840713906, 6285603727816758609, 1072054636653083005]),
        BigInteger([14131499561750797534, 3603585501749045744, 6654910081877801363, 131121550043178146]),
        BigInteger([2308547058138155728, 14611290591686694966, 14498629415485764736, 399365686344451181]),
        BigInteger([216042469711036291, 14712225719724588293, 18327253858955787988, 1257556734790442820]),
        BigInteger([1623908135086529229, 13407327894380134160, 8948411873392033406, 1141416569028784275]),
        BigInteger([4618273137458433979, 9101316775941478578, 3857472798808877274, 314321949662526359]),
        BigInteger([8860621160618917888, 9963140610363752447, 4379532757234729984, 1345280370688173398]),
    ];
    #[rustfmt::skip]
    const TWO_ADICITY: u32 = 47;
    /// TWO_ADIC_ROOT_OF_UNITY = 8065159656716812877374967518403273466521432693661810619979959746626482506078
    /// Encoded in Montgomery form, the value is
    /// (8065159656716812877374967518403273466521432693661810619979959746626482506078 * R % q) =
    /// 7039866554349711480672062101017509031917008525101396696252683426045173093960
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        12646347781564978760u64,
        6783048705277173164u64,
        268534165941069093u64,
        1121515446318641358u64,
    ]);
}

impl FieldParameters for FrParameters {
    #[rustfmt::skip]
    const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    /// GENERATOR = 22
    /// Encoded in Montgomery form, so the value is
    /// (22 * R) % q = 5642976643016801619665363617888466827793962762719196659561577942948671127251
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        2984901390528151251u64,
        10561528701063790279u64,
        5476750214495080041u64,
        898978044469942640u64,
    ]);
    #[rustfmt::skip]
    const INV: u64 = 725501752471715839u64;
    /// MODULUS = 8444461749428370424248824938781546531375899335154063827935233455917409239041
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        725501752471715841u64,
        6461107452199829505u64,
        6968279316240510977u64,
        1345280370688173398u64,
    ]);
    #[rustfmt::skip]
    const MODULUS_BITS: u32 = 253;
    /// (r - 1)/2 =
    /// 4222230874714185212124412469390773265687949667577031913967616727958704619520
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x8508c00000000000,
        0xacd53b7f68000000,
        0x305a268f2e1bd800,
        0x955b2af4d1652ab,
    ]);
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        9015221291577245683u64,
        8239323489949974514u64,
        1646089257421115374u64,
        958099254763297437u64,
    ]);
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        2726216793283724667u64,
        14712177743343147295u64,
        12091039717619697043u64,
        81024008013859129u64,
    ]);
    #[rustfmt::skip]
    const REPR_SHAVE_BITS: u32 = 3;
    // T and T_MINUS_ONE_DIV_TWO, where r - 1 = 2^s * t

    /// t = (r - 1) / 2^s =
    /// 60001509534603559531609739528203892656505753216962260608619555
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0xedfda00000021423,
        0x9a3cb86f6002b354,
        0xcabd34594aacc168,
        0x2556,
    ]);
    /// (t - 1) / 2 =
    /// 30000754767301779765804869764101946328252876608481130304309777
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x76fed00000010a11,
        0x4d1e5c37b00159aa,
        0x655e9a2ca55660b4,
        0x12ab,
    ]);
}

impl PoseidonDefaultParameters for FrParameters {
    const PARAMS_OPT_FOR_CONSTRAINTS: [PoseidonDefaultParametersEntry; 7] = [
        PoseidonDefaultParametersEntry::new(2, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(3, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(4, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(5, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(6, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(7, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(8, 17, 8, 31, 0),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_fields::{FftField, Field, PrimeField};

    #[test]
    fn test_powers_of_g() {
        let two = Fr::from(2u8);

        // Compute the expected powers of G.
        let g = Fr::two_adic_root_of_unity().pow(FrParameters::T);
        let powers = (0..FrParameters::TWO_ADICITY - 1)
            .map(|i| g.pow(two.pow(Fr::from(i as u64).to_repr()).to_repr()).to_repr())
            .collect::<Vec<_>>();

        // Ensure the correct number of powers of G are present.
        assert_eq!(FrParameters::POWERS_OF_G.len() as u64, (FrParameters::TWO_ADICITY - 1) as u64);
        assert_eq!(FrParameters::POWERS_OF_G.len(), powers.len());

        // Ensure the expected and candidate powers match.
        for (expected, candidate) in powers.iter().zip(FrParameters::POWERS_OF_G.iter()) {
            println!("{:?} =?= {:?}", expected, candidate);
            assert_eq!(expected, candidate);
        }
    }
}
