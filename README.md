# TASK 2: Deploy Your First Smart Contract

Welcome to the task 2 of Concordium hackaton. This repository contains the second task of the hackaton which consists on creating and deploying the first contract.

I have created a contract which has one variable as status named `changed` of boolean type initialized as `false` when the contract is created. Once the variable is changed to `true` it can't be change again, so if you try it will raise an error.

## Compiling the contract

Once the contract is generated, I have compiled the project by executing the command `cargo concordium build -e --out ./my_concordium_project .wasm.vi`. See image below:

![Compiling the contract](img/compiling_project.png "Compiling the contract")

