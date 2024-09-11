// add-zilliqa-chain.js

document.addEventListener('DOMContentLoaded', function() {
    const addZilliqaChainButton = document.getElementById('addZilliqaChainButton');
    
    if (addZilliqaChainButton) {
        addZilliqaChainButton.addEventListener('click', addZilliqaChain);
    }
});

async function addZilliqaChain() {
    if (typeof window.ethereum !== 'undefined') {
        try {
            await window.ethereum.request({
                method: "wallet_addEthereumChain",
                params: [
                    {
                        blockExplorerUrls: [
                            "https://otterscan.zq2-prototestnet.zilliqa.com/"
                        ],
                        iconUrls: [
                            "https://www.zilliqa.com/images/icon-zilliqa-testnet.svg",
                            "https://www.zilliqa.com/images/icon-zilliqa-testnet.png"
                        ],
                        nativeCurrency: {
                            name: "ZIL",
                            symbol: "ZIL",
                            decimals: 18
                        },
                        rpcUrls: [
                            "https://api.zq2-prototestnet.zilliqa.com"
                        ],
                        chainId: "0x814f",
                        chainName: "Zilliqa 2 EVM proto-testnet"
                    }
                ],
            });
            alert('Zilliqa 2 EVM proto-testnet has been added to your wallet!');
        } catch (error) {
            console.error(error);
            alert('An error occurred while trying to add the network: ' + error.message);
        }
    } else {
        alert('MetaMask is not installed. Please install it to use this feature.');
    }
}