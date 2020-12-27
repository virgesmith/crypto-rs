// use openssl::ec::{EcPoint, EcGroup};
// //use openssl::ecdsa::EcdsaSig;
// use openssl::bn::{BigNum, BigNumRef, BigNumContext};

// use std::error::Error;
// // int ec_GFp_simple_set_compressed_coordinates(const EC_GROUP *group,
// //                                              EC_POINT *point,
// //                                              const BIGNUM *x_, int y_bit,
// //                                              BN_CTX *ctx)
// pub fn ec_GFp_simple_set_compressed_coordinates(grp: &EcGroup, 
//                                                 point: &mut EcPoint, 
//                                                 x_: BigNum, 
//                                                 y_bit: bool, 
//                                                 mut ctx: &mut BigNumContext) 
//   -> Result<(), Box<dyn Error>>
// {
// //     BN_CTX *new_ctx = NULL;
// //     BIGNUM *tmp1, *tmp2, *x, *y;
// //     int ret = 0;

// // #ifndef FIPS_MODE
// //     /* clear error queue */
// //     ERR_clear_error();
// // #endif

// //     if (ctx == NULL) {
// //         ctx = new_ctx = BN_CTX_new_ex(group->libctx);
// //         if (ctx == NULL)
// //             return 0;
// //     }

// //     y_bit = (y_bit != 0);

// //     BN_CTX_start(ctx);
// //     tmp1 = BN_CTX_get(ctx);
// //     tmp2 = BN_CTX_get(ctx);
//   let mut tmp1 = BigNum::new()?;
//   let mut tmp2 = BigNum::new()?;
// //     x = BN_CTX_get(ctx);
// //     y = BN_CTX_get(ctx);
// let mut x = BigNum::new()?;
// let mut y = BigNum::new()?;
// //     if (y == NULL)
// //         goto err;

// //     /*-
// //      * Recover y.  We have a Weierstrass equation
// //      *     y^2 = x^3 + a*x + b,
// //      * so  y  is one of the square roots of  x^3 + a*x + b.
// //      */

// //     /* tmp1 := x^3 */
// //     if (!BN_nnmod(x, x_, group->field, ctx))
// //         goto err;
//   let xtmp = BigNum::new()?;
//   x.nnmod(&xtmp, &x_, &mut ctx)?;

//   tmp1.mod_sqr(x_, &grp.field(), &mut ctx)?;  

// //     if (group->meth->field_decode == 0) {
// //         /* field_{sqr,mul} work on standard representation */
// //         if (!group->meth->field_sqr(group, tmp2, x_, ctx))
// //             goto err;
// //         if (!group->meth->field_mul(group, tmp1, tmp2, x_, ctx))
// //             goto err;
// //     } else {
// //         if (!BN_mod_sqr(tmp2, x_, group->field, ctx))
// //             goto err;
// //         if (!BN_mod_mul(tmp1, tmp2, x_, group->field, ctx))
// //             goto err;
// //     }

// //     /* tmp1 := tmp1 + a*x */
// //     if (group->a_is_minus3) {
// //         if (!BN_mod_lshift1_quick(tmp2, x, group->field))
// //             goto err;
// //         if (!BN_mod_add_quick(tmp2, tmp2, x, group->field))
// //             goto err;
// //         if (!BN_mod_sub_quick(tmp1, tmp1, tmp2, group->field))
// //             goto err;
// //     } else {
// //         if (group->meth->field_decode) {
// //             if (!group->meth->field_decode(group, tmp2, group->a, ctx))
// //                 goto err;
// //             if (!BN_mod_mul(tmp2, tmp2, x, group->field, ctx))
// //                 goto err;
// //         } else {
// //             /* field_mul works on standard representation */
// //             if (!group->meth->field_mul(group, tmp2, group->a, x, ctx))
// //                 goto err;
// //         }

// //         if (!BN_mod_add_quick(tmp1, tmp1, tmp2, group->field))
// //             goto err;
// //     }

// //     /* tmp1 := tmp1 + b */
// //     if (group->meth->field_decode) {
// //         if (!group->meth->field_decode(group, tmp2, group->b, ctx))
// //             goto err;
// //         if (!BN_mod_add_quick(tmp1, tmp1, tmp2, group->field))
// //             goto err;
// //     } else {
// //         if (!BN_mod_add_quick(tmp1, tmp1, group->b, group->field))
// //             goto err;
// //     }

// //     if (!BN_mod_sqrt(y, tmp1, group->field, ctx)) {
// // #ifndef FIPS_MODE
// //         unsigned long err = ERR_peek_last_error();

// //         if (ERR_GET_LIB(err) == ERR_LIB_BN
// //             && ERR_GET_REASON(err) == BN_R_NOT_A_SQUARE) {
// //             ERR_clear_error();
// //             ECerr(EC_F_EC_GFP_SIMPLE_SET_COMPRESSED_COORDINATES,
// //                   EC_R_INVALID_COMPRESSED_POINT);
// //         } else
// // #endif
// //         {
// //             ECerr(EC_F_EC_GFP_SIMPLE_SET_COMPRESSED_COORDINATES,
// //                   ERR_R_BN_LIB);
// //         }
// //         goto err;
// //     }

// //     if (y_bit != BN_is_odd(y)) {
// //         if (BN_is_zero(y)) {
// //             int kron;

// //             kron = BN_kronecker(x, group->field, ctx);
// //             if (kron == -2)
// //                 goto err;

// //             if (kron == 1)
// //                 ECerr(EC_F_EC_GFP_SIMPLE_SET_COMPRESSED_COORDINATES,
// //                       EC_R_INVALID_COMPRESSION_BIT);
// //             else
// //                 /*
// //                  * BN_mod_sqrt() should have caught this error (not a square)
// //                  */
// //                 ECerr(EC_F_EC_GFP_SIMPLE_SET_COMPRESSED_COORDINATES,
// //                       EC_R_INVALID_COMPRESSED_POINT);
// //             goto err;
// //         }
// //         if (!BN_usub(y, group->field, y))
// //             goto err;
// //     }
// //     if (y_bit != BN_is_odd(y)) {
// //         ECerr(EC_F_EC_GFP_SIMPLE_SET_COMPRESSED_COORDINATES,
// //               ERR_R_INTERNAL_ERROR);
// //         goto err;
// //     }

// //     if (!EC_POINT_set_affine_coordinates(group, point, x, y, ctx))
// //         goto err;

// //     ret = 1;

// //  err:
// //     BN_CTX_end(ctx);
// //     BN_CTX_free(new_ctx);
// //     return ret;
//   Ok(())
// }

// fn get_pubkey(sig: &[u8], msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {

//   let recid = (sig[0] as u32 - 27) & 3;
//   let sig = EcdsaSig::from_der(&sig)?;
//   let sig_r = sig.r();
//   let sig_s = sig.s();  

//   let grp = EcGroup::from_curve_name(Nid::SECP256K1)?;
//   let mut ctx = BigNumContext::new()?;

//   let mut order = BigNum::new()?;
//   grp.order(&mut order, &mut ctx)?;
//   let mut x = order;

//   x.mul_word(recid/2)?;
//   let mut x2 = BigNum::new()?;
//   x2.checked_add(&mut x, &sig_r)?;

//   //let mut p = BigNum::new()?;
//   let mut a = BigNum::new()?;
//   let mut b = BigNum::new()?;
//   let mut field = BigNum::new()?;
//   grp.components_gfp(&mut field, &mut a, &mut b, &mut ctx)?;

//   if x2 == field { return /*0*/ Ok(Vec::new()); }

//   let mut R = EcPoint::new(&grp);

//   //if (!EC_POINT_set_compressed_coordinates_GFp(group, R, x, recid % 2, ctx)) { ret=0; goto err; }

//   Ok(Vec::new())
// }
