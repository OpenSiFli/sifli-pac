# SVD Files

These SVD files are from [SiFli SDK 2.3.1](https://github.com/OpenSiFli/SiFli-SDK/tree/v2.3.1/tools/svd_external).The following modifications were made on `SF32LB52x.svd` to ensure proper loading by chiptool and form:

Invaild UTF-8 charactors when chiptool loaded:
Replaced `…` with `...`, Replaced `‰` with `per mille`, 
These characters will become invaild characters in lib.rs after chiptool loaded it.
