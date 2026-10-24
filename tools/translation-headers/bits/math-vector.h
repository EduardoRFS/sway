/* C2Rust cannot encode the aarch64_vector_pcs prototypes in glibc 2.44.
 * Sway does not use the vector math ABI. Retain scalar math declarations
 * unchanged, while omitting only these unused vector entry points. */
#include <bits/libm-simd-decl-stubs.h>
