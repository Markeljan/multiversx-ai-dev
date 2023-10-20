# multiversx-ai-dev ChatGPT Plugin

Compile and Deploy Rust smart contracts to MultiversX Devnet directly from Chat GPT.

The Multiversx Developer Plugin is crafted to cater to developers working with Multiversx smart contracts. Previously known as Elrond, Multiversx has made a significant impact in the blockchain sphere. To further enhance the developer experience, this plugin is built using the Rust programming language and leverages the robust capabilities of the mypx CLI tool for seamless compilation and deployment operations. Chat GPT Plugin written entirely in Rust 😎 (first time ruster).

[Youtube Demo](https://www.youtube.com/watch?v=IX8RGU0Qpss) | 
[Google Slides](https://docs.google.com/presentation/d/e/2PACX-1vRuhj8WIume_RU2DsLX5-BDJAkHTTuZAcahAGa3tyLjflp6klE2fB4jNPwe277DI1qG0E5oiEsmvPMN/pub?start=false&loop=false&delayms=3000)

<img width="696" alt="image" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/80c617df-ef70-425f-87b5-49757b68ab1a">

## Compile

<img width="843" alt="Screenshot 2023-10-20 at 5 06 52 AM" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/9a3ad832-d331-468a-b2c9-ff2b327bb99c">

Endpoint: /compile
Method: POST
Description: Allows users to compile Rust smart contracts specifically designed for Multiversx. The source code needs to be provided as a string in the request.
Request Payload: JSON format with the source field containing the Rust source code of the smart contract.
Response: Returns a string message indicating the success of the compilation or provides an error message in case of a failure.

## Deploy

![smallergif](https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/e66df1cc-6780-43d8-a22e-1bc81eca0669)

Endpoint: /deploy
Method: GET
Description: Enables users to deploy the most recently compiled smart contract to the Multiversx network.
Response: Outputs a string message confirming the successful deployment of the contract or an error message if the deployment process encounters an issue.

## Getting Started:
create /wallet folder in root dir
geenrate wallet-owner.pem file

clone repo, cd to chatgpt-plugin
```
cargo run
```
install custom ChatGPT plugin to localhost:8000 (requires special access)

<img width="802" alt="Screenshot 2023-10-20 at 4 13 08 AM" src="https://github.com/Markeljan/multiversx-ai-dev/assets/12901349/2cfa88c7-05c4-4136-9c51-1f78ffee1e2d">


### Built With
Rust, Rocket, mxpy cli tool, chatGPT.  For Encode x Multiversx Hackathon


