# multiversx-ai-dev ChatGPT Plugin

Compile and Deploy Rust smart contracts to MultiversX Devnet directly from Chat GPT.

Youtube Demo: [https://www.youtube.com/watch?v=IX8RGU0Qpss](https://www.youtube.com/watch?v=IX8RGU0Qpss)

<img width="696" alt="image" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/80c617df-ef70-425f-87b5-49757b68ab1a">

<img width="1204" alt="image" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/39bd6012-ce82-41f0-b7b4-dc71dac272a2">

### Overview

The Multiversx Developer Plugin is crafted to cater to developers working with Multiversx smart contracts. Previously known as Elrond, Multiversx has made a significant impact in the blockchain sphere. To further enhance the developer experience, this plugin is built using the Rust programming language and leverages the robust capabilities of the mypx CLI tool for seamless compilation and deployment operations. Chat GPT Plugin written entirely in Rust 😎 (first time ruster).

### Features

1. Compile Rust Smart Contracts for Multiversx
Endpoint: /compile
Method: POST
Description: Allows users to compile Rust smart contracts specifically designed for Multiversx. The source code needs to be provided as a string in the request.
Request Payload: JSON format with the source field containing the Rust source code of the smart contract.
Response: Returns a string message indicating the success of the compilation or provides an error message in case of a failure.
2. Deploy Smart Contracts to Multiversx
Endpoint: /deploy
Method: GET
Description: Enables users to deploy the most recently compiled smart contract to the Multiversx network.
Response: Outputs a string message confirming the successful deployment of the contract or an error message if the deployment process encounters an issue.

### Built With
Language: Rust
Tools: mxpy CLI
Conclusion
The Multiversx Developer Plugin is a testament to the evolving landscape of blockchain development. Through the integration of Rust and the mypx CLI tool, we aim to provide developers with an efficient and user-friendly experience when working with Multiversx smart contracts. Join us in this journey and explore the endless possibilities that this plugin brings to the table!

## How to build it:
```
// clone repo, cd to chatgpt-plugin
cargo run

// install custom ChatGPT plugin to localhost:8000 (requires special access)

```
<img width="802" alt="Screenshot 2023-10-20 at 4 13 08 AM" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/2cfa88c7-05c4-4136-9c51-1f78ffee1e2d">

Built for Encode x Multiversx Hackathon


