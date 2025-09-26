use alloy::sol;

sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);
