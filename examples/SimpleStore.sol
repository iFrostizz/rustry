pragma solidity 0.8.20;

contract SimpleStore {
    uint256 number;

    function set(uint256 _number) external {
        number = _number;
    }

    function get() external view returns (uint256) {
        return number;
    }
}
