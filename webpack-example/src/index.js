
async function main() {
    const libsm = await require('@tdos/sm-crypto/sm_crypto')
    // generate private key 
    const privateKey = libsm.sm3('0x00')
    console.log(`privateKey = ${privateKey}`)
    const publicKey = libsm.sm2_pk_from_sk(privateKey, true)
    console.log(`publicKey = ${publicKey}`)

    // sign
    const msg = '0xffff'
    const seed = 128n
    const sig = libsm.sm2_sign(seed, privateKey, msg)
    console.log(`sig = ${sig}`)

    // verify
    const verified = libsm.sm2_verify(seed + 2n, msg, publicKey, sig)
    console.log(`verify result = ${verified}`)
}


main()

