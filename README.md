Run the following command in your terminal to install Solana CLI

sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
Bash
You have the option to substitute "stable" with the release tag that corresponds to the software version you want (such as v1.18.1), or you can opt for one of the three symbolic channel names: stable, beta, or edge.

Now run the following command to check the version of Solana.

solana --version
Bash
Output

solana-cli 1.17.25 (src:d0ed878d; feat:3580551090, client:SolanaLabs)
Bash
We have successfully installed the Solana CLI, Let's move to the next step by installing the 'Anchor' framework.

Install Anchor
To set up and maintain different versions of the Anchor framework, we'll use AVM, the Anchor Version Manager. Because AVM is installed through cargo (Rust's package manager), the installation process remains consistent across all operating systems. Once installed, AVM enables us to easily install the specific version of the Anchor framework we need.

Install AVM
Run the following command in your terminal to install avm.

cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
Bash
Install Anchor using avm
Restart your terminal and run the following command to install Anchor.

avm install latest
avm use latest
Bash
Setup a localhost blockchain cluster
The Solana CLI includes a built-in test validator, enabling you to run a complete blockchain cluster directly from your command line interface.

Now open a new tab of the terminal and run the following command to run the localhost cluster.

solana-test-validator
Bash
Now return to the first terminal tab and configure the Solana CLI with the local cluster by running the following command.

solana config set --url localhost
Bash
To view the configuration of the Solana cli run the following command.

solana config get
Bash
Create a file system wallet
In Solana, a file system wallet is like a digital wallet that lives on your computer. It's a secure place where you can keep your Solana cryptocurrency and interact with the Solana blockchain. Instead of relying on a third-party service, like an online wallet, a file system wallet stores your cryptocurrency keys directly on your device. This gives you more control and security over your funds. You can use your file system wallet to send and receive Solana tokens, participate in decentralized finance (DeFi) activities, and more, all from your computer. It's an essential tool for anyone looking to engage with the Solana ecosystem.

Let's create a file system wallet to use during the development. Run the following command in the terminal.

solana-keygen new
Bash
The solana-keygen command automatically generates a new file system wallet, which is initially saved at ~/.config/solana/id.json. You have the option to specify a different location for the output file by using the --outfile /path option.

Set your new wallet as the default
Run the following command to set the wallet as default.

solana config set -k ~/.config/solana/id.json
Bash
Airdrop SOL token in your wallet
After setting a new wallet as default, you can request a free SOL token by running the following command.

solana airdrop 2
Bash
Now check the current balance in your wallet.

solana balance
Bash

