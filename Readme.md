Blockchain Voting App

Overview
This project is a decentralized voting application built using blockchain technology. The application has a backend developed with Rust using Actix Web, and a frontend built with React and TypeScript. The backend stores voting data and interacts with a Substrate-based blockchain for recordkeeping, while the frontend allows users to cast their votes and view results.

Features
Blockchain-based voting for transparency and security
Docker support for easy deployment
RESTful API for voting and results retrieval
Frontend with React for voting interaction
Prerequisites
To get started, ensure you have the following installed:

Docker and Docker Compose
Node.js (for frontend)
Rust (for backend)
Truffle (for blockchain development)

Setup Instructions
1. Clone the Repository
git clone https://github.com/yourusername/blockchainElection.git
cd blockchainElection

2. Setting up the Backend (Rust)
Backend (Rust)
2.1 Run Backend (Rust) Without Docker:

To run the backend without Docker (locally):
cd voting_backend
cargo run

2.2 Dockerize the Backend:

You can run the backend using Docker.
docker build -t voting_backend .
docker run -p 8080:8080 voting_backend

4. Setting up the Blockchain (Substrate)
To interact with the blockchain, you can use Ganache or any Substrate node locally. For Ganache:

Install Ganache CLI:
npm install -g ganache-cli

Start the Blockchain:

Run Ganache CLI:
ganache-cli
This will start a local blockchain instance on http://localhost:8545.

Deploy Contracts Using Truffle:

In the voting_backend folder, deploy the smart contracts:
truffle migrate --network development


5. Running the Full Application
Start the Backend:

Either run the backend locally using cargo run or build and run the Docker container as described above.

Start the Frontend:

Either run the frontend locally using npm start or build and run the Docker container.

Interact with the Application:

Open the React app in your browser: http://localhost:3000
You should be able to select a candidate and cast your vote.
View Results:

To see the results, you can use the "Results" section of the frontend or make a GET request to http://localhost:8080/results to view all votes.

6. Database Configuration (Optional)
If you're using a database (e.g., MongoDB), ensure that you have the database running locally or in a container. Update your backend configuration in config.rs to connect to the database.

Troubleshooting
Ports Conflict: If you see a "port already in use" error, ensure no other service is running on port 8080 or 3000 and stop conflicting services.
Missing Modules: If you encounter missing module errors (e.g., dotenv or axios), install them using npm install or cargo install as needed.
CORS Issues: If the frontend cannot access the backend API, make sure CORS is enabled properly on the backend.