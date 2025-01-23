# SVD Files

These SVD files are from [SiFli SDK 2.3.0](https://github.com/OpenSiFli/SiFli-SDK/tree/v2.3.0/tools/svd_external). The following modifications were made to ensure proper loading by chiptool:

1. Invalid `resetValue`:
    Replaced `<resetValue>0x</resetValue>` with `<resetValue>0x0</resetValue>`.

2. `&` symbol in descriptions:
    Replaced `  &  ` with `  and  ` (Include spaces on both sides).
