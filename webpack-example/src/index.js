const DEV_IMPORT = '../../sm-crypto/pkg/sm_crypto'

async function main() {
    const libsm = await require('@tdos/sm-crypto/sm_crypto')

    // ======= 国密 ========

    // generate private key 
    let privateKey = libsm.sm3('0x00')
    console.log(`privateKey = ${privateKey}`)
    let publicKey = libsm.sm2_pk_from_sk(privateKey, true)
    console.log(`publicKey = ${publicKey}`)

    // sign
    const msg = '0xffff'
    const seed = 128n
    let sig = libsm.sm2_sign(seed, privateKey, msg)
    console.log(`sig = ${sig}`)

    // verify
    const verified = libsm.sm2_verify(seed + 2n, msg, publicKey, sig)
    console.log(`verify result = ${verified}`)

    // ======= 环签 ========

    // 生成环签私钥
    privateKey = libsm.mlsag_generate_signer(BigInt(111))
    publicKey = libsm.mlsag_pk_from_sk(privateKey)

    // 生成5个用于环签的公钥 混淆
    const decoys = libsm.mlsag_generate_decoys(BigInt(112), 5)
    console.log(decoys)

    // 生成签名
    sig = libsm.mlsag_sign(BigInt(113), privateKey, decoys, '0xffffccee')
    console.log(sig)

    // 验证签名时需要加入自己的公钥
    decoys.push(publicKey)
    console.log(libsm.mlsag_verify(BigInt(114), '0xffffccee', decoys, sig))


    //     

}


main()

