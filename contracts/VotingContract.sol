// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VotingContract {
    struct Candidate {
        uint id;
        string name;
        uint voteCount;
    }

    struct Voter {
        bool hasVoted;
        uint candidateId;
    }

    address public owner;
    uint public candidateCount;
    bool public votingOpen;

    mapping(uint => Candidate) public candidates;
    mapping(address => Voter) public voters;

    event CandidateAdded(uint id, string name);
    event VoteCasted(address voter, uint candidateId);
    event VotingStatusChanged(bool isOpen);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can perform this action");
        _;
    }

    modifier votingIsOpen() {
        require(votingOpen == true, "Voting is currently closed");
        _;
    }

    constructor() {
        owner = msg.sender;
        votingOpen = true;
    }

    function addCandidate(string memory _name) public onlyOwner {
        candidateCount++;
        candidates[candidateCount] = Candidate(candidateCount, _name, 0);
        emit CandidateAdded(candidateCount, _name);
    }

    function castVote(uint _candidateId) public votingIsOpen {
        require(!voters[msg.sender].hasVoted, "You have already voted");
        require(_candidateId > 0 && _candidateId <= candidateCount, "Invalid candidate ID");

        voters[msg.sender] = Voter(true, _candidateId);
        candidates[_candidateId].voteCount++;
        emit VoteCasted(msg.sender, _candidateId);
    }

    function getCandidate(uint _candidateId) public view returns (string memory, uint) {
        require(_candidateId > 0 && _candidateId <= candidateCount, "Invalid candidate ID");
        Candidate memory candidate = candidates[_candidateId];
        return (candidate.name, candidate.voteCount);
    }

    function toggleVotingStatus() public onlyOwner {
        votingOpen = !votingOpen;
        emit VotingStatusChanged(votingOpen);
    }

    function getResults() public view returns (Candidate[] memory) {
        Candidate[] memory results = new Candidate[](candidateCount);
        for (uint i = 1; i <= candidateCount; i++) {
            results[i - 1] = candidates[i];
        }
        return results;
    }
}
