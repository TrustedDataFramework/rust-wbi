interface Crypto {
    function sm3(bytes calldata x) external pure returns (bytes32);

    function sm2_pk_from_sk(bytes32 private_key, bool compress)
        external
        pure
        returns (bytes memory);

    function sm2_verify(
        uint64 seed,
        bytes calldata message,
        bytes calldata public_key,
        bytes calldata sig
    ) external pure returns (bool);
}
