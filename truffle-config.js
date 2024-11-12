// Import environment configuration if using environment variables
require('dotenv').config();
const HDWalletProvider = require('@truffle/hdwallet-provider');

// Define environment variables for Infura and Mnemonic
const { MNEMONIC, PROJECT_ID } = process.env;

module.exports = {
  networks: {
    // Local Ganache configuration
    development: {
      host: "127.0.0.1",     // Ganache address
      port: 8545,            // Ganache port
      network_id: "*",       // Any network ID
    },
    // Configuration for Infura's Goerli network
    goerli: {
      provider: () => new HDWalletProvider(MNEMONIC, `https://goerli.infura.io/v3/${PROJECT_ID}`),
      network_id: 5,          // Goerli network ID
      confirmations: 2,       // Confirm 2 blocks before deployment
      timeoutBlocks: 200,     // Wait 200 blocks before the deployment times out
      skipDryRun: true        // Skip dry run before migration
    },
    // Advanced configuration for a private network (optional)
    private: {
      provider: () => new HDWalletProvider(MNEMONIC, `https://network.io`),
      network_id: 2111,       // Custom network ID for a private network
      production: true        // Treat this network as a public one
    }
  },

  mocha: {
    // Configure test timeout here
    // timeout: 100000
  },

  // Solidity compiler configuration
  compilers: {
    solc: {
      version: "0.8.21",      // Specific Solidity version for this project
      settings: {             // Optimization settings
        optimizer: {
          enabled: true,      // Enable optimization
          runs: 200
        },
        evmVersion: "istanbul" // Specify EVM version (e.g., istanbul)
      }
    }
  }
};
