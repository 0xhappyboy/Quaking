use alloy::sol;

sol! {
    #[sol(rpc)]
    contract IERC20 {
         function totalSupply() external view returns (uint256);
         function balanceOf(address account) external view returns (uint256);
         function transfer(address to, uint256 amount) external returns (bool);
         function allowance(address owner, address spender) external view returns (uint256);
         function approve(address spender, uint256 amount) external returns (bool);
         function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
    #[sol(rpc)]
    contract IERC20METADATA  {
         function name() external view returns (string memory);
         function symbol() external view returns (string memory);
         function decimals() external view returns (uint8);
    }
}
