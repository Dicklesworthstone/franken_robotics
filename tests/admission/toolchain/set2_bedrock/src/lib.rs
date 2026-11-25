pub type ProbeBedrock = (
    fs_ga::Vec3,
    fs_math::dd::Dd,
    fs_qty::Dimensionless,
    fs_rand::StreamKey,
    fs_blake3::ContentHash,
    fs_ivl::Interval,
    fs_simd::TernaryOp,
    fs_exec::AdmittedBudget,
    fs_evidence::ClaimClass,
    fs_ad::Dual64,
);
