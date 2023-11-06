use lambdaworks_math::elliptic_curve::traits::IsPairing;
use lambdaworks_math::{    
    elliptic_curve::
        traits::IsEllipticCurve,
    cyclic_group::IsGroup,
    elliptic_curve::
        short_weierstrass::curves::bls12_381::{    
            pairing::BLS12381AtePairing,        
            curve::BLS12381Curve,
            twist::BLS12381TwistCurve},
};

fn main() {    

    // Generators:

    let _g_1 = BLS12381Curve::generator();
    let _g_2 = BLS12381TwistCurve::generator();

    // Private Key:

    let sk: u64 = 45; // It must be fixed

    // Public Key:

    let pk = _g_1.operate_with_self(sk);

    // Message:

    let msg: u64 = 5456456;
    let hash = _g_2.operate_with_self(msg); // This is not a hash. It must be fixed

    // Signature:

    let _signature = hash.operate_with_self(sk);

    // Verification:

    let p_1 = <BLS12381AtePairing as IsPairing>::compute(&_g_1, &_signature);
    let p_2 = <BLS12381AtePairing as IsPairing>::compute(&pk, &hash);

    if p_1 == p_2 {
        println!("Verified!");
    }

}
